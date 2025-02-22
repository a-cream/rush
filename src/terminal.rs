use std::sync::Mutex;

pub struct Terminal {
    pub print: String,
    pub exit: bool,
    pub exit_code: Option<i32>,
}

pub static TERMINAL: Mutex<Terminal> = Mutex::new(Terminal {
    print: String::new(),
    exit: false,
    exit_code: None,
});
