mod ast;
mod bic;
mod error;
mod executor;
mod node;
mod tokenizer;
use super::run::error::ShellError;

pub fn execute(input: &str) {
    let tokens = tokenizer::tokenize(input);
    let ast = ast::parse(&tokens).expect("Parsing failed");
    match executor::execute(&ast) {
        Ok(_) => (),
        Err(ShellError::BicError(msg)) => {
            eprintln!("{msg}")
        }
        Err(e) => eprintln!("rush: {e}"),
    }
}
