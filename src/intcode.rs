use std::convert::TryInto;
struct Computer {
    pc: usize,
    memory: Vec<i32>,
}

impl Computer {
    fn load(text: &str) -> Computer {
        let mut memory: Vec<i32> = Vec::new();
        memory.extend(text.split(",").map(|i| i.parse::<i32>().unwrap()));
        Computer {
            pc: 0,
            memory: memory,
        }
    }

    fn run(&mut self) {
        if self.pc + 3 > self.memory.len() - 1 {
            return {};
        }
        let opcode = self.memory[self.pc];
        let x: usize = self.memory[self.pc + 1].try_into().unwrap();
        let y: usize = self.memory[self.pc + 2].try_into().unwrap();
        let output: usize = self.memory[self.pc + 3].try_into().unwrap();

        match opcode {
            1 => {
                self.memory[output] = self.memory[x] + self.memory[y];
                self.pc += 4;
                self.run();
            }
            2 => {
                self.memory[output] = self.memory[x] * self.memory[y];
                self.pc += 4;
                self.run()
            }
            99 => println!("done"),
            _ => println!("WTF"),
        }
    }

    fn print_memory(&self) -> String {
        let memory: Vec<String> = self
            .memory
            .clone()
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        memory.join(",")
    }
}

pub fn hello() {
    let mut computer = Computer::load("1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,5,19,23,1,6,23,27,1,27,10,31,1,31,5,35,2,10,35,39,1,9,39,43,1,43,5,47,1,47,6,51,2,51,6,55,1,13,55,59,2,6,59,63,1,63,5,67,2,10,67,71,1,9,71,75,1,75,13,79,1,10,79,83,2,83,13,87,1,87,6,91,1,5,91,95,2,95,9,99,1,5,99,103,1,103,6,107,2,107,13,111,1,111,10,115,2,10,115,119,1,9,119,123,1,123,9,127,1,13,127,131,2,10,131,135,1,135,5,139,1,2,139,143,1,143,5,0,99,2,0,14,0");
    computer.run();
    println!("{:?}", computer.memory);
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
        ];
        for [input, output] in t.into_iter() {
            let mut computer = Computer::load(input);
            computer.run();
            assert_eq!(computer.print_memory(), output);
        }
    }
}
