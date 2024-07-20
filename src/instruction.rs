use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    // '>': Increment the data pointer (to point to the next cell to the right).
    IncrementDp,
    // '<': Decrement the data pointer (to point to the next cell to the left).
    DecrementDp,
    // '+': Increment (increase by one) the byte at the data pointer.
    IncrementVal,
    // '-': Decrement (decrease by one) the byte at the data pointer.
    DecrementVal,
    // '.': Output the byte at the data pointer.
    Output,
    // ',': Accept one byte of input, storing its value in the byte at the data pointer.
    Input,
    // '[': If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ']' command.
    JumpNext,
    // ']': If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching '[' command.
    JumpBack,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Instruction::IncrementDp),
            "<" => Ok(Instruction::DecrementDp),
            "+" => Ok(Instruction::IncrementVal),
            "-" => Ok(Instruction::DecrementVal),
            "." => Ok(Instruction::Output),
            "," => Ok(Instruction::Input),
            "[" => Ok(Instruction::JumpNext),
            "]" => Ok(Instruction::JumpBack),
            _ => Err(()),
        }
    }
}
