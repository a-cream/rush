use super::node::{Ast, LogicType, Token};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        self.expression(0)
    }

    fn expression(&mut self, precedence: u8) -> Result<Ast, String> {
        let mut left = self.primary()?;

        while let Some(&token) = self.tokens.peek() {
            let token_precedence = Parser::precedence(token);

            if token_precedence < precedence {
                break;
            }

            let token = self.tokens.next().unwrap();

            left = self.infix(left, &token, token_precedence)?;
        }

        Ok(left)
    }

    fn primary(&mut self) -> Result<Ast, String> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Arg(value) => {
                    let mut args = vec![value.clone()];

                    while let Some(&&Token::Arg(_)) = self.tokens.peek() {
                        if let &Token::Arg(ref value) = self.tokens.next().unwrap() {
                            args.push(value.clone());
                        }
                    }

                    Ok(Ast::Command(args))
                }
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            Err("Unexpected end of tokens".to_string())
        }
    }

    fn infix(&mut self, left: Ast, token: &Token, precedence: u8) -> Result<Ast, String> {
        match token {
            Token::Pipe => {
                let right = self.expression(precedence + 1)?;
                Ok(Ast::Pipe(Box::new(left), Box::new(right)))
            }
            Token::Background => {
                if self.tokens.peek().is_some() {
                    let right = self.expression(precedence + 1)?;
                    Ok(Ast::Background(Box::new(left), Box::new(right)))
                } else {
                    Ok(Ast::Background(
                        Box::new(left),
                        Box::new(Ast::Command(vec![])),
                    ))
                }
            }
            Token::Logic(LogicType::And) => {
                let right = self.expression(precedence + 1)?;
                Ok(Ast::Logic(Box::new(left), Box::new(right), LogicType::And))
            }
            Token::Logic(LogicType::Or) => {
                let right = self.expression(precedence + 1)?;
                Ok(Ast::Logic(Box::new(left), Box::new(right), LogicType::Or))
            }
            Token::Redirect(redirect_type) => {
                let right = self.expression(precedence + 1)?;
                Ok(Ast::Redirect(
                    Box::new(left),
                    Box::new(right),
                    redirect_type.clone(),
                ))
            }
            Token::Separator => {
                let right = self.expression(precedence + 1)?;
                Ok(Ast::Separator(Box::new(left), Box::new(right)))
            }
            _ => Err(format!("Unexpected infix token: {:?}", token)),
        }
    }

    fn precedence(token: &Token) -> u8 {
        match token {
            Token::Separator => 1,
            Token::Background => 10,
            Token::Logic(_) => 20,
            Token::Pipe => 30,
            Token::Redirect(_) => 40,
            Token::Arg(_) => 50,
        }
    }
}
