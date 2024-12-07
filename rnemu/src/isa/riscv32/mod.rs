use crate::common::{Vaddr, Word};

use super::ISA;

mod executer;
pub use executer::Executer;
pub const GUEST_ISA: &'static str = "riscv32";

#[derive(Default, Debug)]
pub struct Riscv32 {
    gpr: [Word; 32],
    pc: Vaddr,
}

impl Riscv32 {
    pub fn new(pc: Vaddr) -> Self {
        Self { gpr: [0; 32], pc }
    }
}

impl ISA for Riscv32 {
    type Executer = executer::Executer;
    fn set_pc(&mut self, next: Vaddr) {
        self.pc = next;
    }

    fn pc(&self) -> Vaddr {
        self.pc
    }

    fn reg(&self, idx: usize) -> Word {
        self.gpr[idx]
    }

    fn set_reg(&mut self, idx: usize, val: Word) {
        self.gpr[idx] = val;
    }

    fn default_img() -> &'static [u8] {
        let ptr = IMG.as_ptr() as *const u8;
        let len = IMG.len() * std::mem::size_of::<u32>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn executer() -> Self::Executer {
        executer::Executer::new()
    }
}

pub const ISA_LOGO: &'static str = 
r"       _                         __  __                         _ 
      (_)                       |  \/  |                       | |
  _ __ _ ___  ___ ________   __ | \  / | __ _ _ __  _   _  __ _| |
 | '__| / __|/ __|______\ \ / / | |\/| |/ _` | '_ \| | | |/ _` | |
 | |  | \__ \ (__        \ V /  | |  | | (_| | | | | |_| | (_| | |
 |_|  |_|___/\___|        \_/   |_|  |_|\__,_|_| |_|\__,_|\__,_|_|
";

const IMG: [u32; 5] = [
    0x00000297, // auipc t0,0
    0x00028823, // sb  zero,16(t0)
    0x0102c503, // lbu a0,16(t0)
    0x00100073, // ebreak (used as nemu_trap)
    0xdeadbeef, // some data
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        print!("{ISA_LOGO}");
    }
}
