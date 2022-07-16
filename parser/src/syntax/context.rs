use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub(crate) enum Context {
    Module, // Root context

    Imports,
    Import,        // After import keyword
    ExposingBlock, // Inside exposing block

    Declarations,
    Declaration,

    Expr,

    Pattern,

    Block,

    BinOp,
    PrefixOp,

    String,

    NameSpace,
}
