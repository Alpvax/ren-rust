use crate::ast::expression::Literal;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NoTokens,
    NonLiteral,
}

pub fn parse_object_literal(_lexer: &mut Lexer) -> Result<Literal, Error> {
    todo!("Parse object literal body")
}

pub fn parse_array_literal(_lexer: &mut Lexer) -> Result<Literal, Error> {
    todo!("Parse array literal body")
}

pub fn parse_literal(lexer: &mut Lexer) -> Result<Literal, Error> {
    if let Some(tok) = lexer.peek_n::<1>().as_token_array()[0] {
        match tok {
            // Array
            Token::SquareOpen => parse_array_literal(lexer),
            // Boolean
            Token::Bool(b) => {
                lexer.next();
                Ok(Literal::Boolean(b.clone()))
            }
            // Positive number
            Token::Number(n) => {
                lexer.next();
                Ok(Literal::Number(n.clone()))
            }
            // Negative number. Should this be treated as a literal or handled as a unary expression?
            Token::OpSub => {
                match lexer.peek_n::<3>().as_token_array() {
                    [_, Some(Token::Whitespace), Some(Token::Number(n))]
                    | [_, Some(Token::Number(n)), _] => {
                        lexer.next(); //Consume '-'
                        consume_whitespace(lexer);
                        lexer.next(); //Consume number
                        Ok(Literal::Number(-n))
                    }
                    _ => Err(Error::NonLiteral),
                }
            }
            // Object
            Token::CurlyOpen => parse_object_literal(lexer),
            // String
            Token::StringLit(s) => {
                lexer.next();
                Ok(Literal::String(s.to_owned()))
            }
            _ => Err(Error::NonLiteral),
        }
    } else {
        Err(Error::NoTokens)
    }
}
