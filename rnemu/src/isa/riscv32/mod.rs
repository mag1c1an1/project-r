use crate::common::{Vaddr, Word};

use super::ISA;

mod executer;

pub const GUEST_ISA: &'static str = "riscv32";

#[derive(Default, Debug)]
pub struct Riscv32 {
    gpr: [Word; 32],
    pc: Vaddr,
}

impl Riscv32 {
    pub fn new() -> Self {
        Self {
            gpr: [0; 32],
            pc: 0,
        }
    }
}

impl ISA for Riscv32 {
    fn set_pc(&mut self, next: Vaddr) {
        todo!()
    }

    fn pc(&self) -> Vaddr {
        todo!()
    }

    fn reg(&self, idx: usize) -> Word {
        todo!()
    }

    fn set_reg(&self, idx: usize, val: Word) {
        todo!()
    }
}

const ISA_LOGO: &'static str = r"
       _                         __  __                         _ 
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
