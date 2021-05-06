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

pub struct Parser<'s> {
    lexer: &'s Lexer<'s, Token<'s>>,
    statements: Vec<Statement>,
    current: Token<'s>,
    branch: Option<Branch<'s>>,
}
impl<'s> Parser<'s> {
    pub fn new(mut lexer: logos::Lexer<'s, Token<'s>>) -> Parser<'s> {
        Parser {
            lexer: &lexer,
            statements: Vec::new(),
            current: lexer.next().unwrap_or(Token::EOF),
            branch: None,
        }
    }
    fn next_token(&mut self) -> Token<'s> {
        self.next();
        self.current
    }
    pub fn branch(&mut self, length: usize) -> Result<Vec<Token<'s>>, ParseError> {
        let res = Vec::new();
        for i in 0..length {
            if let Some(tok) = self.next() {
                res.push(tok);
            } else {
                return Err(ParseError::UnexpectedTokens(res, length));
            }
        }
        Ok(res)
    }
    pub fn expect_string(&mut self) -> Result<&'s str, Token<'s>> {
        match self.next_token() {
            Token::StrDbl(s) | Token::StrSingle(s) => Ok(s),
            _ => Err(self.current),
        }
    }
    pub fn expect_sequence(&mut self, tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>> {
        todo!("Implement sequence")
    }
}

impl<'s> Iterator for Parser<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.lexer.next();
        self.current = opt.unwrap_or(Token::EOF);
        opt
    }
}

enum TokenNode {

}

pub struct Branch<'s> {
    parser: &'s Parser<'s>,
    read: Vec<Token<'s>>,
}
impl <'s> Branch<'s> {
    pub fn new(parser: &'s mut Parser<'s>) -> Branch<'s> {
        Branch {
            parser,
            read: Vec::new(),
        }
    }
    pub fn success(self) {

    }
}

pub fn parse<'s>(mut lexer: logos::Lexer<'s, Token<'s>>) -> Result<Vec<Statement>, ParseError> {
    let parser = Parser::new(lexer);
    loop {
        match parser.next_token() {
            Token::EOF => break,
            Token::KWImport => {
                import::parse_import(&mut parser);
            }
            Token::KWFun => {
                todo!("Parse 'fun' toplevel")
            }
            Token::KWLet => {
                todo!("Parse 'let' toplevel")
            }
            _ => Err(ParseError::InvalidTopLevel(parser.current))?,
        }
    }
    Ok(parser.statements)
}
