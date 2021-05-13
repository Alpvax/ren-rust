use logos;

pub mod import;
use crate::Token;

/**
 * Top-level statement
 */
#[derive(Debug)]
pub enum Statement {
    Import(import::Import),
    Function(),
    Variable(),
}

type ParseError = ();

pub fn parse<'s>(mut _lexer: logos::Lexer<'s, Token<'s>>) -> Result<Vec<Statement>, ParseError> {
    todo!("Parse tokens")
}
