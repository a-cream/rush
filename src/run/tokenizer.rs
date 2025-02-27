use super::node::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for part in input.split_whitespace() {
        match part {
            "&&" => tokens.push(Token::AndLogical),
            "||" => tokens.push(Token::OrLogical),
            "&>" => tokens.push(Token::OutputRedirection),
            ">>" => tokens.push(Token::AppendRedirection),
            "2>" => tokens.push(Token::ErrorRedirection),
            
            "|" => tokens.push(Token::Pipe),
            "&" => tokens.push(Token::Background),
            ";" => tokens.push(Token::Separator),
            "<" => tokens.push(Token::InputRedirection),
            ">" => tokens.push(Token::OverwriteRedirection),
            
            arg => tokens.push(Token::Arg(arg.to_string())),
        }
    }
    tokens
}
