use crate::{
    common::{Vaddr, Word},
    cpu::{decode::pattern_decode, CPU},
};

pub const GUEST_ISA: &'static str = "riscv32";

pub struct Riscv32CpuState {
    pub gpr: [Word; 32],
    pub pc: Vaddr,
}

pub struct Riscv32ISADecodeInfo {
    inst: u32,
}

const ISA_LOGO: &'static str = r"
       _                         __  __                         _ 
      (_)                       |  \/  |                       | |
  _ __ _ ___  ___ ________   __ | \  / | __ _ _ __  _   _  __ _| |
 | '__| / __|/ __|______\ \ / / | |\/| |/ _` | '_ \| | | |/ _` | |
 | |  | \__ \ (__        \ V /  | |  | | (_| | | | | |_| | (_| | |
 |_|  |_|___/\___|        \_/   |_|  |_|\__,_|_| |_|\__,_|\__,_|_|

";

enum OperandType {
    I,
    U,
    S,
    N,
}

struct Decode {
    pc: Vaddr,
    snpc: Vaddr,
    dnpc: Vaddr,
    isa: Riscv32ISADecodeInfo,
}

impl Decode {
    fn exec_once(&mut self, pc: Vaddr) {}

    fn decode_exec(&mut self, cpu: &mut CPU) {
        self.dnpc = self.snpc;
        self.decode_helper(cpu);
        cpu.set_reg(0, 0);
    }

    fn decode_helper(&mut self, cpu: &mut CPU) {
        {
            let (key, mask, shift) =
                pattern_decode("??????? ????? ????? ??? ????? 00101 11").unwrap();
            if ((self.isa.inst) as u64 >> shift) & mask == key {
                let mut rd = 0;
                let mut src1 = 0;
                let mut src2 = 0;
                let mut imm = 0;
                self.decode_operand(cpu, &mut rd, &mut src1, &mut src2, &mut imm, OperandType::U);
                cpu.set_reg(rd, self.pc + imm);
                return;
            }
        }
    }

    fn decode_operand(
        &mut self,
        cpu: &mut CPU,
        rd: &mut u32,
        src1: &mut Word,
        src2: &mut Word,
        imm: &mut Word,
        typ: OperandType,
    ) {
        let i = self.isa.inst;
        let rs1 = bits!(i, 19, 15) as usize;
        let rs2 = bits!(i, 24, 20) as usize;
        *rd = bits!(i, 11, 7);
        match typ {
            OperandType::I => {
                *src1 = cpu.read_reg(rs1);
                *imm = sext!(bits!(i, 31, 20), 12) as u32;
            }
            OperandType::U => *imm = (sext!(bits!(i, 31, 12), 20) << 12) as u32,
            OperandType::S => {
                *src1 = cpu.read_reg(rs1);
                *src2 = cpu.read_reg(rs2);
                *imm = ((sext!(bits!(i, 31, 25), 7) << 5) as u32) | (bits!(i, 11, 7) as u32);
            }
            OperandType::N => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        print!("{ISA_LOGO}");
    }
}
