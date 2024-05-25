use std::{
    io::{Read, Stdin, Stdout, Write},
    str::FromStr,
};

use crate::instruction::Instruction;

pub struct Machine {
    code: Vec<char>,
    // instruction pointer
    ip: u64,
    memory: Vec<u8>,
    // data pointer
    dp: u64,
    // input stream
    input: Stdin,
    // output stream
    output: Stdout,
    // one byte buffer slice
    buf: [u8; 1],
}

impl Machine {
    pub fn new(code: Vec<char>, input: Stdin, output: Stdout) -> Machine {
        Machine {
            code,
            ip: 0,
            memory: vec![0; 30000],
            dp: 0,
            input,
            output,
            buf: [0],
        }
    }

    pub fn execute(self: &mut Machine) {
        while self.ip < self.code.len() as u64 {
            let ins_char = self.code[self.ip as usize];
            // println!(
            //     "{}, {}, {}",
            //     ins_char, self.dp, self.memory[self.dp as usize]
            // );
            let instruction = Instruction::from_str(ins_char.to_string().as_str());
            if instruction.is_ok() {
                match instruction.unwrap() {
                    Instruction::IncrementDp => self.dp += 1,
                    Instruction::DecrementDp => self.dp -= 1,
                    Instruction::IncrementVal => self.memory[self.dp as usize] += 1,
                    Instruction::DecrementVal => self.memory[self.dp as usize] -= 1,
                    Instruction::Input => self.read_char(),
                    Instruction::Output => self.write_char(),
                    Instruction::JumpNext => {
                        if self.memory[self.dp as usize] == 0 {
                            let mut depth = 1;
                            while depth != 0 {
                                self.ip += 1;
                                let instruction = Instruction::from_str(
                                    self.code[self.ip as usize].to_string().as_str(),
                                )
                                .unwrap();
                                if instruction == Instruction::JumpBack {
                                    depth -= 1;
                                } else if instruction == Instruction::JumpNext {
                                    depth += 1;
                                }
                            }
                        }
                    }
                    Instruction::JumpBack => {
                        if self.memory[self.dp as usize] != 0 {
                            let mut depth = 1;
                            while depth != 0 {
                                self.ip -= 1;
                                // println!("depth {}, ip:{}", depth, self.ip);
                                let instruction = Instruction::from_str(
                                    self.code[self.ip as usize].to_string().as_str(),
                                )
                                .unwrap();
                                if instruction == Instruction::JumpBack {
                                    depth += 1;
                                } else if instruction == Instruction::JumpNext {
                                    depth -= 1;
                                }
                            }
                        }
                    }
                }
                self.ip += 1
            }
        }
    }

    fn read_char(&mut self) {
        let input_char: usize = self.input.read(&mut self.buf).unwrap();
        self.memory[self.dp as usize] = input_char as u8;
    }

    fn write_char(&mut self) {
        let _ = self.output.write(&[self.memory[self.dp as usize]]).unwrap();
    }
}
