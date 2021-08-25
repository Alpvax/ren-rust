#![allow(dead_code)]//XXX
use crate::Token;

use super::{ContextNext, PartialParse, PartialResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    SingleQuote,
    DoubleQuote,
    Template,
}

enum StringSegment {
    Text(String),
    Escape(char),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringContext {
    /// Inside ' " or `
    Start(StringType),
}

#[derive(Debug)]
pub struct StringTokenError;

impl PartialParse for StringContext {
    type Partial = Self;
    type Complete = String;
    type Error = StringTokenError;

    fn is_valid(&self) -> bool {
        false
    }

    fn parse_next<'s>(self, _token: Token, _ctxnext: &mut ContextNext<'s>) -> PartialResult<Self::Partial, Self::Complete, Self::Error> {
        todo!()
    }

    // fn on_enter<T>(parent: dyn PartialParse<T>, lexer: &mut lexer::Lexer<T>) {
    //     *lexer = lexer.morph::<Self::Token>();
    // }

    // fn on_exit<T>(result: PartialResult<Self::Partial, Self::Complete, Self::Error>, lexer: &mut lexer::Lexer<Self::Token>) {
    //     *lexer = lexer.morph::<lexer::Token>();
    // }
}
