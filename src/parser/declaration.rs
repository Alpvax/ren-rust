use crate::ast::declaration::Declaration;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TODO(bool),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TODO(public) => {
                write!(f, "TODO: Actually implement this parser. pub = {}", public)
            }
        }
    }
}

pub fn parse_pub_declaration(lexer: &mut Lexer) -> Result<Declaration, Error> {
    lexer.next(); //consume pub
    parse_declaration(lexer).map_err(|_| Error::TODO(true))
}
pub fn parse_declaration(lexer: &mut Lexer) -> Result<Declaration, Error> {
    if let Some(Token::KWLet) = lexer.next_token() {}
    Err(Error::TODO(false))
}
