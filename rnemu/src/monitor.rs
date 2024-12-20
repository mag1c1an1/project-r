use std::{sync::OnceLock, u64};

use clap::Parser;
use colored::Colorize;
use sdb::{init_sdb, main_loop};

use crate::{
    core::{init_nemu, nemu_exec},
    debug::init_log,
    isa::GUEST_ISA,
    time::now,
};

static PORT: OnceLock<usize> = OnceLock::new();

mod sdb;

fn welcome() {
    log!("Trace: {}", mux!("trace", "ON".green(), "OFF".red().bold()));

    if cfg!(feature = "trace") {
        log!(
            "If trace is enabled, a log file will be generated to record the trace. \
            This may lead to a large log file. \
            If it is not necessary, you can disable it in memuconfig.",
        );
    }
    log!("Build time: {}", now().format("%H:%M:%S %Y-%m-%d"));
    println!("Welcome to {}-NEMU!", GUEST_ISA.yellow().on_red());
    println!("For help, type \"help\"");
}

fn load_img() {}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// run with batch mode
    #[arg(short, long)]
    batch: bool,
    /// output log to file
    #[arg(short)]
    log: Option<String>,
    /// run DiffTest with reference REF_SO
    #[arg(short)]
    diff: Option<String>,
    /// run DiffTtest with port PORT
    #[arg(short)]
    port: Option<String>,
    #[arg(short)]
    /// img file
    image_file: Option<String>,
}

pub fn init_monitor() {
    let args = Args::parse();
    init_log(args.log);
    init_nemu(args.image_file);
    init_sdb(args.batch);
    welcome();
}
pub fn engine_start() {
    if cfg!(feature = "am") {
        nemu_exec(u64::MAX);
    } else {
        main_loop();
    }
}
