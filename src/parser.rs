use logos::Lexer;

use crate::import::Import;
use crate::Token;

/**
 * Top-level statement
 */
#[derive(Debug)]
pub enum Statement {
    Import(Import),
    Function(),
    Variable(),
}

type ParseError = ();

pub fn parse<'s>(mut lexer: logos::Lexer<'s, Token<'s>>) -> Result<Vec<Statement>, ParseError> {
    todo!("Parse tokens")
}
