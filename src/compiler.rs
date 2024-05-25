use std::str::FromStr;

use crate::instruction::{Instruction, InstructionType};

pub struct Compiler {
    code: Vec<char>,
    position: u64,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new(code: String) -> Self {
        let trimmed_code = code.chars().filter(|c| !c.is_whitespace()).collect();
        Self {
            code: trimmed_code,
            position: 0,
            instructions: vec![],
        }
    }

    pub fn compile(&mut self) -> Vec<Instruction> {
        let mut loop_stack = vec![];
        while self.position < self.code.len() as u64 {
            //println!("{:?}", self.instructions);
            let current_char =
                InstructionType::from_str(self.code[self.position as usize].to_string().as_str())
                    .unwrap();
            match current_char {
                InstructionType::Right
                | InstructionType::Left
                | InstructionType::Plus
                | InstructionType::Minus
                | InstructionType::PutChar
                | InstructionType::ReadChar => {
                    self.compile_foldable_instruction(current_char);
                    self.position += 1;
                }
                InstructionType::JumpIfZero => {
                    self.set_instruction(InstructionType::JumpIfZero, 0);
                    let ins_pos = self.get_instruction_position();
                    loop_stack.push(ins_pos);
                    self.position += 1;
                }
                InstructionType::JumpIfNotZero => {
                    let ins_pos = loop_stack.pop().unwrap();

                    self.set_instruction(InstructionType::JumpIfNotZero, ins_pos as u8);
                    self.instructions[ins_pos as usize].argument =
                        self.get_instruction_position() as u8;
                    self.position += 1;
                }
            }
        }

        self.instructions.clone()
    }

    fn compile_foldable_instruction(&mut self, ins_type: InstructionType) {
        let mut ctn = 1;
        while (self.position < (self.code.len() - 1) as u64)
            && self.code[(self.position + 1) as usize]
                == ins_type.to_string().chars().next().unwrap()
        {
            ctn += 1;
            self.position += 1;
        }

        self.set_instruction(ins_type, ctn);
    }

    fn set_instruction(&mut self, ins_type: InstructionType, argument: u8) {
        self.instructions.push(Instruction { ins_type, argument });
    }

    fn get_instruction_position(&self) -> u64 {
        self.instructions.len() as u64 - 1
    }
}
