use logos::{Span, SpannedIter};
mod lexer;

use crate::ast;
pub use lexer::Token;

macro_rules! parent_parser {
    ($p:ty, $o:ty, $e:ty, {$u:item $p_e: item}) => {
        impl Parser for $p {
            type Output = $o;
            type Error = $e;
            fn try_parse_token(&mut self, token: crate::parser::SpannedToken) -> crate::parser::TokenParseResult {
                if let Some(s) = self.get_subparser() {
                    s.try_parse_token(token)
                } else {
                    let (r, c) = self.parse_token(token);
                    let fallback = if c { crate::parser::TokenParseResult::Continue } else { crate::parser::TokenParseResult::Retry };
                    match r {
                        crate::parser::ParentParserTokenResult::Done => crate::parser::TokenParseResult::Done,
                        crate::parser::ParentParserTokenResult::Continue => crate::parser::TokenParseResult::Continue,
                        crate::parser::ParentParserTokenResult::SubParser(s) => {
                            self.start_subparser(s);
                            fallback
                        },
                        crate::parser::ParentParserTokenResult::Error(e) => {
                            self.push_error(e);
                            crate::parser::TokenParseResult::Continue
                        },
                    }
                }
            }
            $u
            $p_e
        }
    };
}

pub mod module;

pub type SpannedToken<'s> = (crate::Token<'s>, Span);

pub type ParserResult<T, E> = Result<T, Vec<ParserError<E>>>;
pub type ParserError<E> = (E, Span);

pub enum TokenParseResult {
    Done,
    Continue,
    Retry,
}

pub trait Parser {
    type Output;
    type Error;
    fn try_parse_token(&mut self, token: SpannedToken) -> TokenParseResult;
    fn unwrap(self) -> Result<Self::Output, Vec<Self::Error>>;
    fn push_error(&mut self, error: Self::Error);
}

pub enum SimpleParserTokenResult<E> {
    Done,
    Continue,
    Error(E),
}

pub trait SimpleParser: Parser {
    fn parse_token(&mut self, token: SpannedToken) -> (SimpleParserTokenResult<Self::Error>, bool);
    fn try_parse_token(&mut self, token: SpannedToken) -> TokenParseResult {
        match self.parse_token(token) {
            (SimpleParserTokenResult::Done, _) => TokenParseResult::Done,
            (_, true) => TokenParseResult::Continue,
            (_, false) => TokenParseResult::Retry,
        }
    }
}

pub enum ParentParserTokenResult<E, S> {
    Done,
    Continue,
    SubParser(S),
    Error(E),
}
impl<SE, PE, S> From<ParentParserTokenResult<PE, S>> for SimpleParserTokenResult<SE>
where
    PE: Into<SE>,
{
    fn from(r: ParentParserTokenResult<PE, S>) -> Self {
        match r {
            ParentParserTokenResult::Done => Self::Done,
            ParentParserTokenResult::Continue | ParentParserTokenResult::SubParser(_) => {
                Self::Continue
            }
            ParentParserTokenResult::Error(e) => Self::Error(e.into()),
        }
    }
}

pub trait ParentParser: Parser {
    type SubParser: Parser;
    //type Error: From<<<Self as ParentParser>::SubParser as Parser>::Error>;
    fn get_subparser(&mut self) -> Option<&mut Self::SubParser>;
    fn start_subparser(&mut self, sub: Self::SubParser);
    fn parse_token(
        &mut self,
        token: SpannedToken,
    ) -> (ParentParserTokenResult<Self::Error, Self::SubParser>, bool);
    /*fn try_parse_token(&mut self, token: Token) -> TokenParseResult {
        if let Some(s) = self.get_subparser() {
            s.try_parse_token(token)
        } else {
            let (r, c) = self.parse_token(token);
            let fallback = if c { TokenParseResult::Continue } else { TokenParseResult::Retry };
            match r {
                ParentParserTokenResult::Done => TokenParseResult::Done,
                ParentParserTokenResult::Continue => TokenParseResult::Continue,
                ParentParserTokenResult::SubParser(s) => {
                    self.start_subparser(s);
                    fallback
                },
                ParentParserTokenResult::Error(e) => {
                    self.push_error(e);
                    TokenParseResult::Continue
                },
            }
        }
    }*/
}

pub fn parse<'s>(
    mut lexer: SpannedIter<'s, Token<'s>>,
) -> ParserResult<ast::Module, module::ModuleParseErr> {
    let mut parser = module::ModuleParser::new();
    let mut token = lexer.next().unwrap_or((Token::EOF, Span::default()));
    loop {
        match parser.try_parse_token(token.clone()) {
            TokenParseResult::Done => break,
            TokenParseResult::Continue => {
                token = lexer.next().unwrap_or((Token::EOF, Span::default()));
            }
            TokenParseResult::Retry => {}
        }
    }
    parser.unwrap()
}
