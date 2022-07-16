use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub(crate) enum Context {
    Module, // Root context

    Import,    // After import keyword
    ImportNS,  //After as
    ImportExp, // Inside exposing block

    Public, //After pub keyword

    Pattern,

    Block,

    BinOp,
    PrefixOp,
}
