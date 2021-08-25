use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    Single,
    Double,
}

pub fn parse_string<'s>(
    lexer: lexer::Lexer<'s, Token>,
    q_type: QuoteType,
) -> (
    lexer::Lexer<'s, Token>,
    Result<String, Vec<(lexer::Span, &'s str)>>,
) {
    match q_type {
        QuoteType::Single => {
            use lexer::token::string::SingleStringToken as T;
            let mut str_lex: lexer::Lexer<'s, T> = lexer.morph();
            let mut text = String::new();
            let mut errors = Vec::new();
            while let Some(tok) = str_lex.next() {
                match tok {
                    T::Delimiter => {
                        break;
                    }
                    T::Error => {
                        errors.push((str_lex.span(), str_lex.slice()));
                    }
                    T::Escape(c) => {
                        text.push_str(match c {
                            '\'' => "'",
                            '\\' => r"\",
                            'n' => "\n",
                            'r' => "\r",
                            't' => "\t",
                            _ => unreachable!("Invalid escape"),
                        });
                    }
                    T::Text(t) => {
                        text.push_str(&t);
                    }
                }
            }
            (
                str_lex.morph(),
                if errors.len() < 1 {
                    Ok(text)
                } else {
                    Err(errors)
                },
            )
        }
        QuoteType::Double => {
            use lexer::token::string::DoubleStringToken as T;
            let mut str_lex: lexer::Lexer<'s, T> = lexer.morph();
            let mut text = String::new();
            let mut errors = Vec::new();
            while let Some(tok) = str_lex.next() {
                match tok {
                    T::Delimiter => {
                        break;
                    }
                    T::Error => {
                        errors.push((str_lex.span(), str_lex.slice()));
                    }
                    T::Escape(c) => {
                        text.push_str(match c {
                            '"' => "\"",
                            '\\' => r"\",
                            'n' => "\n",
                            'r' => "\r",
                            't' => "\t",
                            _ => unreachable!("Invalid escape"),
                        });
                    }
                    T::Text(t) => {
                        text.push_str(&t);
                    }
                }
            }
            (
                str_lex.morph(),
                if errors.len() < 1 {
                    Ok(text)
                } else {
                    Err(errors)
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*, QuoteType::*};
    use lexer::{Logos, Token::{SingleQuote, DoubleQuote, Placeholder}};

    #[test]
    fn parse_single_string() {
        let mut lexer = Token::lexer(r"'a string\n\twith an \\ escaped \'escape\''_");
        if let Some(SingleQuote) = lexer.next() {
            let (mut lex, res) = parse_string(lexer, Single);
            assert_eq!((lex.next(), lex.next()), (Some(Placeholder), None));
            assert_eq!(res, Ok("a string\n\twith an \\ escaped 'escape'".to_owned()));
            assert_eq!(parse_string(Token::lexer("an \\invalid escape'"), Single).1, Err(vec![(3..4, r"\")]));
        }
    }

    #[test]
    fn parse_double_string() {
        let mut lexer = Token::lexer(r#""a string\n\twith an \\ escaped \"escape\""_"#);
        if let Some(DoubleQuote) = lexer.next() {
            let (mut lex, res) = parse_string(lexer, Double);
            assert_eq!((lex.next(), lex.next()), (Some(Placeholder), None));
            assert_eq!(res, Ok("a string\n\twith an \\ escaped \"escape\"".to_owned()));
            assert_eq!(parse_string(Token::lexer("an \\invalid escape\""), Double).1, Err(vec![(3..4, r"\")]));
        }
    }
}
