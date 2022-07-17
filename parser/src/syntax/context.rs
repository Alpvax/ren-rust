use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub(crate) enum Context {
    Module, // Root context

    Imports,
    Import,        // After import keyword
    NameSpace,
    ExposingBlock, // Inside exposing block

    Declarations,
    Declaration,

    Expr,

    String,
    Scoped,
    Constructor,
    Array,
    Record,
    Field,

    Conditional,
    Condition,
    Then,
    Else,

    Where,
    Branch,
    Guard,

    Lambda,
    Params,

    Application,

    PrefixOp,
    BinOp,

    Args,

    Pattern,
}
