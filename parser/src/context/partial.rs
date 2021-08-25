use std::mem;

use crate::{Token, string::{QuoteType, parse_string}};

pub(crate) struct ContextNext<'source> {
    pub(crate) lexer: &'source mut lexer::Lexer<'source, Token>,
}
impl<'s> ContextNext<'s> {
    pub fn parse_string(&mut self, q_type: QuoteType) -> Result<String, Vec<(lexer::Span, &str)>> {
        let lexer = mem::replace(*&mut self.lexer, lexer::lexer(""));
        let (lex, r) = parse_string(lexer, q_type);
        *self.lexer = lex;
        r
    }
    pub fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }
}

pub(crate) trait PartiallyValid {
    fn is_valid(&self) -> bool;
}
pub(crate) trait PartialParse {
    //<Token = super::Token> {
    type Partial: PartiallyValid;
    type Complete;
    type Error;
    fn is_valid(&self) -> bool;
    fn parse_next<'source>(self, token: Token, ctx: &mut ContextNext<'source>) -> PartialResult<Self::Partial, Self::Complete, Self::Error>;
    // fn on_token(self, token: Token) -> PartialResult<Self::Partial, Self::Complete, Self::Error>;
    // fn on_enter<'s, T: lexer::Logos<'s>, P, C, E>(parent: dyn PartialParse<T, Partial = P, Complete = C, Error = E>, lexer: &mut lexer::Lexer<T>) {}
    // fn on_exit<T>(result: PartialResult<Self::Partial, Self::Complete, Self::Error>, lexer: &mut lexer::Lexer<Token>) {}
}
impl<T> PartiallyValid for T
where
    T: PartialParse,
{
    fn is_valid(&self) -> bool {
        self.is_valid()
    }
}
// pub(crate) trait PartialParseCustom {
//     type Token: lexer::Logos;
//     type Partial: PartiallyValid;
//     type Complete;
//     type Error;
//     fn is_valid(&self) -> bool;
//     fn on_token(self, token: Self::Token) -> PartialResult<Self::Partial, Self::Complete, Self::Error>;
//     fn mutate_lexer<T>(lexer: &mut lexer::Lexer<T>) -> lexer::Lexer<Self::Token>;
// }
// impl<T> PartiallyValid for T
// where
//     T: PartialParseCustom,
// {
//     fn is_valid(&self) -> bool {
//         self.is_valid()
//     }
// }

#[derive(Debug, PartialEq)]
pub(crate) enum PartialResult<P, R, E>
where
    P: PartiallyValid,
{
    Partial(P),
    Complete(R, Option<Token>),
    Err(E),
}
impl<P, R, E> PartialResult<P, R, E>
where
    P: PartiallyValid,
{
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        match self {
            PartialResult::Partial(ctx) => ctx.is_valid(),
            PartialResult::Complete(..) => true,
            PartialResult::Err(_) => false,
        }
    }
}
