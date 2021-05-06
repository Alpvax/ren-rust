use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'s> {
    #[token("import")]
    KWImport,
    #[token("as")]
    KWAs,
    #[token("exposing")]
    KWExposing,
    #[token("let")]
    KWLet,
    #[token("fun")]
    KWFun,
    #[token("pub")]
    KWPub,
    #[token("where")]
    KWWhere,
    #[token("if")]
    KWIf,
    #[token("then")]
    KWThen,
    #[token("else")]
    KWElse,

    #[regex(r"[A-Z][A-Za-z0-9]*(\.[A-Z][A-Za-z0-9]*)*", |lex| lex.slice().split('.').collect::<Vec<_>>())]
    Namespace(Vec<&'s str>),
    #[regex(r"([a-z][A-Za-z0-9]*)")]
    VarName(&'s str),

    #[regex(r#""(?:\\"|[^"])*""#, trim_quotes)]
    StrDbl(&'s str),
    #[regex(r#"'(?:\\"|[^"])*'"#, trim_quotes)]
    StrSingle(&'s str),

    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,

    EOF,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}

fn trim_quotes<'s>(lex: &mut Lexer<'s, Token<'s>>) -> &'s str {
    let s = lex.slice();
    let len = s.len();
    if len > 2 {
        &s[1..len - 1]
    } else {
        ""
    }
}
