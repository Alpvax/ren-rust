use std::collections::HashMap;
use std::str::FromStr;

use logos::{Lexer, Logos};

pub mod string;

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
    #[token("pkg")]
    KWPkg,
    #[token("ext")]
    KWExt,
    #[token("as")]
    KWAs,
    #[token("exposing")]
    KWExposing,

    #[token("type")]
    KWType,
    #[token("pub")]
    KWPub,
    #[token("let")]
    KWLet,
    #[token("run")]
    KWRun,
    #[token("ret")]
    KWRet,

    #[token("if")]
    KWIf,
    #[token("then")]
    KWThen,
    #[token("else")]
    KWElse,
    #[token("where")]
    KWWhere,
    #[token("is")]
    KWIs,

    #[regex(r"[A-Z][A-Za-z0-9]*", owned)]
    Namespace(String),
    #[regex(r"([a-z][A-Za-z0-9]*)", owned)]
    VarName(String),
    #[regex(r"_")]
    Placeholder,

    #[regex(r"//[^\r\n]*", parse_comment)]
    Comment,

    /*#[regex(r#""(?:\\"|[^"])*""#, trim_quotes)] // Double quoted
    #[regex(r#"'(?:\\'|[^'])*'"#, trim_quotes)] // Single quoted*/
    // #[regex(r#"('(?:\\'|[^'])*')|"(?:\\"|[^"])*""#, trim_quotes)] //Combined
    //StringLit(String),

    #[regex(r"((?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?)|(0|[1-9][0-9]*)|(0[xX][0-9a-fA-F]+)|(0[oO][0-7]+)|(0[bB][01]+)", parse_number)]
    Number(f64),

    #[regex("true|false", parse_slice)]
    Bool(bool),

    #[regex(r"\(\)|undefined")]
    Undefined,

    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[token("`")]
    Backtick,

    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
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

    /*#[regex(r"[\r\n]+", priority=2)]
    NewLine,*/
    #[regex(r"\s+")]
    Whitespace,

    #[error]
    Error,

    /// Manually added when end of input is reached (no more tokens)
    EOF,

    /// Manually added text part
    StringSegment(string::StringSegment),
}

// impl TryFrom<&Token> for crate::ast::expression::Operator {
//     type Error = ();

//     fn try_from(value: &Token) -> Result<Self, Self::Error> {
//         match value {
//             Token::OpPipe => Ok(Self::Pipe),
//             Token::OpCompose => Ok(Self::Compose),
//             Token::OpEq => Ok(Self::Eq),
//             Token::OpNotEq => Ok(Self::NotEq),
//             Token::OpLte => Ok(Self::Lte),
//             Token::OpGte => Ok(Self::Gte),
//             Token::OpAnd => Ok(Self::And),
//             Token::OpOr => Ok(Self::Or),
//             Token::OpCons => Ok(Self::Cons),
//             Token::OpJoin => Ok(Self::Join),
//             Token::OpDiscard => Ok(Self::Discard),
//             Token::OpLt => Ok(Self::Lt),
//             Token::OpGt => Ok(Self::Gt),
//             Token::OpAdd => Ok(Self::Add),
//             Token::OpSub => Ok(Self::Sub),
//             Token::OpMul => Ok(Self::Mul),
//             Token::OpDiv => Ok(Self::Div),
//             Token::OpPow => Ok(Self::Pow),
//             Token::OpMod => Ok(Self::Mod),
//             _ => Err(()),
//         }
//     }
// }
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

// --- COMMENT HANDLING --------------------------------------------------------
fn parse_comment<'s>(lex: &mut Lexer<'s, Token>) {
    lex.extras
        .comments
        .insert(lex.span().start, lex.slice().to_owned());
}

// --- COMBINED NUMBER HANDLING ------------------------------------------------
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