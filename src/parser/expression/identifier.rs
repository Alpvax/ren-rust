use crate::ast::Identifier;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NoTokens,
    InvalidOperator,
    MissingPeriod,
    MissingName,
    InvalidNamespace,
    InvalidFieldName,
}

pub fn parse_operator(lexer: &mut Lexer) -> Result<Identifier, Error> {
    if let Some(l) = lexer.peek_n_exact::<3>() {
        if let [Token::ParenOpen, t_op, Token::ParenClose] = l.as_token_array_unchecked() {
            if let Ok(op) = Operator::try_from(t_op) {
                lexer.nth(2); //Consume '(' op ')'
                return Ok(Identifier::Operator(op));
            }
        }
    }
    Err(Error::InvalidOperator)
}

fn parse_scoped(lexer: &mut Lexer) -> Result<Identifier, Error> {
    let mut ns = Vec::new();
    while let [Some(Token::Namespace(n)), Some(Token::Period)] =
        lexer.peek_n::<2>().as_token_array()
    {
        ns.push(n.clone());
        lexer.nth(1);
    }
    if ns.len() < 1 {
        match lexer.peek_n::<2>().as_token_array() {
            [Some(&Token::Namespace(_)), _] => Err(Error::MissingPeriod),
            _ => Err(Error::InvalidNamespace),
        }
    } else if let Some(Token::VarName(n)) = lexer.peek_token() {
        let name = n.clone();
        lexer.next();
        Ok(Identifier::Scoped(ns, name))
    } else {
        Err(Error::MissingName)
    }
}

pub fn parse_identifier(lexer: &mut Lexer) -> Result<Identifier, Error> {
    match lexer.peek_token() {
        None => Err(Error::NoTokens),
        Some(Token::ParenOpen) => parse_operator(lexer),
        Some(Token::Namespace(_)) => parse_scoped(lexer),
        Some(Token::VarName(n)) => {
            let name = n.clone();
            lexer.next();
            Ok(Identifier::Local(name))
        }
        Some(Token::Period) => {
            lexer.next(); //Consume '.'
            if let Some(Token::VarName(n)) = lexer.next_token() {
                Ok(Identifier::Field(n))
            } else {
                Err(Error::InvalidFieldName)
            }
        }
        _ => todo!("invalid?"),
    }
}
