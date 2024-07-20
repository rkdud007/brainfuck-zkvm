use std::{
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

use crate::{instruction::Instruction, register::Register};

pub struct Machine {
    code: Vec<Instruction>,
    memory: [u8; 30000],

    // one byte buffer slice
    buf: [u8; 1],
    // register
    register: Register,
}

impl Machine {
    pub fn new(code: Vec<char>) -> Machine {
        let code: Vec<Instruction> = code
            .iter()
            .map(|c| Instruction::from_str(c.to_string().as_str()).unwrap())
            .collect();

        Machine {
            code: code.clone(),
            memory: [0; 30000],
            buf: [0],
            register: Register::new(code[0], code[1]),
        }
    }

    pub fn execute(&mut self) {
        println!("{:?}", self.code);
        while self.register.ip < self.code.len() as u64 {
            let instruction = &self.code[self.register.ip as usize];

            self.register.ci = *instruction;

            match instruction {
                Instruction::IncrementDp => self.register.mp += 1,
                Instruction::DecrementDp => self.register.mp -= 1,
                Instruction::IncrementVal => self.memory[self.register.mp as usize] += 1,
                Instruction::DecrementVal => self.memory[self.register.mp as usize] -= 1,
                Instruction::Input => self.read_char(),
                Instruction::Output => self.write_char(),
                Instruction::JumpNext => {
                    if self.memory[self.register.mp as usize] == 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            self.register.ip += 1;
                            let instruction = &self.code[self.register.ip as usize];
                            if instruction == &Instruction::JumpBack {
                                depth -= 1;
                            } else if instruction == &Instruction::JumpNext {
                                depth += 1;
                            }
                        }
                    }
                }
                Instruction::JumpBack => {
                    if self.memory[self.register.mp as usize] != 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            self.register.ip -= 1;
                            let instruction = &self.code[self.register.ip as usize];
                            if instruction == &Instruction::JumpBack {
                                depth += 1;
                            } else if instruction == &Instruction::JumpNext {
                                depth -= 1;
                            }
                        }
                    }
                }
            }

            let next_instruction = &self.code[self.register.ip as usize];
            self.register.ni = *next_instruction;

            println!("{:?}", self.register);

            self.register.ip += 1;
            self.register.clk += 1;
        }

        println!("{:?}", self.register);
    }

    fn read_char(&mut self) {
        let mut stdin = stdin();
        let input_char: usize = stdin.read(&mut self.buf).unwrap();
        self.memory[self.register.mp as usize] = input_char as u8;
    }

    fn write_char(&mut self) {
        let mut stdout = stdout();
        let _ = stdout
            .write(&[self.memory[self.register.mp as usize]])
            .unwrap();
    }
}
