use core::engine_start;
use monitor::init_monitor;

#[macro_use]
mod common;
mod core;
mod cpu;
#[macro_use]
mod debug;
mod isa;
mod memory;
mod monitor;
mod time;

fn main() {
    init_monitor();
    engine_start();
}
