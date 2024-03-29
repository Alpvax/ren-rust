use super::*;

// fn check<T: Into<SyntaxPart>>(input: &str, kind: T) {
//     let mut lexer = Lexer::new(input);
//     assert_eq!(lexer.next(), Some((kind.into(), input)))
// }

#[test]
fn simple_string_tokens() {
    let input = "\"Hello world\"";
    for (lexed, expected) in Lexer::new(input).zip([
        (Token::SymDoubleQuote.into(), "\""),
        (StringToken::Text.into(), "Hello world"),
        (StringToken::Delimiter.into(), "\""),
        (TokenType::None, "Should not be reached by zip function"),
    ]) {
        assert_eq!(lexed, expected);
    }
}

#[test]
fn template_string_tokens() {
    let input = "\"Hello ${world}\"";
    for (lexed, expected) in Lexer::new(input).zip(
        [
            (Token::SymDoubleQuote.into(), "\""),
            (StringToken::Text.into(), "Hello "),
            (StringToken::ExprStart.into(), "${"),
            (Token::IdLower.into(), "world"),
            (Token::SymRBrace.into(), "}"),
            (StringToken::Delimiter.into(), "\""),
            (TokenType::None, "Should not be reached by zip function"),
        ]
        .into_iter(),
    ) {
        assert_eq!(lexed, expected);
    }
}

#[test]
fn nested_template_string_tokens() {
    let input = r#""Hello ${"world ${3}"}""#;
    for (lexed, expected) in Lexer::new(input).zip(
        [
            (Token::SymDoubleQuote.into(), "\""),
            (StringToken::Text.into(), "Hello "),
            (StringToken::ExprStart.into(), "${"),
            (Token::SymDoubleQuote.into(), "\""),
            (StringToken::Text.into(), "world "),
            (StringToken::ExprStart.into(), "${"),
            (Token::Number.into(), "3"),
            (Token::SymRBrace.into(), "}"),
            (StringToken::Delimiter.into(), "\""),
            (Token::SymRBrace.into(), "}"),
            (StringToken::Delimiter.into(), "\""),
            (TokenType::None, "Should not be reached by zip function"),
        ]
        .into_iter(),
    ) {
        assert_eq!(lexed, expected);
    }
}

#[test]
fn template_string_with_nested_block_tokens() {
    let input = "\"Hello ${{2+3}/4}\"";
    for (lexed, expected) in Lexer::new(input).zip(
        [
            (Token::SymDoubleQuote.into(), "\""),
            (StringToken::Text.into(), "Hello "),
            (StringToken::ExprStart.into(), "${"),
            (Token::SymLBrace.into(), "{"),
            (Token::Number.into(), "2"),
            (Token::OpAdd.into(), "+"),
            (Token::Number.into(), "3"),
            (Token::SymRBrace.into(), "}"),
            (Token::OpDiv.into(), "/"),
            (Token::Number.into(), "4"),
            (Token::SymRBrace.into(), "}"),
            (StringToken::Delimiter.into(), "\""),
            (TokenType::None, "Should not be reached by zip function"),
        ]
        .into_iter(),
    ) {
        assert_eq!(lexed, expected);
    }
}
