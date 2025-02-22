mod run;
mod shell;
mod terminal;
use libc::{signal, SIGINT, SIG_IGN};

fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }

    shell::run();
}
