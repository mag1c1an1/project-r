use std::sync::Mutex;
use std::{fs::File, sync::OnceLock};

pub static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        let name = &name[..name.len() - 3];
        if let Some(n) = name.rfind(':') {
            &name[n + 1..]
        } else {
            name
        }
    }};
}

macro_rules! _log_file {
    ($arg:expr) => {
        use crate::debug::LOG_FILE;
        use std::io::Write;
        if let Some(mtx) = LOG_FILE.get() {
            let mut f = mtx.lock().expect("Get LOG_FILE Failed");
            write!(*f, "{}\n", $arg).expect("Write to LOG_FILE Failed");
        }
    };
}

macro_rules! myfile {
    () => {{
        let x = file!();
        if let Some(n) = x.find('/') {
            &x[n + 1..]
        } else {
            x
        }
    }};
}

macro_rules! log {
    () => {};
    ($($arg:tt)+) => {
        use colored::Colorize;
        let x = format!("{}",format_args!($($arg)+));
        let blue = format!("[{}:{} {}]",myfile!(),line!(),function_name!()).truecolor(59,142,234).bold();
        println!("{} {}",blue,x);
        _log_file!(x);
    };
}

pub fn init_log(file: Option<String>) {
    if let Some(file) = file {
        let f = File::create(file).unwrap();
        LOG_FILE.get_or_init(|| Mutex::new(f));
    }
}

#[cfg(test)]
mod tests {
    use crate::debug::init_log;
    use chrono::FixedOffset;
    use colored::Colorize;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn t() {
        let x = "rnemu::main";
        if let Some(n) = x.rfind(':') {
            println!("{:?}", &x[n + 1..]);
        }
    }

    #[test]
    fn write_test() {
        let mut f = File::create("foo.txt").unwrap();
        write!(f, "{}", "World").unwrap();
        write!(f, "{}", "World").unwrap();
        write!(f, "{}", "World").unwrap();
        write!(f, "{}", "World").unwrap();
    }

    #[test]
    fn log_test() {
        init_log(Some("foo.txt".into()));
        log!("hhh{}", "www");
    }
    #[test]
    fn color_test() {
        println!("{}", "this is red on blue".red().on_blue());
        println!("{}", "this is also red on blue".on_blue().red());
        println!(
            "{}",
            "you can use truecolor values too!".truecolor(0, 255, 136)
        );
        println!(
            "{}",
            "you can use truecolor values too!".truecolor(59, 142, 234)
        );
        println!(
            "{}",
            "you can use truecolor values too!"
                .custom_color(colored::CustomColor::new(59, 142, 234))
        );
        // "background truecolor also works :)".on_truecolor(135, 28, 167);
        // "truecolor from tuple".custom_color((0, 255, 136));
        // "background truecolor from tuple".on_custom_color((0, 255, 136));
        // "bright colors are welcome as well"
        //     .on_bright_blue()
        //     .bright_red();
        // "you can also make bold comments".bold();
        println!(
            "{} {} {}",
            "or use".cyan(),
            "any".italic().yellow(),
            "string type".cyan()
        );
        // "or change advice. This is red".yellow().blue().red();
        // "or clear things up. This is default color and style"
        //     .red()
        //     .bold()
        //     .clear();
        println!(
            "{}",
            "purple and magenta are the same"
                .purple()
                .magenta()
                .bold()
                .bold()
                .bold()
        );
        // "and so are normal and clear".normal().clear();
        // "you can specify color by string"
        //     .color("blue")
        //     .on_color("red");
        // String::from("this also works!").green().bold();
        // format!(
        //     "{:30}",
        //     "format works as expected. This will be padded".blue()
        // );
        // format!(
        //     "{:.3}",
        //     "and this will be green but truncated to 3 chars".green()
        // );
    }

    #[test]
    fn myfile_test() {
        println!("{}", myfile!());
    }

    #[test]
    fn time_test() {
        use chrono::{DateTime, Local};

        let dt = Local::now();
        println!("{}", dt);
        let naive_utc = dt.naive_utc();
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        println!("{}", dt_new);
        // assert_eq!(dt, dt_new);
    }
}
