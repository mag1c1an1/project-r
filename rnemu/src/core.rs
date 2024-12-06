use std::sync::{Arc, Mutex};

use crate::{common::Vaddr, cpu::CPU};
use lazy_static::lazy_static;

lazy_static! {
    // static ref NEMU: Arc<Mutex<Nemu>> = Arc::new(Mutex::new(Nemu::new()));
}

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
    cpu: CPU,
}

impl Nemu {
    fn new() -> Self {
        todo!()
    }

    fn execute(n: u64) {
        todo!()
    }

    fn exec(&mut self) {
        todo!()
    }
}

pub fn engine_start() {
    // NEMU.lock().unwrap().exec();
}
