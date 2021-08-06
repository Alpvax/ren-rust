use logos::{Lexer, Logos};
use super::LexerExtras;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = LexerExtras)]
pub enum TemplateLiteralToken {
    #[error]
    Error,

    #[regex(r"\\[\\nrt$`]", priority = 5, callback = last_char)]
    Escape(char),

    #[token("`")]
    Delimiter,

    #[regex(r"[^\$`]+", owned_string)]
    Text(String),

    #[token("${")]
    ExprStart,

    // #[token("}")]
    // ExprEnd,
}

#[derive(Logos, Debug, Clone,  PartialEq)]
#[logos(extras = LexerExtras)]
pub enum DoubleStringToken {
    #[error]
    Error,

    #[regex(r#"\\[\\nrt"]"#, priority = 5, callback = last_char)]
    Escape(char),

    #[token("\"")]
    Delimiter,

    #[regex(r#"[^\\"]+"#, owned_string)]
    Text(String),
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = LexerExtras)]
pub enum SingleStringToken {
    #[error]
    Error,

    #[regex(r"\\[\\nrt']", priority = 5, callback = last_char)]
    Escape(char),

    #[token("'")]
    Delimiter,

    #[regex(r"[^\\']+", owned_string)]
    Text(String),
}

fn owned_string<'s, T: logos::Logos<'s, Source = str>>(lex: &mut Lexer<'s, T>) -> String {
    lex.slice().to_owned()
}

fn last_char<'s, T: logos::Logos<'s, Source = str>>(lex: &mut Lexer<'s, T>) -> char {
    lex.slice().chars().last().unwrap()
}
