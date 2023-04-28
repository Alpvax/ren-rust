use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
// #[logos(extras = LexerExtras)]
pub enum Token {
    #[error]
    Error = 0,

    #[regex(r"//[^\r\n]*\r?\n?")]
    Comment,

    // Keywords
    #[token("as")]
    KWAs,
    #[token("assert")]
    KWAssert,
    #[token("case")]
    KWCase,
    #[token("else")]
    KWElse,
    #[token("expect")]
    KWExpect,
    // #[token("exposing")]
    // KWExposing,
    #[token("ext")]
    KWExt,
    #[token("forall")]
    KWForall,
    #[token("fun")]
    KWFun,
    #[token("if")]
    KWIf,
    #[token("import")]
    KWImport,
    #[token("let")]
    KWLet,
    #[token("on")]
    KWOn,
    #[token("pkg")]
    KWPkg,
    #[token("pub")]
    KWPub,
    // #[token("ren")] // 'ren' keyword reserved for future use
    // KWRen,
    #[token("switch")]
    KWSwitch,
    #[token("then")]
    KWThen,
    #[token("type")]
    KWType,

    // Identifiers
    #[regex(r"[A-Z][A-Za-z0-9]*")]
    IdUpper,
    #[regex(r"([a-z][A-Za-z0-9_]*)")]
    IdLower,

    // Literals
    /// Supported number formats:
    /// - int: zero or positive ints with an optional scientific postfix. (`[0-9]+([eE][+-]?[0-9]+)?`)
    /// - float: positive floats with an optional scientific postfix. portion before `.` is optional. (`[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?`)
    /// - hexadecimal:- `0x[0-9a-fA-F]+` always positive. e.g. 0xAE = 174
    /// - binary:-      `0b[01]+`        always positive. e.g. 0b11 = 3
    /// - octal:-       `0o[0-7]+`       always positive. e.g. 0o14 = 12
    #[regex(r"[0-9]+(?:[eE][+-]?[0-9]+)?")] // Int
    #[regex(r"[0-9]*\.[0-9]+(?:[eE][+-]?[0-9]+)?")] // Float
    #[regex(r"(0[xX][0-9a-fA-F]+)|(0[oO][0-7]+)|(0[bB][01]+)")] // Hex/Oct/Bin
    Number,

    // Operators
    #[token("+")]
    OpAdd, //infixLeft  6
    #[token("&")]
    OpAnd, //infixRight 3
    #[token("<>")]
    OpConcat, //infixRight 5
    #[token("/")]
    OpDiv, //infixLeft  7
    #[token("==")]
    OpEq, //infixLeft  4
    #[token(">")]
    OpGt, //infixLeft  4
    #[token(">=")]
    OpGte, //infixLeft  4
    #[token("<")]
    OpLt, //infixLeft  4
    #[token("<=")]
    OpLte, //infixLeft  4
    #[token("%")]
    OpMod, //infixRight 8
    #[token("*")]
    OpMul, //infixLeft  7
    #[token("!=")]
    OpNeq, //infixLeft  4
    #[token("|")]
    OpOr, //infixRight 2
    #[token("|>")]
    OpPipe, //infixLeft 1
    #[token("^")]
    OpPow, //infixRight 8
    #[token(";")]
    OpSeq,
    #[token("-")]
    OpSub, //infixLeft  6

    // Symbols
    #[token("→")]
    #[token("->")]
    SymArrow,
    #[token("@")]
    SymAt,
    #[token(":")]
    SymColon,
    #[token(",")]
    SymComma,
    #[token("\"")]
    SymDoubleQuote,
    #[token(".")]
    SymDot,
    #[token("..")]
    SymDoubleDot,
    // #[token("//")]
    // SymDoubleSlash,
    #[token("=")]
    SymEquals,
    #[token("#")]
    SymHash,
    #[token("{")]
    SymLBrace,
    #[token("[")]
    SymLBracket,
    #[token("(")]
    SymLParen,
    #[token("?")]
    SymQuestion,
    #[token("}")]
    SymRBrace,
    #[token("]")]
    SymRBracket,
    #[token(")")]
    SymRParen,
    #[token("_")]
    SymUnderscore,

    //////////////////////////////////////////////////////////////////

    // #[token("=>")]
    // OpFatArrow,

    // #[token(">>")]
    // OpCompose, //infixRight 9
    // #[token("::")]
    // OpCons, //infixRight 5

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

    #[regex(r#"\\[\\nrt$"]"#, priority = 5)]
    Escape,

    #[token("\"")]
    Delimiter,

    #[regex(r#"[^\\$"]+"#)]
    Text,

    #[token("${")]
    ExprStart,
}
