use super::{Lexer, StringToken, Token, TokenType};

mod string;

// fn check<T: Into<SyntaxPart>>(input: &str, kind: T) {
//     let mut lexer = Lexer::new(input);
//     assert_eq!(lexer.next(), Some((kind.into(), input)))
// }

#[test]
fn lambda() {
    let input = "fun name => if is_vowel (Str.take 1 name) then name else Str.drop 1 name";
    for (lexed, expected) in Lexer::new(input).zip([
        (Token::KWFun.into(), "fun"),
        (Token::Whitespace.into(), " "),
        (Token::VarName.into(), "name"),
        (Token::Whitespace.into(), " "),
        (Token::OpArrow.into(), "=>"),
        (Token::Whitespace.into(), " "),
        (Token::KWIf.into(), "if"),
        (Token::Whitespace.into(), " "),
        (Token::VarName.into(), "is_vowel"),
        (Token::Whitespace.into(), " "),
        (Token::ParenOpen.into(), "("),
        (Token::Namespace.into(), "Str"),
        (Token::Period.into(), "."),
        (Token::VarName.into(), "take"),
        (Token::Whitespace.into(), " "),
        (Token::Number.into(), "1"),
        (Token::Whitespace.into(), " "),
        (Token::VarName.into(), "name"),
        (Token::ParenClose.into(), ")"),
        (Token::Whitespace.into(), " "),
        (Token::KWThen.into(), "then"),
        (Token::Whitespace.into(), " "),
        (Token::VarName.into(), "name"),
        (Token::Whitespace.into(), " "),
        (Token::KWElse.into(), "else"),
        (Token::Whitespace.into(), " "),
        (Token::Namespace.into(), "Str"),
        (Token::Period.into(), "."),
        (Token::VarName.into(), "drop"),
        (Token::Whitespace.into(), " "),
        (Token::Number.into(), "1"),
        (Token::Whitespace.into(), " "),
        (Token::VarName.into(), "name"),
        (TokenType::None, "Should not be reached by zip function"),
    ]) {
        assert_eq!(lexed, expected);
    }
}
