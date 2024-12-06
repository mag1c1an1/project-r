use common::Vaddr;
use cpu::CPU;
use memory::MemoryBank;

#[macro_use]
mod common;
mod cpu;
mod isa;
mod memory;
mod monitor;

enum State {
    Running,
    Stop,
    Quit,
    Abort,
    End,
}

struct NemuState {
    state: State,
    halt_pc: Vaddr,
    halt_ret: u32,
}

struct Nemu {
    state: NemuState,
    mem: MemoryBank,
    cpu: CPU,
}

impl Nemu {
    fn execute(n: u64) {
        todo!()
    }

    fn exec(&mut self) {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
