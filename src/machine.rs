use std::{
    error::Error,
    io::{Read, Stdin, Stdout, Write},
    str::FromStr,
};

use crate::{crypto::field::FieldElement, instruction::InstructionType, registers::Registers};

pub struct ProgramMemory {
    code: Vec<FieldElement>,
}

pub struct MutableState {
    ram: Vec<FieldElement>,
    registers: Registers,
}

pub struct IO {
    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

pub struct Machine {
    program: ProgramMemory,
    state: MutableState,
    io: IO,
}

impl Machine {
    pub fn new(code: Vec<FieldElement>, input: Stdin, output: Stdout) -> Machine {
        Machine {
            program: ProgramMemory { code },
            state: MutableState {
                ram: vec![FieldElement::zero(); 100],
                registers: Registers::new(),
            },
            io: IO {
                input: Box::new(input),
                output: Box::new(output),
            },
        }
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.registers.ci = self.program.code[self.state.registers.ip.to_usize()];
        self.state.registers.ni = self.program.code[self.state.registers.ip.to_usize() + 1];
        let target_ci = self.state.registers.ci;
        let ins_type =
            InstructionType::from_str(&(target_ci.to_usize() as u8 as char).to_string()).unwrap();

        println!("register: {}", self.state.registers);
        self.execute_instruction(ins_type)?;
        // let trace: Vec<Registers> = Vec::new();
        while self.state.registers.ip.to_usize() < self.program.code.len() - 1 {
            self.state.registers.clk += FieldElement::one();
            self.state.registers.ip += FieldElement::one();
            let target_ci = self.program.code[self.state.registers.ip.to_usize()];

            let ins_type =
                InstructionType::from_str(&(target_ci.to_usize() as u8 as char).to_string())
                    .unwrap();
            self.state.registers.ni = self.program.code[self.state.registers.ip.to_usize() + 1];
            println!("register: {}", self.state.registers);
            self.execute_instruction(ins_type)?;
        }

        self.state.registers.clk += FieldElement::one();
        self.state.registers.ip += FieldElement::one();
        println!("register: {}", self.state.registers);
        Ok(())
    }

    fn read_char(&mut self) -> Result<(), std::io::Error> {
        let mut buf = [0; 1];
        self.io.input.read_exact(&mut buf)?;
        let input_char = buf[0] as usize;
        self.state.ram[self.state.registers.mp.to_usize()] = FieldElement::from(input_char as u64);
        Ok(())
    }

    fn write_char(&mut self) -> Result<(), std::io::Error> {
        let char_to_write = self.state.ram[self.state.registers.mp.to_usize()].to_usize() as u8;
        self.io.output.write_all(&[char_to_write])?;
        println!("\n");
        Ok(())
    }

    fn execute_instruction(&mut self, ins: InstructionType) -> Result<(), Box<dyn Error>> {
        match ins {
            InstructionType::Right => {
                self.state.registers.mp += FieldElement::one();
            }
            InstructionType::Left => {
                self.state.registers.mp -= FieldElement::one();
            }
            InstructionType::Plus => {
                let mp = self.state.registers.mp.to_usize();
                self.state.ram[mp] += FieldElement::one();
            }
            InstructionType::Minus => {
                let mp = self.state.registers.mp.to_usize();
                self.state.ram[mp] -= FieldElement::one();
            }
            InstructionType::ReadChar => {
                self.read_char()?;
            }
            InstructionType::PutChar => {
                self.write_char()?;
            }
            InstructionType::JumpIfZero => {
                let mp = self.state.registers.mp.to_usize();
                let ip = self.state.registers.ip.to_usize();
                let argument = self.program.code[ip + 1];
                self.state.registers.ni = argument;
                if self.state.ram[mp] == FieldElement::zero() {
                    self.state.registers.ip = argument;
                    return Ok(());
                }
                self.state.registers.ip += FieldElement::one();
            }
            InstructionType::JumpIfNotZero => {
                let mp = self.state.registers.mp.to_usize();
                let ip = self.state.registers.ip.to_usize();
                let argument = self.program.code[ip + 1];
                if self.state.ram[mp] != FieldElement::zero() {
                    self.state.registers.ip = argument - FieldElement::one();
                    return Ok(());
                }
                self.state.registers.ip += FieldElement::one();
            }
        }
        self.state.registers.mv = self.state.ram[self.state.registers.mp.to_usize()];
        self.state.registers.mvi = if self.state.registers.mv == FieldElement::zero() {
            FieldElement::zero()
        } else {
            self.state.registers.mv.inverse()
        };

        Ok(())
    }
}
