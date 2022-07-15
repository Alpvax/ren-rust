mod syntax;

use logos::Logos;
use rowan::{Language, SyntaxKind};

use syntax::LexerHolder;
pub(crate) use syntax::{Context, StringToken, SyntaxPart, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RenLang;
impl Language for RenLang {
    type Kind = syntax::SyntaxPart;

    fn kind_from_raw(raw: SyntaxKind) -> Self::Kind {
        Self::Kind::try_from(raw.0).expect("Failed converting rowan::SyntaxKind to SyntaxPart!")
    }

    fn kind_to_raw(kind: Self::Kind) -> SyntaxKind {
        SyntaxKind(kind.into())
    }
}

pub(crate) type SyntaxNode = rowan::SyntaxNode<RenLang>;

pub(crate) struct Lexer<'source> {
    internal: LexerHolder<'source>,
    peeked: Option<(SyntaxPart, &'source str)>,
}
impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            internal: LexerHolder::Main(syntax::token::Token::lexer(input)),
            peeked: None,
        }
    }
    pub fn slice(&self) -> &'source str {
        self.internal.slice()
    }
    pub fn is_string_token(&self) -> bool {
        match self.internal {
            LexerHolder::String(_) => true,
            _ => false,
        }
    }
    pub fn peek(&mut self) -> Option<(SyntaxPart, &'source str)> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }
        self.peeked
    }
}
impl<'source> Iterator for Lexer<'source> {
    type Item = (SyntaxPart, &'source str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_some() {
            self.peeked.take()
        } else {
            let res = self.internal.next();
            if let Some((sp, _)) = res {
                if sp == SyntaxPart::RawToken(Token::DoubleQuote)
                    || sp == SyntaxPart::StringToken(StringToken::Delimiter)
                {
                    println!("Morphing");//XXX
                    self.internal.morph();
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn check<T: Into<SyntaxPart>>(input: &str, kind: T) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((kind.into(), input)))
    }

    #[test]
    fn string_tokens() {
        let input = "\"Hello world\"";
        let mut lexer = Lexer::new(input);
        for (lexed, expected) in lexer.zip(
            [
                (Token::DoubleQuote.into(), "\""),
                (StringToken::Text.into(), "Hello world"),
                (StringToken::Delimiter.into(), "\""),
                (SyntaxPart::Error, "Should not be reached"),
            ]
            .into_iter(),
        ) {
            println!("lexed: {:?}; expected: {:?}", lexed, expected);
            assert_eq!(lexed, expected);
        }
        // assert!(lexer.next().is_none());
    }
}
