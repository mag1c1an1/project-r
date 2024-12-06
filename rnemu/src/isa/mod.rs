mod riscv32;

pub use riscv32::GUEST_ISA;

pub struct ISA {
    pub inner: riscv32::Riscv32CpuState,
}

impl ISA {}
