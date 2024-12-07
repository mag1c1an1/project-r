use chrono::TimeDelta;
use spin::mutex::SpinMutex;
use std::sync::{Arc, OnceLock};

use crate::{
    common::Vaddr,
    isa::{Riscv32, ISA},
    memory::MemoryBank,
    time::now,
};

static NEMU: OnceLock<Arc<SpinMutex<Nemu<Riscv32>>>> = OnceLock::new();

#[derive(PartialEq, Eq)]
enum NemuState {
    Running,
    Stop,
    Quit,
    Abort,
    End,
}

struct Nemu<T: ISA> {
    state: NemuState,
    halt_pc: Vaddr,
    halt_ret: u32,
    cpu: T,
    // statistic
    timer: TimeDelta,
    nr_guest_inst: u64,
    mem: MemoryBank,
}

impl<T: ISA> Nemu<T> {
    fn new() -> Self {
        todo!()
    }

    fn exec_once(&mut self) {
        todo!()
    }

    fn execute(&mut self, n: u64) {
        for _ in 0..n {
            self.exec_once();
            self.nr_guest_inst += 1;
            if self.state == NemuState::Running {
                break;
            }
            if cfg!(feature = "device") {
                todo!()
            }
        }
    }

    fn exec(&mut self, n: u64) {
        use NemuState::*;
        match &self.state {
            End | Abort | Quit => {
                println!(
                    "Program execution has ended. To restart the program, exit NEMU and run again."
                );
            }
            _ => {
                self.state = Running;
            }
        }

        let start_time = now();
        self.execute(n);
        let end_time = now();
        self.timer += end_time - start_time;
        match &self.state {
            Running => {
                self.state = Stop;
                return;
            }
            End | Abort => {
                todo!()
            }
            _ => {}
        }
        self.statistic();
    }

    fn statistic(&self) {}

    pub fn inst_fetch() {}
}

pub fn init_nemu() {}

pub fn engine_start() {
    // NEMU.lock().unwrap().exec();
}
