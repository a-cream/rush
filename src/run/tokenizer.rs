use super::node::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for part in input.split_whitespace() {
        match part {
            "|" => tokens.push(Token::Pipe),
            "&&" => tokens.push(Token::And),
            ">" => tokens.push(Token::OverwriteRedirection),
            ">>" => tokens.push(Token::AppendRedirection),
            "2>" => tokens.push(Token::ErrorRedirection),
            arg => tokens.push(Token::Arg(arg.to_string())),
        }
    }
    tokens
}
