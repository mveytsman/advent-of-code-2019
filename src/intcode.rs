use permutohedron::Heap;
use std::collections::HashMap;

pub type Word = i64;

pub struct Computer {
    input: Option<Word>,
    pc: Word,
    pub memory: Memory,
    pub outputs: Vec<Word>,
    halted: bool,
    relative_base: Word,
}

#[derive(Debug)]
pub struct Memory {
    mem: HashMap<Word, Word>,
}

impl Memory {
    pub fn load(text: &str) -> Memory {
        let mut mem = HashMap::new();
        for (i, word) in text.split(",").enumerate() {
            let word = word.parse::<Word>().unwrap();
            mem.insert(i as Word, word);
        }
        Self { mem: mem }
    }

    pub fn read(&mut self, ptr: Word) -> Word {
        if ptr < 0 {
            panic!("out of bounds");
        }
        match self.mem.get(&ptr) {
            Some(val) => *val,
            None => {
                self.mem.insert(ptr, 0);
                0
            }
        }
    }

    pub fn write(&mut self, ptr: Word, value: Word) {
        if ptr < 0 {
            panic!("out of bounds");
        }
        self.mem.insert(ptr, value);
    }

    fn print(&self) -> String {
        let mut keys: Vec<&Word> = self.mem.keys().collect();
        keys.sort();

        keys.iter()
            .map(|i| self.mem[*i].to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn new(num: Word) -> Self {
        match num {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("unknown mode"),
        }
    }
}
#[derive(Debug)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JIT(Mode, Mode),
    JIF(Mode, Mode),
    LT(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    ARB(Mode),
    Halt,
}
impl Opcode {
    fn new(opcode: Word) -> Self {
        let op = opcode % 100;
        let mode1 = Mode::new((opcode / 100) % 10);
        let mode2 = Mode::new((opcode / 1000) % 10);
        let mode3 = Mode::new((opcode / 10000) % 10);
        match op {
            1 => Opcode::Add(mode1, mode2, mode3),
            2 => Opcode::Mult(mode1, mode2, mode3),
            3 => Opcode::Input(mode1),
            4 => Opcode::Output(mode1),
            5 => Opcode::JIT(mode1, mode2),
            6 => Opcode::JIF(mode1, mode2),
            7 => Opcode::LT(mode1, mode2, mode3),
            8 => Opcode::Eq(mode1, mode2, mode3),
            9 => Opcode::ARB(mode1), //adjust relative base
            99 => Opcode::Halt,
            x => panic!("unknown opcode: {}", x),
        }
    }
}

impl Computer {
    pub fn load(text: &str) -> Computer {
        let mut memory = Memory::load(text);
        Computer {
            input: None,
            pc: 0,
            memory: memory,
            outputs: vec![],
            halted: false,
            relative_base: 0,
        }
    }

    pub fn input(&mut self, input: Word) {
        match self.input {
            None => self.input = Some(input),
            Some(_) => panic!("already have input!"),
        }
    }

    pub fn run_with_input(&mut self, input: Word) {
        self.input(input);
        self.run();
    }

    pub fn run(&mut self) {
	let mut r = self.step();
	while r {
	    r = self.step();
	}
    }

    pub fn halted(&self) -> bool {
	self.halted
    }

    fn step(&mut self) -> bool{
        let opcode = Opcode::new(self.read_and_advance());
        match opcode {
            Opcode::Add(mode1, mode2, mode3) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance();

                let result = inputs[0] + inputs[1];
                self.write(output_addr, result, mode3);
            }

            Opcode::Mult(mode1, mode2, mode3) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance();

                let result = inputs[0] * inputs[1];
                self.write(output_addr, result, mode3);
            }

            Opcode::Input(mode1) => {
                match self.input {
                    Some(input) => {
                        let output_addr = self.read_and_advance();
                        self.write(output_addr, input, mode1);
                        self.input = None;
                    }
                    None => {
                        // move pc back and wait for more input
                        self.pc -= 1;
			return false;
                    }
                }
            }

            Opcode::Output(mode1) => {
                let inputs = self.get_operands(vec![mode1]);
                let result = inputs[0];
                self.outputs.push(result);
            }

            Opcode::JIT(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                if inputs[0] != 0 {
                    self.pc = inputs[1];
                }
            }

            Opcode::JIF(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                if inputs[0] == 0 {
                    self.pc = inputs[1];
                }
            }

            Opcode::LT(mode1, mode2, mode3) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance();

                if inputs[0] < inputs[1] {
                    self.write(output_addr, 1, mode3);
                } else {
                    self.write(output_addr, 0, mode3);
                }
            }

            Opcode::Eq(mode1, mode2, mode3) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance();

                if inputs[0] == inputs[1] {
                    self.write(output_addr, 1, mode3);
                } else {
                    self.write(output_addr, 0, mode3);
                }
            }
            // adjust relative base
            Opcode::ARB(mode1) => {
                let inputs = self.get_operands(vec![mode1]);
                self.relative_base += inputs[0];
            }

            Opcode::Halt => {
                self.halted = true;
		return false;
            }
        }
	true
    }

    fn read_and_advance(&mut self) -> Word {
        let out = self.memory.read(self.pc);
        self.pc += 1;
        out
    }

    fn write(&mut self, addr: Word, value: Word, mode: Mode) {
        match mode {
            Mode::Position => self.memory.write(addr, value),
            Mode::Immediate => panic!("can't write in immediate mode"),
            Mode::Relative => self.memory.write(self.relative_base + addr, value),
        }
    }

    fn get_operands(&mut self, modes: Vec<Mode>) -> Vec<Word> {
        let mut output = vec![];
        for mode in modes {
            let value = match mode {
                Mode::Position => {
                    let pointer = self.read_and_advance();
                    self.memory.read(pointer)
                }
                Mode::Immediate => self.read_and_advance(),
                Mode::Relative => {
                    let pointer = self.relative_base + self.read_and_advance();
                    self.memory.read(pointer)
                }
            };
            output.push(value);
        }
        output
    }
}

pub fn day7(input: &str) -> Word {
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phase_settings);
    let mut max_output = 0;
    for permutation in heap {
        let mut last_output = 0;
        for i in permutation {
            let mut computer = Computer::load(input);
            computer.run_with_input(i);
            if !computer.halted {
                computer.run_with_input(last_output);
            }
            last_output = computer.outputs[0]
        }
        if last_output > max_output {
            max_output = last_output;
        }
    }
    max_output
}

fn prev_index(i: usize, max: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        max - 1
    }
}

pub fn day7_2(input: &str) -> Word {
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phase_settings);
    let mut max_output = 0;
    for permutation in heap {
        let mut computers = vec![];
        for i in permutation {
            let mut computer = Computer::load(input);
            computer.run_with_input(i);
            computers.push(computer);
        }

        let mut num_halted = 0;
        let mut i = 1;
        computers[0].run_with_input(0);
        while num_halted < computers.len() {
            i = i % computers.len();
            if computers[i].halted {
                num_halted += 1; // this may be a bug
            } else {
                let output = *computers[prev_index(i, computers.len())]
                    .outputs
                    .last()
                    .unwrap();
                computers[i].run_with_input(output);
            }

            i += 1;
        }

        let last_output = *computers[computers.len() - 1].outputs.last().unwrap();

        if last_output > max_output {
            max_output = last_output;
        }
    }
    max_output
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_inputer() {
        let t = vec![
            ["1,0,0,0,99", "2,0,0,0,99"],
            ["2,3,0,3,99", "2,3,0,6,99"],
            ["2,4,4,5,99,0", "2,4,4,5,99,9801"],
            ["1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"],
            ["1002,4,3,4,33", "1002,4,3,4,99"],
        ];
        for [input, output] in t.into_iter() {
            let mut computer = Computer::load(input);
            computer.run();
            assert_eq!(computer.memory.print(), output);
        }
    }

    #[test]
    fn test_part2() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = Computer::load(input);
        computer.run_with_input(1);
        assert_eq!(vec![999], computer.outputs);

        let mut computer = Computer::load(input);
        computer.run_with_input(8);
        assert_eq!(vec![1000], computer.outputs);

        let mut computer = Computer::load(input);
        computer.run_with_input(9);
        assert_eq!(vec![1001], computer.outputs);
    }

    #[test]
    fn test_day7() {
        assert_eq!(
            43210,
            day7("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        );
        assert_eq!(
            54321,
            day7("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        );
        assert_eq!(65210,day7("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"))
    }

    #[test]
    fn test_day7_2() {
        assert_eq!(139629729, day7_2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"));
        assert_eq!(18216, day7_2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"));
    }

    #[test]
    fn test_day9_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut computer = Computer::load(input);
        computer.run();
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            computer.outputs
        );

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut computer = Computer::load(input);
        computer.run();
        assert_eq!(vec![1219070632396864], computer.outputs);

        let input = "104,1125899906842624,99";
        let mut computer = Computer::load(input);
        computer.run();
        assert_eq!(vec![1125899906842624], computer.outputs);
    }
}
