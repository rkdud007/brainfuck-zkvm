use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Register {
    // cycle
    pub clk: u64,
    // instruction pointer
    pub ip: u64,
    // current instruction
    pub ci: Instruction,
    // next instruction
    pub ni: Instruction,
    // memory pointer
    pub mp: u64,
    //memory value
    pub mv: u8,
    // memory value inverse
    pub mvi: u8,
}

impl Register {
    pub fn new(ci: Instruction, ni: Instruction) -> Self {
        Self {
            clk: 0,
            ip: 0,
            ci,
            ni,
            mp: 0,
            mv: 0,
            mvi: 0,
        }
    }
}
