use crate::crypto::field::FieldElement;

pub struct Compiler {
    code: Vec<char>,
    instructions: Vec<FieldElement>,
}

impl Compiler {
    pub fn new(code: String) -> Self {
        let trimmed_code = code.chars().filter(|c| !c.is_whitespace()).collect();
        Self {
            code: trimmed_code,
            instructions: vec![],
        }
    }

    pub fn compile(&mut self) -> Vec<FieldElement> {
        let mut loop_stack = vec![];
        for symbol in &self.code {
            self.instructions.push(FieldElement::from(*symbol as u64));

            match *symbol {
                '[' => {
                    self.instructions.push(FieldElement::from(0));
                    loop_stack.push(self.instructions.len() - 1);
                }
                ']' => {
                    let start_pos = loop_stack.pop().unwrap();
                    let loop_end_pos = self.instructions.len() as u8 + 1;
                    self.instructions[start_pos] = FieldElement::from(loop_end_pos as u64);
                    self.instructions
                        .push(FieldElement::from((start_pos + 1) as u64));
                }
                _ => (),
            }
        }

        self.instructions.clone()
    }
}
