use std::collections::HashMap;
use std::str::FromStr;

use logos::{Lexer, Logos};

#[derive(Debug, Default, Clone)]
pub struct LexerExtras {
    comments: HashMap<usize, String>,
}

#[derive(Logos, Debug, Clone, /*Copy,*/ PartialEq)]
#[logos(extras = LexerExtras)]
pub enum Token {
    #[token("ren")] // 'ren' keyword reserved for future use
    KWRen,

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
    #[token("ret")]
    KWRet,
    #[token("if")]
    KWIf,
    #[token("then")]
    KWThen,
    #[token("else")]
    KWElse,

    #[regex(r"[A-Z][A-Za-z0-9]*", owned)]
    Namespace(String),
    #[regex(r"([a-z][A-Za-z0-9]*)", owned)]
    VarName(String),
    #[regex(r"_([a-z][A-Za-z0-9]*)?", owned)]
    Wildcard(String),

    #[regex(r"//[^\r\n]*", parse_comment)]
    Comment,

    /*#[regex(r#""(?:\\"|[^"])*""#, trim_quotes)] // Double quoted
    #[regex(r#"'(?:\\'|[^'])*'"#, trim_quotes)] // Single quoted*/
    #[regex(r#"('(?:\\'|[^'])*')|"(?:\\"|[^"])*""#, trim_quotes)] //Combined
    String(String),

    /*#[regex(r"(?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?", parse_slice)] // Float
    #[regex(r"0|[1-9][0-9]*", parse_slice)] // Int
    #[regex(r"0[xX][0-9a-fA-F]+", callback = parse_int_base(16))] // Hex
    #[regex(r"0[oO][0-7]+", callback = parse_int_base(8))] // Oct
    #[regex(r"0[bB][01]+", callback = parse_int_base(2))] // Bin*/
    #[regex(r"((?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?)|(0|[1-9][0-9]*)|(0[xX][0-9a-fA-F]+)|(0[oO][0-7]+)|(0[bB][01]+)", parse_number)]
    //Combined
    Number(f64),

    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,

    #[token("=")]
    OpAssign,
    #[token("=>")]
    OpFun,

    #[token("|>")]
    OpPipe, //infixLeft 1
    #[token(">>")]
    OpCompose, //infixRight 9
    #[token("==")]
    OpEq, //infixLeft  4
    #[token("!=")]
    OpNotEq, //infixLeft  4
    #[token("<=")]
    OpLte, //infixLeft  4
    #[token(">=")]
    OpGte, //infixLeft  4
    #[token("&&")]
    OpAnd, //infixRight 3
    #[token("||")]
    OpOr, //infixRight 2
    #[token("::")]
    OpCons, //infixRight 5
    #[token("++")]
    OpJoin, //infixRight 5

    // SINGLE CHARACTER OPERATORS
    #[token(";")]
    OpDiscard, //infixRight 1
    #[token("<")]
    OpLt, //infixLeft  4
    #[token(">")]
    OpGt, //infixLeft  4
    #[token("+")]
    OpAdd, //infixLeft  6
    #[token("-")]
    OpSub, //infixLeft  6
    #[token("*")]
    OpMul, //infixLeft  7
    #[token("/")]
    OpDiv, //infixLeft  7
    #[token("^")]
    OpPow, //infixRight 8
    #[token("%")]
    OpMod, //infixRight 8

    #[regex("true|false", parse_slice)]
    Bool(bool),

    /*#[regex(r"[\r\n]+", priority=2)]
    NewLine,*/
    #[regex(r"\s+")]
    Whitespace,

    #[error]
    Error,

    // Manually added when end of input is reached (no more tokens)
    EOF,
}
/*impl Token {
    pub fn is_whitespace(&self) -> bool {
        match self {
            Self::Whitespace | Self::NewLine => true,
            _ => false,
        }
    }
}*/

fn owned<'s>(lex: &mut Lexer<'s, Token>) -> String {
    lex.slice().to_owned()
}

fn parse_slice<'s, T: FromStr>(lex: &mut Lexer<'s, Token>) -> Result<T, T::Err> {
    lex.slice().parse()
}

fn parse_comment<'s>(lex: &mut Lexer<'s, Token>) {
    lex.extras
        .comments
        .insert(lex.span().start, lex.slice().to_owned());
}

fn parse_int_base<'s>(radix: u32) -> impl FnMut(&mut Lexer<'s, Token>) -> f64 {
    move |lex: &mut Lexer<'s, Token>| {
        u32::from_str_radix(&lex.slice()[2..], radix)
            .unwrap()
            .into()
    }
}

enum ParseNumError {
    BaseError(std::num::ParseIntError),
    Float(std::num::ParseFloatError),
}
fn parse_number<'s>(lex: &mut Lexer<'s, Token>) -> Result<f64, ParseNumError> {
    fn parse_base(s: &str, base: u32) -> Result<f64, ParseNumError> {
        u32::from_str_radix(&s[2..], base)
            .map_err(ParseNumError::BaseError)
            .map(u32::into)
    }
    let s = lex.slice();
    if s.len() > 2 {
        match &s[..2] {
            "0x" | "0X" => parse_base(s, 16),
            "0o" | "0O" => parse_base(s, 8),
            "0b" | "0B" => parse_base(s, 2),
            _ => s.parse().map_err(ParseNumError::Float),
        }
    } else {
        s.parse().map_err(ParseNumError::Float)
    }
}

fn trim_quotes<'s>(lex: &mut Lexer<'s, Token>) -> String {
    let s = lex.slice();
    let len = s.len();
    if len > 2 {
        s[1..len - 1].to_owned()
    } else {
        String::new()
    }
}
