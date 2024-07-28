use crate::crypto::field::FieldElement;

#[derive(Clone)]
pub struct Registers {
    /// cycle
    pub clk: FieldElement,
    /// instruction pointer
    pub ip: FieldElement,
    /// current instruction
    pub ci: FieldElement,
    /// next instruction
    pub ni: FieldElement,
    /// memory pointer
    pub mp: FieldElement,
    /// memory value
    pub mv: FieldElement,
    /// memory value inverse
    pub mvi: FieldElement,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            clk: FieldElement::zero(),
            ip: FieldElement::zero(),
            ci: FieldElement::zero(),
            ni: FieldElement::zero(),
            mp: FieldElement::zero(),
            mv: FieldElement::zero(),
            mvi: FieldElement::zero(),
        }
    }
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "clk:{}, ip: {}, ci: {}, ni: {}, mp: {}, mv: {}, mvi: {}",
            self.clk, self.ip, self.ci, self.ni, self.mp, self.mv, self.mvi
        )
    }
}
