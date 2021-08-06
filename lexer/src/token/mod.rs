use std::collections::HashMap;
use std::str::FromStr;

use logos::{Lexer, Logos};

pub mod expression;
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
    #[regex(r"_([a-z][A-Za-z0-9]*)?", named_wildcard)]
    Wildcard(Option<String>),

    #[regex(r"//[^\r\n]*", parse_comment)]
    Comment,

    /*#[regex(r#""(?:\\"|[^"])*""#, trim_quotes)] // Double quoted
    #[regex(r#"'(?:\\'|[^'])*'"#, trim_quotes)] // Single quoted*/
    #[regex(r#"('(?:\\'|[^'])*')|"(?:\\"|[^"])*""#, trim_quotes)] //Combined
    StringLit(String),

    #[regex("true|false", parse_slice)]
    Bool(bool),

    #[regex(r"\(\)|undefined")]
    Undefined,

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

    // Manually added when end of input is reached (no more tokens)
    //EOF,
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

fn parse_comment<'s>(lex: &mut Lexer<'s, Token>) {
    lex.extras
        .comments
        .insert(lex.span().start, lex.slice().to_owned());
}

fn named_wildcard<'s>(lex: &mut Lexer<'s, Token>) -> Option<String> {
    let s = lex.slice();
    if s.len() > 1 {
        Some(s[1..].to_owned())
    } else {
        None
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
