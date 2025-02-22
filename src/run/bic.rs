use crate::run;
use libc::{getrusage, rusage, RUSAGE_SELF};
use std::{env, path::Path, time::Instant};

pub fn cd(arg: &str) -> Result<(), String> {
    let new_path = if arg.is_empty() || arg == "~" {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        arg.to_string()
    };

    let path = Path::new(&new_path);
    match env::set_current_dir(path) {
        Ok(()) => Ok(()),
        Err(_) => Err(format!(
            "cd: The directory '{}' does not exist\n",
            path.display()
        )),
    }
}

fn get_time_usage() -> (f64, f64) {
    let mut usage: rusage = unsafe { std::mem::zeroed() };
    unsafe { getrusage(RUSAGE_SELF, &mut usage) };

    let user_time = usage.ru_utime.tv_sec as f64 + usage.ru_utime.tv_usec as f64 / 1_000_000.0;
    let sys_time = usage.ru_stime.tv_sec as f64 + usage.ru_stime.tv_usec as f64 / 1_000_000.0;

    (user_time, sys_time)
}

pub fn time(args: &str) {
    let (usr_start, sys_start) = get_time_usage();
    let start = Instant::now();
    run::execute(args);
    let duration = start.elapsed();
    let (usr_end, sys_end) = get_time_usage();
    println!();
    println!("{}", "_".repeat(27_usize));
    println!(
        "Executed in  {:>6.2} millis",
        duration.as_secs_f64() * 1000.0
    );
    println!(
        "   usr time  {:>6.2} millis",
        (usr_end - usr_start) * 1000.0
    );
    println!(
        "   sys time  {:>6.2} millis",
        (sys_end - sys_start) * 1000.0
    );
}

pub fn exit(code: i32) {
    std::process::exit(code);
}
