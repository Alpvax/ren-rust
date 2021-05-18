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

trait SubParser<V, P> {
    fn new_subparser(self) -> P;
}

//trait Builder<T, E, SE = ()> {
//    type SubValues;
//    type SubParsers: Builder<V, SE>;
trait Builder<T, E> {
    type SubValues;
    fn accept_token(self, token: &NextToken) -> BuilderResult<T, Self, E>
    where
        Self: Sized;
    fn accept_value<V>(self, value: Self::SubValues) -> Result<Self, E>
    where
        Self: Sized;
    fn _map_unfinished_to_error(self) -> E;
}

enum ModuleValues {
    Import(ast::import::Import),
    Declaration(ast::declaration::Declaration),
}

struct ModuleParser {
    imports_done: bool,
    imports: Vec<ast::import::Import>,
    declarations: Vec<ast::declaration::Declaration>,
}
impl ModuleParser {
    fn new() -> Self {
        Self {
            imports_done: false,
            imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
}
impl Builder<ast::Module, ParseError> for ModuleParser {
    type SubValues = ModuleValues;

    fn accept_token(self, _token: &NextToken) -> BuilderResult<ast::Module, Self, ParseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn accept_value<V>(mut self, value: Self::SubValues) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        match value {
            ModuleValues::Import(i) => {
                if self.imports_done {
                    return Err(ParseError::ImportBelowDeclarations);
                } else {
                    self.imports.push(i);
                }
            }
            ModuleValues::Declaration(d) => {
                self.imports_done = true;
                self.declarations.push(d);
            }
        };
        Ok(self)
    }

    fn _map_unfinished_to_error(self) -> ParseError {
        ParseError::UnexpectedEOF
    }
}

#[derive(Debug)]
pub enum ParseError {
    ImportBelowDeclarations,
    UnexpectedEOF,
}

pub fn parse<'s>(mut lexer: logos::SpannedIter<'s, Token<'s>>) -> Result<ast::Module, ParseError> {
    let mut parser = ModuleParser::new();
    let mut token = lexer.next().unwrap_or((Token::EOF, logos::Span::default()));
    loop {
        match parser.accept_token(&token) {
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
