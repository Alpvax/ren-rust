use std::collections::HashMap;

use crate::ast::expression::Literal;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NoTokens,
    NonLiteral,
    InvalidStartToken,
    UnclosedObject,
    UnclosedArray,
    InvalidKey,
    DuplicateKey,
    MissingValue,
    InvalidValue(ExpressionError),
    MissingComma,
}

pub fn parse_object_literal(lexer: &mut Lexer) -> Result<Literal, Error> {
    if let Some(Token::CurlyOpen) = lexer.peek_token() {
        lexer.next(); //Consume '{'
        let mut map = HashMap::new();
        loop {
            if let Some(tok) = lexer.next_token() {
                match tok {
                    Token::Whitespace => continue,
                    Token::CurlyClose => break,
                    Token::VarName(key) => {
                        if map.contains_key(&key) {
                            return Err(Error::DuplicateKey);
                        }
                        consume_whitespace(lexer);
                        let mut t = lexer.peek_token();
                        if let Some(Token::Colon) = t {
                            lexer.next(); //Consume ':'
                            consume_whitespace(lexer);
                            map.insert(
                                key.clone(),
                                parse_expression(lexer).map_err(|e| Error::InvalidValue(e))?,
                            );
                            consume_whitespace(lexer);
                            t = lexer.peek_token();
                        }
                        if match t {
                            Some(Token::Comma) => {
                                lexer.next(); //Consume ','
                                true
                            }
                            Some(Token::CurlyClose) => true,
                            _ => return Err(Error::MissingComma),
                        } && !map.contains_key(&key)
                        {
                            map.insert(key.clone(), Expression::local_var(&key));
                        }
                    }
                    _ => return Err(Error::InvalidKey),
                }
            } else {
                return Err(Error::UnclosedObject);
            }
        }
        Ok(Literal::Object(map))
    } else {
        Err(Error::InvalidStartToken)
    }
}

pub fn parse_array_literal(lexer: &mut Lexer) -> Result<Literal, Error> {
    if let Some(Token::SquareOpen) = lexer.peek_token() {
        lexer.next(); //Consume '['
        let mut values = Vec::new();
        loop {
            consume_whitespace(lexer);
            match lexer.peek_token() {
                Some(Token::SquareClose) => {
                    lexer.next(); //Consume ']'
                    break;
                }
                Some(_) => {
                    values.push(parse_expression(lexer).map_err(|e| Error::InvalidValue(e))?);
                    consume_whitespace(lexer);
                    match lexer.peek_token() {
                        Some(Token::Comma) => {
                            lexer.next(); //Consume ','
                        }
                        Some(Token::SquareClose) => continue,
                        _ => return Err(Error::MissingValue),
                    }
                }
                _ => return Err(Error::UnclosedArray),
            }
        }
        Ok(Literal::Array(values))
    } else {
        Err(Error::InvalidStartToken)
    }
}

pub fn parse_literal(lexer: &mut Lexer) -> Result<Literal, Error> {
    if let Some(tok) = lexer.peek_n::<1>().as_token_array()[0] {
        match tok {
            // Array
            Token::SquareOpen => parse_array_literal(lexer),
            // Boolean
            // Token::Bool(b) => {
            //     lexer.next();
            //     Ok(Literal::Boolean(b.clone()))
            // }
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
            // Token::Undefined => {
            //     lexer.next();
            //     Ok(Literal::Undefined)
            // }
            _ => Err(Error::NonLiteral),
        }
    } else {
        Err(Error::NoTokens)
    }
}
