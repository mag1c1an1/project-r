use chrono::TimeDelta;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use spin::mutex::SpinMutex;
use std::{fmt, slice::Windows, sync::OnceLock};

use crate::{
    common::{Vaddr, Word},
    isa::{Executer, Riscv32, ISA, ISA_LOGO},
    log,
    memory::{MemoryBank, RESET_VECTOR},
    time::now,
};
cfg_if::cfg_if! {
    if #[cfg(feature="riscv32")] {
        static NEMU: OnceLock<SpinMutex<Nemu<Riscv32>>> = OnceLock::new();
    }
}

#[derive(PartialEq, Eq)]
enum NemuState {
    Running,
    Stop,
    Quit,
    Abort,
    End,
}

impl fmt::Display for NemuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NemuState::Running => write!(f, "{}", "RUNNING"),
            NemuState::Stop => write!(f, "{}", "STOP"),
            NemuState::Quit => write!(f, "{}", "QUIT"),
            NemuState::Abort => write!(f, "{}", "ABORT"),
            NemuState::End => write!(f, "{}", "END"),
        }
    }
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

impl Nemu<Riscv32> {
    fn new(cpu: Riscv32, img: Option<String>) -> Self {
        let default_img = Riscv32::default_img();
        let mem = MemoryBank::new(&default_img);
        Self {
            state: NemuState::Stop,
            halt_pc: 0,
            halt_ret: 0,
            cpu,
            timer: TimeDelta::default(),
            nr_guest_inst: 0,
            mem,
        }
    }

    fn execute(&mut self, n: u64) -> Result<(), Vaddr> {
        let mut executer = Riscv32::executer();
        for _ in 0..n {
            self.nr_guest_inst += 1;
            self.exec_once(&mut executer, self.cpu.pc())?;
            if self.state != NemuState::Running {
                break;
            }
            if cfg!(feature = "device") {
                todo!()
            }
        }
        Ok(())
    }
    fn exec_once(&mut self, executer: &mut Executer, pc: Vaddr) -> Result<(), Vaddr> {
        executer.set_pc(pc);
        executer.set_snpc(pc);
        executer.exec_once(&mut self.cpu, &mut self.mem)?;
        self.cpu.set_pc(executer.dnpc());
        Ok(())
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
        if let Err(pc) = self.execute(n) {
            self.invalid(pc);
        }
        let end_time = now();
        self.timer += end_time - start_time;
        match &self.state {
            Running => {
                self.state = Stop;
                return;
            }
            End => {
                let x = if self.halt_ret == 0 {
                    "HIT GOOD TRAP".green().bold()
                } else {
                    "HIT BAD TRAP".green().bold()
                };
                log!("nemu: {} at pc = 0x{:x}", x, self.halt_pc)
            }
            Abort => {
                log!(
                    "nemu: {} at pc = 0x{:x}",
                    format!("{}", self.state).red().bold(),
                    self.halt_pc
                );
            }
            _ => {}
        }
        self.statistic();
    }

    fn statistic(&self) {
        let time = self.timer.num_microseconds().unwrap_or(0) as u64;
        log!("host time spent = {} us", time);
        log!("total guest instructions = {}", self.nr_guest_inst);

        if time > 0 {
            let val = self.nr_guest_inst * 1000000 / time;
            log!(
                "simulation frequency = {} inst/s",
                val.to_formatted_string(&Locale::en)
            );
        } else {
            log!("Finish running in less than 1 us and can not calculate the simulation frequency");
        }
    }

    fn set_state(&mut self, state: NemuState, halt_pc: Vaddr, halt_ret: u32) {
        self.state = state;
        self.halt_pc = halt_pc;
        self.halt_ret = halt_ret;
    }

    fn invalid(&mut self, this_pc: Vaddr) {
        let mut pc = this_pc;
        let mut tmp: [Word; 2] = [0; 2];
        tmp[0] = self.mem.inst_fetch(&mut pc, 4);
        tmp[1] = self.mem.inst_fetch(&mut pc, 4);

        let ptr = tmp.as_ptr() as *const u8;
        let len = tmp.len() * std::mem::size_of::<u32>();
        let p = unsafe { std::slice::from_raw_parts(ptr, len) };
        println!("invalid opcode(PC = 0x{:x}):", this_pc);
        println!(
            "\t{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} ...",
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]
        );
        println!("\t{:08x} {:08x} ...", tmp[0], tmp[1]);
        println!("There are two cases which will trigger this unexpected exception:");
        println!(
            "1. The instruction at PC =  0x{:x}  is not implemented.",
            this_pc
        );
        println!("2. Something is implemented incorrectly.");
        println!(
            "Find this PC(0x{:x}) in the disassembling result to distinguish which case it is.\n",
            this_pc
        );
        let x = format!(
            "If it is the first case, see\n{}\n\
    for more details.\n\nIf it is the second case, remember:\n\
    * The machine is always right!\n\
    * Every line of untested code is always wrong!",
            ISA_LOGO
        );

        println!("{}", x.red().bold());
        self.set_state(NemuState::Abort, this_pc, u32::MAX);
    }
}

impl Nemu<Riscv32> {}

pub fn init_nemu(img: Option<String>) {
    if cfg!(feature = "riscv32") {
        NEMU.get_or_init(|| {
            let cpu = Riscv32::new(RESET_VECTOR as Vaddr);
            let nemu = Nemu::new(cpu, img);
            SpinMutex::new(nemu)
        });
    }
}

pub fn nemu_exec(n: u64) {
    NEMU.get().unwrap().lock().exec(u64::MAX);
}
