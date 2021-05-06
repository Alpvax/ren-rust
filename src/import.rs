use crate::Token;
use crate::names::{Namespace, VarName};
use crate::parser::{ParseError, Parser, Statement};

#[derive(Debug)]
pub struct Import {
    path: String,
    namespace: Option<Vec<Namespace>>,
    exposing: Option<Vec<VarName>>,
}

impl Import {
    pub fn new(path: &str, namespace: Option<Vec<&str>>, exposing: Option<Vec<&str>>) -> Import {
        Import {
            path: path.to_owned(),
            namespace: namespace.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
            exposing: exposing.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
        }
    }
}

pub fn parse_import<'s>(parser: &mut Parser<'s>) -> Result<Statement, ParseError<'s>> {
    let mut tok = lexer.next().ok_or(ParseError::UnexpectedEOF)?;
    let path = parse_str(tok).or_else(|tok| {
        Err(ParseError::UnexpectedToken(
            tok,
            "Expected string <path>".to_owned(),
        ))
    })?;
    let namespace =
        if let Token::KWAs = lexer.next().ok_or(ParseError::UnexpectedEOF)? {
            let tok = lexer.next().ok_or(ParseError::UnexpectedEOF)?;
            if let Token::Namespace(ns) = tok {
                Some(ns)
            } else {
                Err(ParseError::UnexpectedToken(
                    tok,
                    "Expected namespace".to_owned(),
                ))?
            }
        } else {
            None
        };
    let exposing =
        if let Token::KWExposing = lexer.next().ok_or(ParseError::UnexpectedEOF)? {
            tok = lexer.next().ok_or(ParseError::UnexpectedEOF)?;
            if let Token::CurlyOpen = tok {
                let mut vec = Vec::new();
                loop {
                    tok = lexer.next().ok_or(ParseError::UnexpectedEOF)?;
                    match tok {
                        Token::VarName(n) => vec.push(n),
                        Token::Comma => continue,
                        Token::CurlyClose => break,
                        _ => Err(ParseError::UnexpectedToken(
                            tok,
                            "Expected comma seperated list of names".to_owned(),
                        ))?,
                    }
                }
                Some(vec)
            } else {
                Err(ParseError::UnexpectedToken(
                    tok,
                    "Expected opening curly brace".to_owned(),
                ))?
            }
        } else {
            None
        };
    Ok(Statement::Import(Import::new(
        path, namespace, exposing,
    )))
}
