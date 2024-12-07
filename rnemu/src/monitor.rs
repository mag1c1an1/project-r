use std::sync::OnceLock;

use clap::Parser;
use colored::Colorize;

use crate::{debug::init_log, isa::GUEST_ISA, time::now};

static PORT: OnceLock<usize> = OnceLock::new();
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
    println!("For help, type \"help\"\n");
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
}

pub fn init_monitor() {
    let args = Args::parse();
    init_log(args.log);
    welcome();
}
