mod riscv32;
mod x86;
pub use riscv32::Riscv32;
pub use riscv32::GUEST_ISA;

use crate::common::{Vaddr, Word};

pub trait ISA {
    /// set pc to `next``
    fn set_pc(&mut self, next: Vaddr);
    /// get current PC
    fn pc(&self) -> Vaddr;
    /// get register[idx]
    fn reg(&self, idx: usize) -> Word;
    /// set register[idx]
    fn set_reg(&self, idx: usize, val: Word);
}
