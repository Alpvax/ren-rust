#![cfg(test)]

use super::*;

mod token {
    use super::Token::{self, *};

    fn to_tokens(input: &str, keep_whitespace: bool) -> Vec<Token> {
        use logos::Logos;
        Token::lexer(input)
            .filter(|t| keep_whitespace || t != &Whitespace)
            .collect()
    }
    #[test]
    fn number_tokens() {
        use Number as N;
        assert_eq!(
            to_tokens(
                "0 0.1 .2 341 14.682 1.3e2 3.4e-3 0xa0 0o14 0b100 0XfF 0O30 0B1010",
                false
            ),
            vec![
                N(0.0),
                N(0.1),
                N(0.2),
                N(341.0),
                N(14.682),
                N(1.3e2),
                N(3.4e-3),
                N(160.0),
                N(12.0),
                N(4.0),
                N(255.0),
                N(24.0),
                N(10.0)
            ],
        )
    }
}
