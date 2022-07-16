mod syntax;

use logos::Logos;
use rowan::{Language, SyntaxKind};

use syntax::LexerHolder;
pub(crate) use syntax::{Context, StringToken, SyntaxPart, Token, TokenType};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NestedContext {
    String,
    Expr,
}

pub(crate) struct Lexer<'source> {
    internal: LexerHolder<'source>,
    context: Vec<NestedContext>,
    peeked: Option<(TokenType, &'source str)>,
}
impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            internal: LexerHolder::Main(syntax::token::Token::lexer(input)),
            context: Vec::new(),
            peeked: None,
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
    pub fn peek(&mut self) -> Option<(TokenType, &'source str)> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }
        self.peeked
    }
}
impl<'source> Iterator for Lexer<'source> {
    type Item = (TokenType, &'source str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_some() {
            self.peeked.take()
        } else {
            let res = self.internal.next();
            let parent_ctx = self.context.last();
            if let Some((t, _)) = res {
                match (t, parent_ctx) {
                    (TokenType::Token(Token::DoubleQuote), _) => {
                        self.internal.morph_to_string();
                    }
                    (TokenType::String(StringToken::Delimiter), _) => {
                        self.internal.morph_to_main();
                    }
                    (TokenType::String(StringToken::ExprStart), _) => {
                        self.internal.morph_to_main();
                        self.context.push(NestedContext::String)
                    }
                    (TokenType::Token(Token::CurlyClose), Some(NestedContext::String)) => {
                        self.internal.morph_to_string();
                    }
                    (TokenType::Token(Token::CurlyClose), Some(NestedContext::Expr)) => {
                        self.context.pop();
                    }
                    (TokenType::Token(Token::CurlyOpen), _) => {
                        self.context.push(NestedContext::Expr);
                    }
                    _ => {}
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    // fn check<T: Into<SyntaxPart>>(input: &str, kind: T) {
    //     let mut lexer = Lexer::new(input);
    //     assert_eq!(lexer.next(), Some((kind.into(), input)))
    // }

    #[test]
    fn simple_string_tokens() {
        let input = "\"Hello world\"";
        for (lexed, expected) in Lexer::new(input).zip([
            (Token::DoubleQuote.into(), "\""),
            (StringToken::Text.into(), "Hello world"),
            (StringToken::Delimiter.into(), "\""),
            (TokenType::None, "Should not be reached by zip function"),
        ]) {
            println!("lexed: {:?}; expected: {:?}", lexed, expected);
            assert_eq!(lexed, expected);
        }
        // assert!(lexer.next().is_none());
    }

    #[test]
    fn template_string_tokens() {
        let input = "\"Hello ${world}\"";
        for (lexed, expected) in Lexer::new(input).zip(
            [
                (Token::DoubleQuote.into(), "\""),
                (StringToken::Text.into(), "Hello "),
                (StringToken::ExprStart.into(), "${"),
                (Token::VarName.into(), "world"),
                (Token::CurlyClose.into(), "}"),
                (StringToken::Delimiter.into(), "\""),
                (TokenType::None, "Should not be reached by zip function"),
            ]
            .into_iter(),
        ) {
            println!("lexed: {:?}; expected: {:?}", lexed, expected);
            assert_eq!(lexed, expected);
        }
    }

    #[test]
    fn nested_template_string_tokens() {
        let input = r#""Hello ${"world ${3}"}""#;
        for (lexed, expected) in Lexer::new(input).zip(
            [
                (Token::DoubleQuote.into(), "\""),
                (StringToken::Text.into(), "Hello "),
                (StringToken::ExprStart.into(), "${"),
                (Token::DoubleQuote.into(), "\""),
                (StringToken::Text.into(), "world "),
                (StringToken::ExprStart.into(), "${"),
                (Token::Number.into(), "3"),
                (Token::CurlyClose.into(), "}"),
                (StringToken::Delimiter.into(), "\""),
                (Token::CurlyClose.into(), "}"),
                (StringToken::Delimiter.into(), "\""),
                (TokenType::None, "Should not be reached by zip function"),
            ]
            .into_iter(),
        ) {
            println!("lexed: {:?}; expected: {:?}", lexed, expected);
            assert_eq!(lexed, expected);
        }
    }

    #[test]
    fn template_string_with_nested_block_tokens() {
        let input = "\"Hello ${{2+3}/4}\"";
        for (lexed, expected) in Lexer::new(input).zip(
            [
                (Token::DoubleQuote.into(), "\""),
                (StringToken::Text.into(), "Hello "),
                (StringToken::ExprStart.into(), "${"),
                (Token::CurlyOpen.into(), "{"),
                (Token::Number.into(), "2"),
                (Token::OpAdd.into(), "+"),
                (Token::Number.into(), "3"),
                (Token::CurlyClose.into(), "}"),
                (Token::OpDiv.into(), "/"),
                (Token::Number.into(), "4"),
                (Token::CurlyClose.into(), "}"),
                (StringToken::Delimiter.into(), "\""),
                (TokenType::None, "Should not be reached by zip function"),
            ]
            .into_iter(),
        ) {
            println!("lexed: {:?}; expected: {:?}", lexed, expected);
            assert_eq!(lexed, expected);
        }
    }
}
