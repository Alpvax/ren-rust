use crate::lexer::{SyntaxPart, Token};

use super::Parser;

pub(super) fn expr(p: &mut Parser) {
    match p.peek() {
        SyntaxPart::Token(Token::Number) | SyntaxPart::Token(Token::VarName) => p.bump(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::{expect};

    #[test]
    fn parse_number() {
        check("143", expect![[r#"
        Context(Module)@0..3
          Token(Number)@0..3 "143""#]])
    }

    #[test]
    fn parse_varname() {
        check("varName1", expect![[r#"
        Context(Module)@0..8
          Token(VarName)@0..8 "varName1""#]])
    }
}