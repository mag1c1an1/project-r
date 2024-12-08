use std::{sync::OnceLock, u64};

use lazy_static::lazy_static;
use rustyline::error::ReadlineError;
use spin::mutex::SpinMutex;

use crate::core::nemu_exec;

static DEBUGGER: OnceLock<SpinMutex<Debugger>> = OnceLock::new();

struct Debugger {
    batch_mode: bool,
}

impl Debugger {
    pub fn new(batch_mode: bool) -> Self {
        Self { batch_mode }
    }

    pub fn run(&self) {
        if self.batch_mode {
            nemu_exec(u64::MAX);
        }
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline("(nemu) ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    if self.exec(line.trim()) == -1 {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    // println!("CTRL-D");
                    break;
                }
                Err(_) => println!("No input"),
            }
        }
        rl.save_history("history.txt");
    }

    fn exec(&self, line: &str) -> i32 {
        let splits: Vec<&str> = line.split_whitespace().collect();
        let (head, args) = (splits[0], &splits[1..].join(" "));
        match head {
            "c" => {
                nemu_exec(u64::MAX);
                return -1;
            }
            "help" => cmd_help(args),
            "q" => {
                return -1;
            }
            _ => {
                println!("Unknown command {}", line);
            }
        }
        0
    }
}

lazy_static::lazy_static! {
    static ref CMD_TABLE:Vec<(&'static str,&'static str)> = {
        vec!{
            ("help","Display information about all supported commands"),
            ("c","Continue the execution of the program"),
            ("si","si [N] 让程序单步执行N条指令后暂停执行,当N没有给出时, 缺省为1"),
            ("info","info r/w 打印寄存器状态,打印监视点信息"),
            ("p","p EXPR 求出表达式EXPR的值"),
            ("x","x N EXPR 求出表达式EXPR的值, 将结果作为起始内存地址, 以十六进制形式输出连续的N个4字节"),
            ("w","w EXPR 当表达式EXPR的值发生变化时, 暂停程序执行"),
            ("d","d N 删除序号为N的监视点"),
            ("q","Exit NEMU"),
        }
    };
}

fn cmd_help(arg: &str) {
    if arg.is_empty() {
        for (name, desc) in CMD_TABLE.iter() {
            println!("{} - {}", name, desc)
        }
    } else {
        for (name, desc) in CMD_TABLE.iter() {
            if name == &arg {
                println!("{} - {}", name, desc);
                return;
            }
        }
        println!("Unknown Command: {}", arg);
    }
}

pub fn init_sdb(b: bool) {
    DEBUGGER.get_or_init(|| SpinMutex::new(Debugger::new(b)));
}

pub fn main_loop() {
    DEBUGGER.get().unwrap().lock().run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_test() {
        let s = "a     b c";
        let v: Vec<&str> = s.split_whitespace().collect();
        println!("{:?}", v);
    }
}
