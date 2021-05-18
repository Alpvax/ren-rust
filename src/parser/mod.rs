use logos;

//pub mod declaration;
//pub mod expression;
pub mod import;
use crate::{ast, Token};

type NextToken<'s> = (Token<'s>, logos::Span);

enum BuilderResult<T, B, E>
where
    B: Builder<T, E>,
{
    Done(T, Option<B>),
    Continue(B),
    Error(E),
}
impl<T, B, E> BuilderResult<T, B, E>
where
    B: Builder<T, E>,
{
    pub fn finished(self) -> Result<T, E> {
        match self {
            BuilderResult::Done(t, b) => {
                b.map_or_else(|| Ok(t), |b| Err(b._map_unfinished_to_error()))
            }
            BuilderResult::Continue(b) => Err(b._map_unfinished_to_error()),
            BuilderResult::Error(e) => Err(e),
        }
    }
    pub fn builder(self) -> Result<B, ()> {
        if let Self::Continue(b) = self {
            Ok(b)
        } else {
            Err(())
        }
    }
}

//trait Builder<T, E, V = (), SE = ()> {
//    type SubParsers: Builder<V, SE>;
trait Builder<T, E> {
    fn accept_token(self, token: &NextToken) -> BuilderResult<T, Self, E>
    where
        Self: Sized;
    fn accept_value<V>(&self, value: V) -> Result<Self, E>
    where
        Self: Sized;
    fn _map_unfinished_to_error(self) -> E;
}

struct ModuleParser {
    imports_done: bool,
    imports_parser: (),
}
impl ModuleParser {
    fn new() -> Self {
        Self {
            imports_done: false,
            imports_parser: (),
        }
    }
}
impl Builder<ast::Module, ParseError> for ModuleParser {
    fn accept_token(self, token: &NextToken) -> BuilderResult<ast::Module, Self, ParseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn accept_value<V>(&self, value: V) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        todo!("SubParsers")
    }

    fn _map_unfinished_to_error(self) -> ParseError {
        ParseError::UnexpectedEOF
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
}

pub fn parse<'s>(mut lexer: logos::SpannedIter<'s, Token<'s>>) -> Result<ast::Module, ParseError> {
    let mut parser = ModuleParser::new();
    let mut result;
    let mut token = lexer.next().unwrap_or((Token::EOF, logos::Span::default()));
    loop {
        result = parser.accept_token(&token);
        if let (Token::EOF, _) = &token {
            return result.finished();
        }
        match result {
            BuilderResult::Done(v, p) => {
                if let Some(p) = p {
                    parser = p;
                } else {
                    return Ok(v);
                }
            }
            BuilderResult::Continue(p) => {
                parser = p;
                token = lexer.next().unwrap_or((Token::EOF, logos::Span::default()));
            }
            BuilderResult::Error(e) => return Err(e),
        };
    }
}
