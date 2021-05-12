use crate::names::{Namespace, VarName};
use crate::parser::{ParseError, Parser, Statement};
use crate::Token;

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

pub fn parse_import<'s>(parser: &'s mut Parser<'s>) -> Result<Statement, ParseError<'s>> {
    todo!("Parse import")
    /*let path = parser.expect_string()?;
    let namespace = if let Token::KWAs = parser.next_token() {
        if let Token::Namespace(ns) = parser.next_token() {
            Some(vec![ns]) //TODO: return multiple
        } else {
            Err(parser.unexpected_token("Expected namespace"))?
        }
    } else {
        None
    };
    let exposing = if let Token::KWExposing = parser.next_token() {
        if let Token::CurlyOpen = parser.next_token() {
            let mut vec = Vec::new();
            loop {
                match parser.next_token() {
                    Token::VarName(n) => vec.push(n),
                    Token::Comma => continue,
                    Token::CurlyClose => break,
                    Token::EOF => Err(ParseError::UnexpectedEOF)?,
                    _ => Err(parser.unexpected_token("Expected comma seperated list of names"))?,
                }
            }
            Some(vec)
        } else {
            Err(parser.unexpected_token("Expected opening curly brace"))?
        }
    } else {
        None
    };
    Ok(Statement::Import(Import::new(path, namespace, exposing)))*/
}
