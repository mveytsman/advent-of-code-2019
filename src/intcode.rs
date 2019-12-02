use std::convert::TryInto;
pub struct Computer {
    pc: usize,
    pub memory: Vec<i32>,
}

impl Computer {
    pub fn load(text: &str) -> Computer {
        let mut memory: Vec<i32> = Vec::new();
        memory.extend(text.split(",").map(|i| i.parse::<i32>().unwrap()));
        Computer {
            pc: 0,
            memory: memory,
        }
    }

    pub fn run(&mut self) {
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
            99 => {
                // done
                ()
            }
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
