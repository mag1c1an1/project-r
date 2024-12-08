#![allow(unused)]

use monitor::{engine_start, init_monitor};

#[macro_use]
mod common;
mod core;
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
