use crate::token::{self, Token, LexerExtras};

#[derive(Debug, Clone)]
pub struct TokenPointer<'source, T> where T: logos::Logos<'source> {
    token: T,
    span: logos::Span,
    consumed: bool,
    _phantom: &'source std::marker::PhantomData<()>,
}
impl <'source, T>TokenPointer<'source, T> where T: logos::Logos<'source> {
    pub fn consume(mut self) -> T {
        self.consumed = true;
        self.token
    }
    pub fn consume_and_return<R>(&mut self, ret: R) -> R {
        self.consumed = true;
        ret
    }
    pub fn get(&self) -> &T {
        &self.token
    }
    pub fn span(&mut self) -> &logos::Span {
        &self.span
    }
    fn new(token: T, span: logos::Span) -> Self {
        Self {
            token,
            span,
            consumed: false,
            _phantom: &std::marker::PhantomData,
        }
    }
}
impl <'source, T>TokenPointer<'source, T> where T: logos::Logos<'source> + Clone {
    pub fn clone_token(&self) -> T {
        self.token.clone()
    }
}

pub struct Lexer<'source, T>
where T: logos::Logos<'source> {
    internal: Option<logos::Lexer<'source, T>>,
    current: Option<TokenPointer<'source, T>>,
    skip_predicate: fn(&T) -> bool,
}
impl <'source> Lexer<'source, Token> {
    pub fn skip_whitespace(&mut self, skip: bool) {
        self.skip_predicate = if skip { |tok| tok == &Token::Whitespace } else { |_| false };
    }
}
impl <'source> Lexer<'source, Token> {
    pub fn new(input: &'source str) -> Self {
        Self {
            internal: Some(logos::Lexer::new(input)),
            current: None,
            skip_predicate: |_| false,
        }
    }
    pub fn skip_predicate(&mut self, predicate: fn(&Token) -> bool) {
        self.skip_predicate = predicate;
    }
    pub fn read<'l>(&'l mut self) -> Option<&'l mut TokenPointer<'source, Token>> {
        match self.current {
            Some(TokenPointer { consumed: true, .. }) | None => self.current = self._next_internal().map(|tok| TokenPointer::new(tok, self.internal.as_ref().unwrap().span())),
            _ => (),
        }
        self.current.as_mut()
    }
    fn _next_internal(&mut self) -> Option<Token> {
        use token::string::{StringSegment, StringType};
        let tok = match self.internal.as_mut().unwrap().next() {
            Some(Token::DoubleQuote) => {
                use token::string::DoubleStringToken::*;
                let mut str_lexer = self.switch::<token::string::DoubleStringToken>();
                let mut text = String::new();
                let res = loop {
                    match str_lexer.next() {
                        Some(Error) => break Some(Token::Error),
                        Some(Escape(c)) => {
                            text.push('\\');
                            text.push(c);
                        }
                        Some(Delimiter) => break Some(Token::StringSegment(StringSegment::Full(text, StringType::Double))),
                        Some(Text(t)) => text.push_str(&t),
                        None => break Some(Token::Error),
                    }
                };
                self.internal = Some(str_lexer.revert());
                res
            },
            Some(Token::SingleQuote) => {
                use token::string::SingleStringToken::*;
                let mut str_lexer = self.switch::<token::string::SingleStringToken>();
                let mut text = String::new();
                let res = loop {
                    match str_lexer.next() {
                        Some(Error) => break Some(Token::Error),
                        Some(Escape(c)) => {
                            text.push('\\');
                            text.push(c);
                        }
                        Some(Delimiter) => break Some(Token::StringSegment(StringSegment::Full(text, StringType::Single))),
                        Some(Text(t)) => text.push_str(&t),
                        None => break Some(Token::Error),
                    }
                };
                self.internal = Some(str_lexer.revert());
                res
            },
            Some(Token::Backtick) => {
                use token::string::TemplateLiteralToken::*;
                let mut str_lexer = self.switch::<token::string::TemplateLiteralToken>();
                let mut text = String::new();
                let res = loop {
                    match str_lexer.next() {
                        Some(Error) => break Some(Token::Error),
                        Some(Escape(c)) => {
                            text.push('\\');
                            text.push(c);
                        }
                        Some(Delimiter) => break Some(Token::StringSegment(StringSegment::Full(text, StringType::Backtick))), //TODO: end
                        Some(Text(t)) => text.push_str(&t),
                        Some(ExprStart) => break Some(Token::StringSegment(StringSegment::Start(text, StringType::Backtick))), //TODO: mid
                        None => break Some(Token::Error),
                    }
                };
                self.internal = Some(str_lexer.revert());
                res
            },
            tok => tok,
        };
        if let Some(t) = &tok {
            if (self.skip_predicate)(t) {
                return self._next_internal();
            }
        }
        tok
    }
    fn switch<T>(&mut self) -> SwitchedLexer<'source, T>
    where T: logos::Logos<'source, Source = str, Extras = LexerExtras> {
        SwitchedLexer(self.internal.take().unwrap().morph::<T>())
    }
}

pub struct SwitchedLexer<'source, T>(logos::Lexer<'source, T>) where T: logos::Logos<'source, Source = str, Extras = LexerExtras>;
impl <'source, T> SwitchedLexer<'source, T> where T: logos::Logos<'source, Source = str, Extras = LexerExtras> {
    pub fn revert(self) -> logos::Lexer<'source, Token> {
        self.0.morph::<Token>()
    }
}
impl <'source, T> Iterator for SwitchedLexer<'source, T> where T: logos::Logos<'source, Source = str, Extras = LexerExtras> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
