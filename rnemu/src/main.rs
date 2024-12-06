use core::engine_start;

#[macro_use]
mod common;
mod core;
mod cpu;
mod debug;
mod isa;
mod memory;
mod monitor;

fn main() {
    engine_start();
}
