use libc::{signal, tcgetattr, tcsetattr, termios, ECHOCTL, SIGINT, SIG_IGN, TCSANOW};
mod run;
mod shell;
mod terminal;

fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }

    let mut term: termios = unsafe { std::mem::zeroed() };
    unsafe {
        tcgetattr(libc::STDIN_FILENO, &mut term);
    }

    let original_term = term;
    term.c_lflag &= !ECHOCTL;

    unsafe {
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &term);
    }

    let exit_code = shell::run();

    unsafe {
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &original_term);
    }

    std::process::exit(exit_code);
}
