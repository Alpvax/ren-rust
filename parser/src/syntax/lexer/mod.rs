mod token;

use logos::Logos;
pub(crate) use token::{StringToken, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenType {
    Token(Token),
    String(StringToken),
    None,
}
impl From<Token> for TokenType {
    fn from(tok: Token) -> Self {
        Self::Token(tok)
    }
}
impl From<StringToken> for TokenType {
    fn from(tok: StringToken) -> Self {
        Self::String(tok)
    }
}
impl TokenType {
    pub fn is<T: Into<Self>>(&self, token: T) -> bool {
        self == &token.into()
    }
}

pub(crate) type Lexeme<'source> = (TokenType, &'source str);

enum LexerHolder<'source> {
    Main(logos::Lexer<'source, Token>),
    String(logos::Lexer<'source, StringToken>),
    /// Should only be used when morphing in order to take the lexer instance
    None,
}
impl<'source> LexerHolder<'source> {
    pub(super) fn morph(&mut self) {
        let prev = std::mem::replace(self, Self::None);
        *self = match prev {
            Self::Main(lex) => Self::String(lex.morph()),
            Self::String(lex) => Self::Main(lex.morph()),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        };
    }
    pub(super) fn morph_to_string(&mut self) {
        if let Self::Main(_) = self {
            self.morph();
        }
    }
    pub(super) fn morph_to_main(&mut self) {
        if let Self::String(_) = self {
            self.morph();
        }
    }
}
impl<'source> Iterator for LexerHolder<'source> {
    type Item = Result<Lexeme<'source>, &'source str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LexerHolder::Main(lex) => lex.next().map(|r| {
                r.map(|t| (TokenType::Token(t), lex.slice()))
                    .map_err(|_| lex.slice())
            }),
            LexerHolder::String(lex) => lex.next().map(|r| {
                r.map(|t| (TokenType::String(t), lex.slice()))
                    .map_err(|_| lex.slice())
            }),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NestedContext {
    String,
    Expr,
}

pub(crate) struct Lexer<'source> {
    internal: LexerHolder<'source>,
    context: Vec<NestedContext>,
    peeked: Option<Lexeme<'source>>,
    lex_errors: Vec<&'source str>,
}
impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            internal: LexerHolder::Main(Token::lexer(input)),
            context: Vec::new(),
            peeked: None,
            lex_errors: Vec::new(),
        }
    }
    // pub fn slice(&self) -> &'source str {
    //     self.internal.slice()
    // }
    // pub fn is_string_token(&self) -> bool {
    //     match self.internal {
    //         LexerHolder::String(_) => true,
    //         _ => false,
    //     }
    // }
    pub fn peek(&mut self) -> Option<Lexeme<'source>> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }
        self.peeked
    }
}
impl<'source> Iterator for Lexer<'source> {
    type Item = Lexeme<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_some() {
            self.peeked.take()
        } else {
            loop {
                match self.internal.next() {
                    Some(Err(s)) => {
                        self.lex_errors.push(s);
                    }
                    Some(Ok(res @ (t, _))) => {
                        match (t, self.context.last()) {
                            (TokenType::Token(Token::SymDoubleQuote), _) => {
                                self.internal.morph_to_string();
                            }
                            (TokenType::String(StringToken::Delimiter), _) => {
                                self.internal.morph_to_main();
                            }
                            (TokenType::String(StringToken::ExprStart), _) => {
                                self.internal.morph_to_main();
                                self.context.push(NestedContext::String)
                            }
                            (TokenType::Token(Token::SymRBrace), Some(NestedContext::String)) => {
                                self.internal.morph_to_string();
                            }
                            (TokenType::Token(Token::SymRBrace), Some(NestedContext::Expr)) => {
                                self.context.pop();
                            }
                            (TokenType::Token(Token::SymLBrace), _) => {
                                self.context.push(NestedContext::Expr);
                            }
                            _ => {}
                        }
                        break Some(res);
                    }
                    None => break None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
