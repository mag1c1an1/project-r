use crate::{
    common::Word,
    isa::{Riscv32, ISA},
};

pub trait OperandHelper: Send {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args;
}
pub enum OperandType {
    I,
    U,
    S,
    J,
    R,
    B,
    N,
}

pub struct Args {
    pub rd: usize,
    pub src1: Word,
    pub src2: Word,
    pub imm: Word,
}

impl Args {
    pub fn new(rd: usize, src1: Word, src2: Word, imm: Word) -> Self {
        Self {
            rd,
            src1,
            src2,
            imm,
        }
    }
}

pub struct ROperand;
impl OperandHelper for ROperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let rs1 = bits!(inst, 19, 15);
        let rs2 = bits!(inst, 24, 20);
        let rd = bits!(inst, 11, 7);
        let src1 = isa.reg(rs1 as usize);
        let src2 = isa.reg(rs2 as usize);
        let imm = 0;
        Args::new(rd as usize, src1, src2, imm)
    }
}
pub struct BOperand;
impl OperandHelper for BOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let rs1 = bits!(inst, 19, 15);
        let rs2 = bits!(inst, 24, 20);
        let rd = bits!(inst, 11, 7);
        let src1 = isa.reg(rs1 as usize);
        let src2 = isa.reg(rs2 as usize);
        let imm = (sext!(bits!(inst, 31, 31), 1) << 20)
            | (bits!(inst, 30, 21) << 1)
            | (bits!(inst, 20, 20) << 11)
            | (bits!(inst, 19, 12) << 12);
        Args::new(rd as usize, src1, src2, imm)
    }
}
pub struct UOperand;
impl OperandHelper for UOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let imm = sext!(bits!(inst, 31, 12), 20) << 12;
        Args::new(0, 0, 0, imm)
    }
}
pub struct JOperand;
impl OperandHelper for JOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let imm = (sext!(bits!(inst, 31, 31), 1) << 20)
            | (bits!(inst, 30, 21) << 1)
            | (bits!(inst, 20, 20) << 11)
            | (bits!(inst, 19, 12) << 12);
        Args::new(0, 0, 0, imm)
    }
}
pub struct SOperand;
impl OperandHelper for SOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let rs1 = bits!(inst, 19, 15);
        let rs2 = bits!(inst, 24, 20);
        let rd = bits!(inst, 11, 7);
        let src1 = isa.reg(rs1 as usize);
        let src2 = isa.reg(rs2 as usize);
        let imm = (sext!(bits!(inst, 31, 25), 7) << 5) | bits!(inst, 11, 7);

        Args::new(rd as usize, src1, src2, imm)
    }
}
pub struct NOperand;
impl OperandHelper for NOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        Args::new(0, 0, 0, 0)
    }
}
pub struct IOperand;
impl OperandHelper for IOperand {
    fn decode_operand(&self, inst: Word, isa: &Riscv32) -> Args {
        let rs1 = bits!(inst, 19, 15);
        let rs2 = bits!(inst, 24, 20);
        let rd = bits!(inst, 11, 7);
        let src1 = isa.reg(rs1 as usize);
        let src2 = 0;
        let imm = sext!(bits!(inst, 31, 20), 12);
        Args::new(rd as usize, src1, src2, imm)
    }
}
