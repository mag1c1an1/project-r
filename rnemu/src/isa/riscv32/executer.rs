use std::{os::unix::net::SocketAddr, sync::Arc};

use operand::{
    Args, BOperand, IOperand, JOperand, NOperand, OperandHelper, OperandType, ROperand, SOperand,
    UOperand,
};
use spin::mutex::SpinMutex;

use super::Riscv32;
use crate::{
    common::{Vaddr, Word},
    isa::ISA,
    memory::MemoryBank,
};

mod operand;
struct Decoder<P, A>
where
    P: Fn() -> Option<(u64, u64, u64)>,
    A: Fn(&mut Riscv32, &MemoryBank, Args),
{
    pred: P,
    apply: A,
    helper: Box<dyn OperandHelper>,
}

impl<P, A> Decoder<P, A>
where
    P: Fn() -> Option<(u64, u64, u64)>,
    A: Fn(&mut Riscv32, &MemoryBank, Args),
{
    pub fn new(pred: P, apply: A, typ: OperandType) -> Self {
        let helper: Box<dyn OperandHelper> = match typ {
            OperandType::I => Box::new(IOperand),
            OperandType::U => Box::new(UOperand),
            OperandType::S => Box::new(SOperand),
            OperandType::N => Box::new(NOperand),
            OperandType::R => Box::new(ROperand),
            OperandType::J => Box::new(JOperand),
            OperandType::B => Box::new(BOperand),
        };
        Self {
            pred,
            apply,
            helper,
        }
    }
}
trait Decode: Send {
    fn decode(&self, inst: Word) -> Option<(u64, u64, u64)>;
    fn apply(&self, inst: Word, cpu: &mut Riscv32, mem: &MemoryBank);
}

impl<P, A> Decode for Decoder<P, A>
where
    P: Fn() -> Option<(u64, u64, u64)> + Send,
    A: Fn(&mut Riscv32, &MemoryBank, Args) + Send,
{
    fn decode(&self, inst: Word) -> Option<(u64, u64, u64)> {
        (self.pred)()
    }

    fn apply(&self, inst: Word, cpu: &mut Riscv32, mem: &MemoryBank) {
        let args = self.helper.decode_operand(inst, cpu);
        (self.apply)(cpu, mem, args)
    }
}

fn pattern_decode(pattern: &str) -> Option<(u64, u64, u64)> {
    let mut key: u64 = 0;
    let mut mask: u64 = 0;
    let mut shift: u64 = 0;
    let mut temp_shift: u64 = 0;

    for (i, c) in pattern.chars().enumerate() {
        match c {
            '0' => {
                key = key << 1;
                mask = (mask << 1) | 1;
                temp_shift = 0;
            }
            '1' => {
                key = (key << 1) | 1;
                mask = (mask << 1) | 1;
                temp_shift = 0;
            }
            '?' => {
                key = key << 1;
                mask = mask << 1;
                temp_shift += 1;
            }
            ' ' => continue, // 忽略空格
            _ => {
                return None;
            }
        }

        shift = temp_shift; // 记录最后的连续 '?' 的数量
    }

    // 右移 `key` 和 `mask`，忽略末尾的 '?'
    Some((key >> shift, mask >> shift, shift))
}

macro_rules! pat {
    ($p:literal,$_name: ident,$o:expr, $b:expr) => {{
        let pred = || $crate::isa::riscv32::executer::pattern_decode($p);
        let a = $b;
        Box::new($crate::isa::riscv32::executer::Decoder::new(pred, a, $o))
            as Box<dyn $crate::isa::riscv32::executer::Decode>
    }};
}

pub struct Executer {
    pc: Vaddr,
    snpc: Vaddr,
    dnpc: Vaddr,
    inst: Word,
    decoders: Arc<SpinMutex<Vec<Box<dyn Decode>>>>,
}

impl Executer {
    pub fn new() -> Self {
        Self {
            pc: 0,
            snpc: 0,
            dnpc: 0,
            inst: 0,
            decoders: DECODERS.clone(),
        }
    }

    pub fn exec_once(&mut self, cpu: &mut Riscv32, mem: &mut MemoryBank) -> Result<(), Vaddr> {
        self.inst = mem.inst_fetch(&mut self.snpc, 4);
        self.dnpc = self.snpc;
        let guard = self.decoders.lock();
        for d in guard.iter() {
            if let Some((key, mask, shift)) = d.decode(self.inst) {
                if ((self.inst as u64) >> shift) & mask == key {
                    d.apply(self.inst, cpu, mem);
                    cpu.set_reg(0, 0);
                    return Ok(());
                }
            }
        }
        Err(self.pc)
    }
    pub fn set_pc(&mut self, pc: Vaddr) {
        self.pc = pc;
    }
    pub fn set_snpc(&mut self, snpc: Vaddr) {
        self.snpc = snpc;
    }

    pub fn dnpc(&self) -> Vaddr {
        self.dnpc
    }
}

lazy_static::lazy_static! {
    static ref DECODERS: Arc<SpinMutex<Vec<Box<dyn Decode>>>> = {
        let decoders =  vec!{
            // pat!(
            //     "??????? ????? ????? ??? ????? 00101 11",
            //     auipc,
            //     OperandType::U,
            //     |cpu: &mut Riscv32, mem: &MemoryBank,args:Args| {
            //         cpu.set_reg(args.rd, cpu.pc + args.imm);
            //     }
            // ),
        };
        Arc::new(SpinMutex::new(decoders))
    };
}
mod tests {
    use crate::isa::{Riscv32, ISA};
    use crate::memory::MemoryBank;

    use super::*;
    #[test]
    fn macro_test() {
        let x = pat!(
            "00000 00 0 00 0 00 0",
            li,
            OperandType::U,
            |cpu: &mut Riscv32, mem: &MemoryBank, args: Args| { cpu.set_pc(3) }
        );
        let y = |cpu: &mut Riscv32, mem: &MemoryBank| {
            cpu.set_pc(3);
        };
    }

    #[test]
    fn executer_test() {
        let executer = Executer::new();
        println!("success");
    }
}
