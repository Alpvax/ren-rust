use logos::{Lexer, Logos};
use super::LexerExtras;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringSegment {
    Full(String, StringType),
    Start(String, StringType),
    Mid(String),
    End(String, StringType),
}
impl StringSegment {
    fn get_text(&self) -> &str {
        match self {
            StringSegment::Full(s, _) |
            StringSegment::Start(s, _) |
            StringSegment::Mid(s) |
            StringSegment::End(s, _) => s
        }
    }
    fn get_delim_type(&self) -> StringType {
        match self {
            StringSegment::Full(_, t) |
            StringSegment::Start(_, t) |
            StringSegment::End(_, t) => *t,
            StringSegment::Mid(_) => StringType::Backtick,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    Single,
    Double,
    Backtick,
}

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
