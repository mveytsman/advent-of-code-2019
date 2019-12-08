use permutohedron::Heap;
use std::convert::TryInto;

pub struct Computer {
    pc: usize,
    pub memory: Vec<i32>,
    outputs: Vec<i32>,
    halted: bool,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn new(num: i32) -> Self {
        match num {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("unknown mode"),
        }
    }
}
#[derive(Debug)]
enum Opcode {
    Add(Mode, Mode),
    Mult(Mode, Mode),
    Input,
    Output(Mode),
    JIT(Mode, Mode),
    JIF(Mode, Mode),
    LT(Mode, Mode),
    Eq(Mode, Mode),
    Halt,
}
impl Opcode {
    fn new(opcode: i32) -> Self {
        let op = opcode % 100;
        let mode1 = Mode::new((opcode / 100) % 10);
        let mode2 = Mode::new((opcode / 1000) % 10);
        let _mode3 = Mode::new((opcode / 10000) % 10);
        match op {
            1 => Opcode::Add(mode1, mode2),
            2 => Opcode::Mult(mode1, mode2),
            3 => Opcode::Input,
            4 => Opcode::Output(mode1),
            5 => Opcode::JIT(mode1, mode2),
            6 => Opcode::JIF(mode1, mode2),
            7 => Opcode::LT(mode1, mode2),
            8 => Opcode::Eq(mode1, mode2),
            99 => Opcode::Halt,
            x => panic!("unknown opcode: {}", x),
        }
    }
}

impl Computer {
    pub fn load(text: &str) -> Computer {
        let mut memory: Vec<i32> = Vec::new();
        memory.extend(text.split(",").map(|i| i.parse::<i32>().unwrap()));
        Computer {
            pc: 0,
            memory: memory,
            outputs: vec![],
            halted: false,
        }
    }

    pub fn run(&mut self, input: Vec<i32>) {
        let opcode = Opcode::new(self.read_and_advance());
        match opcode {
            Opcode::Add(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance() as usize;

                let result = inputs[0] + inputs[1];
                self.memory[output_addr] = result;
                self.run(input);
            }

            Opcode::Mult(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance() as usize;

                let result = inputs[0] * inputs[1];
                self.memory[output_addr] = result;
                self.run(input);
            }

            Opcode::Input => {
                if input.len() == 0 {
                    //stop and wait for more input
                    self.pc -= 1;
                    return;
                }
                let output_addr = self.read_and_advance() as usize;
                self.memory[output_addr] = input[0];
                self.run(input[1..].to_vec());
            }

            Opcode::Output(mode1) => {
                let inputs = self.get_operands(vec![mode1]);

                let result = inputs[0];
                self.outputs.push(result);
                self.run(input);
            }

            Opcode::JIT(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                if inputs[0] != 0 {
                    self.pc = inputs[1] as usize;
                }
                self.run(input);
            }

            Opcode::JIF(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                if inputs[0] == 0 {
                    self.pc = inputs[1] as usize;
                }
                self.run(input);
            }

            Opcode::LT(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance() as usize;

                if inputs[0] < inputs[1] {
                    self.memory[output_addr] = 1;
                } else {
                    self.memory[output_addr] = 0;
                }
                self.run(input);
            }

            Opcode::Eq(mode1, mode2) => {
                let inputs = self.get_operands(vec![mode1, mode2]);
                let output_addr = self.read_and_advance() as usize;

                if inputs[0] == inputs[1] {
                    self.memory[output_addr] = 1;
                } else {
                    self.memory[output_addr] = 0;
                }
                self.run(input);
            }

            Opcode::Halt => self.halted = true,
        }
    }

    fn print_memory(&self) -> String {
        self.memory
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn read_and_advance(&mut self) -> i32 {
        let out = self.memory[self.pc];
        self.pc += 1;
        out
    }

    fn get_operands(&mut self, modes: Vec<Mode>) -> Vec<i32> {
        let mut output = vec![];
        for mode in modes {
            match mode {
                Mode::Position => {
                    let pointer = self.read_and_advance() as usize;
                    output.push(self.memory[pointer]);
                }
                Mode::Immediate => {
                    output.push(self.read_and_advance());
                }
            }
        }
        output
    }
}

pub fn day7(input: &str) -> i32 {
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phase_settings);
    let mut max_output = 0;
    for permutation in heap {
        let mut last_output = 0;
        dbg!(&permutation);
        for i in permutation {
            let mut computer = Computer::load(input);
            dbg!(&last_output);
            computer.run(vec![i, last_output]);
            last_output = computer.outputs[0]
        }
        if last_output > max_output {
            max_output = last_output;
        }
    }
    max_output
}

pub fn day7_2(input: &str) -> i32 {
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phase_settings);
    let mut max_output = 0;
    for permutation in heap {
        let mut last_output = 0;
        dbg!(&permutation);
        let mut computers = vec![];
        for i in permutation {
            let mut computer = Computer::load(input);
            computer.run(vec![i, last_output]);
            last_output = computer.outputs[0];
            computers.push(computer);
        }
        // keep running till we are all done
        let num_halted = 0;
        //	while num_halted < computers.len() {
        //	    for
        //	}

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
            computer.run(vec![1]);
            assert_eq!(computer.print_memory(), output);
        }
    }

    #[test]
    fn test_part2() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = Computer::load(input);
        computer.run(vec![1]);
        assert_eq!(vec![999], computer.outputs);

        let mut computer = Computer::load(input);
        computer.run(vec![8]);
        assert_eq!(vec![1000], computer.outputs);

        let mut computer = Computer::load(input);
        computer.run(vec![9]);
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
}
