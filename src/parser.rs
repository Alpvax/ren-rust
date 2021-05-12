use logos::Lexer;

use crate::{import, Token};

/**
 * Top-level statement
 */
#[derive(Debug)]
pub enum Statement {
    Import(import::Import),
    Function(),
    Variable(),
    EOF,
}

#[derive(Debug)]
pub enum ParseError<'s> {
    InvalidTopLevel(Token<'s>),
    UnexpectedEOF,
    UnexpectedToken(Token<'s>, String),
    UnexpectedTokens(Vec<Token<'s>>, usize),
}
impl<'s> From<Option<Token<'s>>> for ParseError<'s> {
    fn from(opt: Option<Token<'s>>) -> Self {
        if opt.is_none() {
            Self::UnexpectedEOF
        } else {
            panic!("Trying to convert a valid token to an error!")
        }
    }
}
impl<'s> From<Token<'s>> for ParseError<'s> {
    fn from(t: Token<'s>) -> ParseError<'s> {
        ParseError::UnexpectedToken(t, "Expected something else".to_owned())
    }
}
/*impl<'s> From<Result<&'s str, &'s Token<'s>>> for ParseError<'s> {
    fn from(r: Result<&'s str, &'s Token<'s>>) -> Self {
        if let Err(t) = r {
            ParseError::UnexpectedToken(
                t,
                "Expected string".to_owned(),
            )
        } else {
            panic!("Trying to convert a valid token to an error!")
        }
    }
}*/
impl<'s, T> From<Result<T, Token<'s>>> for ParseError<'s> {
    fn from(r: Result<T, Token<'s>>) -> ParseError<'s> {
        if let Err(t) = r {
            ParseError::UnexpectedToken(t, "Expected something else".to_owned())
        } else {
            panic!("Trying to convert a valid token to an error!")
        }
    }
}
/*impl<'s> From<Result<&str, &'s Token<'s>>> for ParseError<'s> {
    fn from(r: Result<&str, &'s Token>) -> Self {
        if let Err(t) = r {
            ParseError::UnexpectedToken(
                t,
                "Expected string".to_owned(),
            )
        } else {
            panic!("Trying to convert a valid token to an error!")
        }
    }
}*/

pub struct Parser<'s> {
    lexer: Lexer<'s, Token<'s>>,
    statements: Vec<Statement>,
    pub current: Token<'s>,
    branch: Option<Branch<'s>>,
}
impl<'s> Parser<'s> {
    pub fn new(mut lexer: logos::Lexer<'s, Token<'s>>) -> Parser<'s> {
        Parser {
            lexer,
            statements: Vec::new(),
            current: Token::EOF,
            branch: None,
        }
    }
    pub fn next_token(&mut self) -> Token<'s> {
        self.next();
        self.current
    }
    pub fn branch(&mut self, length: usize) -> Result<Vec<Token<'s>>, ParseError> {
        let mut res = Vec::new();
        for _ in 0..length {
            if let Some(tok) = self.next() {
                res.push(tok);
            } else {
                return Err(ParseError::UnexpectedTokens(res, length));
            }
        }
        Ok(res)
    }
    pub fn expect_string(&'s mut self) -> Result<&'s str, Token<'s>> {
        match self.next_token() {
            Token::StrDbl(s) | Token::StrSingle(s) => Ok(s),
            _ => Err(self.current),
        }
    }
    pub fn expect_sequence(&mut self, _tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>> {
        todo!("Implement sequence")
    }
    pub fn unexpected_token(&self, message: &str) -> ParseError {
        ParseError::UnexpectedToken(self.current, message.to_owned())
    }
}

impl<'s> Iterator for Parser<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.lexer.next() {
            self.current = t;
            Some(self.current)
        } else {
            self.current = Token::EOF;
            None
        }
    }
}

enum TokenNode {}

pub struct Branch<'s> {
    parser: &'s Parser<'s>,
    read: Vec<Token<'s>>,
}
impl<'s> Branch<'s> {
    pub fn new(parser: &'s mut Parser<'s>) -> Branch<'s> {
        Branch {
            parser,
            read: Vec::new(),
        }
    }
    pub fn success(self) {}
}

pub fn parse<'s>(lexer: logos::Lexer<'s, Token<'s>>) -> Result<Vec<Statement>, ParseError> {
    let parser = Parser::new(lexer);
    for tok in parser {
        println!("{:?}", tok);
        /*match tok {
            Token::EOF => break,
            Token::KWImport => {
                todo!("Parse 'import' toplevel (parser.branch)")//import::parse_import(parser.branch(10));
            }
            Token::KWFun => {
                todo!("Parse 'fun' toplevel")
            }
            Token::KWLet => {
                todo!("Parse 'let' toplevel")
            }
            _ => Err(ParseError::InvalidTopLevel(tok))?,
        }*/
    }
    Ok(Vec::new()) //Ok(parser.statements)
}
