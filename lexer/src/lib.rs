pub mod token;
//mod lexer;

pub use logos::{Lexer, Logos, Span};

pub use token::Token;
#[cfg(test)]
mod test;

pub fn lexer<'source>(source: &'source str) -> Lexer<'source, Token> {
    Token::lexer(source)
}
