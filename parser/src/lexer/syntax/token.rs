use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
// #[logos(extras = LexerExtras)]
pub enum Token {
    #[error]
    Error = 0,

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

    #[regex(r"[A-Z][A-Za-z0-9]*")]
    Namespace,
    #[regex(r"([a-z][A-Za-z0-9]*)")]
    VarName,
    #[regex(r"_")]
    Placeholder,

    #[regex(r"//[^\r\n]*")]
    Comment,

    /*#[regex(r#""(?:\\"|[^"])*""#, trim_quotes)] // Double quoted
    #[regex(r#"'(?:\\'|[^'])*'"#, trim_quotes)] // Single quoted*/
    // #[regex(r#"('(?:\\'|[^'])*')|"(?:\\"|[^"])*""#, trim_quotes)] //Combined
    //StringLit(String),
    #[regex(r"((?:0|[1-9][0-9]*)?(?:\.[0-9]+)(?:[eE][+-]?[0-9]+)?)|(0|[1-9][0-9]*)|(0[xX][0-9a-fA-F]+)|(0[oO][0-7]+)|(0[bB][01]+)")]
    Number,

    #[regex("true|false")]
    Bool,

    #[regex(r"\(\)|undefined")]
    Undefined,

    #[token("\"")]
    DoubleQuote,
    // #[token("'")]
    // SingleQuote,
    // #[token("`")]
    // Backtick,
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
}

#[derive(
    Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
pub enum StringToken {
    #[error]
    Error = 0,

    #[regex(r"\\[\\nrt$`]", priority = 5)]
    Escape,

    #[token("\"")]
    Delimiter,

    #[regex(r#"[^\$"]+"#)]
    Text,

    #[token("${")]
    ExprStart,
}
