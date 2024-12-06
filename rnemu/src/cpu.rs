use crate::{common::Word, isa::ISA};

pub mod decode;

pub struct CPU {
    pub isa: ISA,
}

impl CPU {
    pub fn read_reg(&self, idx: usize) -> Word {
        self.isa.inner.gpr[idx]
    }

    pub fn set_reg(&mut self, rd: u32, val: u32) {
        self.isa.inner.gpr[rd as usize] = val;
    }
}
