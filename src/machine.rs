use std::io::{Read, Stdin, Stdout, Write};

use crate::instruction::{Instruction, InstructionType};

pub struct Machine {
    code: Vec<Instruction>,
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
    pub fn new(code: Vec<Instruction>, input: Stdin, output: Stdout) -> Machine {
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
            let ins = &self.code[self.ip as usize];
            // println!(
            //     "{}, {}, {}",
            //     ins_char, self.dp, self.memory[self.dp as usize]
            // );

            match ins.ins_type {
                InstructionType::Right => self.dp += ins.argument as u64,
                InstructionType::Left => self.dp -= ins.argument as u64,
                InstructionType::Plus => self.memory[self.dp as usize] += ins.argument,
                InstructionType::Minus => self.memory[self.dp as usize] -= ins.argument,
                InstructionType::ReadChar => {
                    for _ in 0..ins.argument {
                        self.read_char()
                    }
                }
                InstructionType::PutChar => {
                    for _ in 0..ins.argument {
                        self.write_char()
                    }
                }
                InstructionType::JumpIfZero => {
                    if self.memory[self.dp as usize] == 0 {
                        self.ip = ins.argument as u64;
                    }
                }
                InstructionType::JumpIfNotZero => {
                    if self.memory[self.dp as usize] != 0 {
                        self.ip = ins.argument as u64;
                    }
                }
            }
            self.ip += 1
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
