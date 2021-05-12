use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
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
    #[token("and")]
    KWAnd,
    #[token("if")]
    KWIf,
    #[token("then")]
    KWThen,
    #[token("else")]
    KWElse,

    #[regex(r"[A-Z][A-Za-z0-9]*")]
    Namespace(&'s str),
    #[regex(r"([a-z][A-Za-z0-9]*)")]
    VarName(&'s str),

    #[regex(r"//[^\r\n]*", |l| &l.slice()[2..])]
    Comment(&'s str),

    #[regex(r#""(?:\\"|[^"])*""#, trim_quotes)]
    StrDbl(&'s str),
    #[regex(r#"'(?:\\"|[^"])*'"#, trim_quotes)]
    StrSingle(&'s str),

    #[regex(r"(?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?", |l| l.slice().parse())]
    Float(f64),
    #[regex(r"0|[1-9][0-9]*", |l| l.slice().parse())]
    Int(isize),
    #[regex(r"0[xX][0-9a-fA-F]+", callback = parse_usize_base(16))]
    HexNumber(usize),
    #[regex(r"0[oO][0-7]+", callback = parse_usize_base(8))]
    OctNumber(usize),
    #[regex(r"0[bB][01]+", callback = parse_usize_base(2))]
    BinNumber(usize),

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


    #[token("undefined")]
    Undefined,
    #[regex("true|false", |l| l.slice().parse())]
    Bool(bool),

    EOF,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}

fn parse_usize_base<'s>(radix: u32) -> impl FnMut(&mut Lexer<'s, Token<'s>>) -> usize {
    move |lex: &mut Lexer<'s, Token<'s>>| usize::from_str_radix(&lex.slice()[2..], radix).unwrap()
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
