mod riscv32;
mod x86;
// pub use riscv32::GUEST_ISA;
// pub use riscv32::ISA_LOGO;

use crate::common::{Vaddr, Word};

pub trait ISA {
    type Executer;
    /// set pc to `next``
    fn set_pc(&mut self, next: Vaddr);
    /// get current PC
    fn pc(&self) -> Vaddr;
    /// get register[idx]
    fn reg(&self, idx: usize) -> Word;
    /// set register[idx]
    fn set_reg(&mut self, idx: usize, val: Word);
    // get default test img
    fn default_img() -> &'static [u8];
    fn executer() -> Self::Executer;
}

struct Instruction {
    mask: u32,
    data: u32,
    name: &'static str,
    operation: fn(cpu: &mut RvCpu, word: u32, address: u64) -> Result<(), i32>,
}
