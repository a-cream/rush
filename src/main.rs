use crate::rush::run;
use libc::{signal, SIGINT, SIG_IGN};
use std::env::{self};
use std::path::PathBuf;
mod rush;

fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }

    let mut shell = rush::Rush::new(None);
    let mut original_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    loop {
        match shell.input() {
            Ok(user_input) => {
                run::execute(&user_input);
                handle_dir(&mut shell, &mut original_dir);
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}

fn handle_dir(shell: &mut rush::Rush, original_dir: &mut PathBuf) {
    let updated_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };
    if *original_dir != updated_dir {
        if updated_dir.to_str() == run::dir::home_directory() {
            shell.update_dir(PathBuf::from("~"));
        } else {
            shell.update_dir(updated_dir.clone());
        }
        *original_dir = updated_dir;
    }
}
