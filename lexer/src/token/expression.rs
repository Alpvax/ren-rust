use logos::{Lexer, Logos};
use super::LexerExtras;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = LexerExtras)]
pub enum ExpressionToken {
    #[error]
    Error,

    ///TODO: move back into toplevel?
    #[regex(r"//[^\r\n]*", parse_comment)]
    Comment,

    // --- BEGIN STRING LITERAL ------------------------------------------------
    #[token("\"")]
    DoubleStringStart,
    #[token("'")]
    SingleStringStart,
    #[token("`")]
    TemplateStart,

    /// --- BEGIN BLOCK EXPRESSION / OBJECT LITERAL / PATTERN ------------------
    #[token("{")]
    CurlyOpen,
    /// End block / object literal or pattern / end expression inside template
    #[token("}")]
    CurlyClose,

    /// --- BEGIN ARRAY LITERAL / PATTERN --------------------------------------
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,

    /// --- BEGIN SUBEXPRESSION -------------------------------------------------
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,

    /// --- COMBINED NUMBER HANDLING -------------------------------------------
    // #[regex(r"(?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?", parse_slice)] // Float
    // #[regex(r"0|[1-9][0-9]*", parse_slice)] // Int
    // #[regex(r"0[xX][0-9a-fA-F]+", callback = parse_int_base(16))] // Hex
    // #[regex(r"0[oO][0-7]+", callback = parse_int_base(8))] // Oct
    // #[regex(r"0[bB][01]+", callback = parse_int_base(2))] // Bin
    #[regex(r"((?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?)|(0|[1-9][0-9]*)|(0[xX][0-9a-fA-F]+)|(0[oO][0-7]+)|(0[bB][01]+)", parse_number)]
    Number(f64),
}

//TODO: move back into toplevel?
/// --- COMMENT HANDLING -------------------------------------------------------
fn parse_comment<'s>(lex: &mut Lexer<'s, ExpressionToken>) {
    lex.extras
        .comments
        .insert(lex.span().start, lex.slice().to_owned());
}


/// --- COMBINED NUMBER HANDLING -------------------------------------------
enum ParseNumError {
    BaseError(std::num::ParseIntError),
    Float(std::num::ParseFloatError),
}
fn parse_number<'s>(lex: &mut Lexer<'s, ExpressionToken>) -> Result<f64, ParseNumError> {
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
