use std::borrow::BorrowMut;

use crate::token::{self, Token};

pub struct Lexer<'source> {
    internal: logos::Lexer<'source, Token>,
}
impl <'s> Lexer<'s> {
    pub fn new(internal: logos::Lexer<'s>) -> Self {
        Self {
            internal,//: Box::new(internal),
        }
    }
    fn _next_internal(&mut self) -> Option<Token> {
        let tok = self.internal.borrow_mut().next();
        match tok {
            Some(Token::DoubleQuote) => self.switch::<token::string::DoubleStringToken>(),
            Some(Token::SingleQuote) => self.switch::<token::string::SingleStringToken>(),
            Some(Token::Backtick) => self.switch::<token::string::TemplateLiteralToken>(),
            _ => tok,
        }
    }
    fn switch<T: logos::Logos>(&mut self) {
        self.internal.morph::<T>()
    }
}
impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //self.peekable.pop_front().or_else(|| self._next_internal())
        self._next_internal()
    }
}

fn foo() {
    Token::Comma
}
