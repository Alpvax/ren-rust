use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum TemplateLiteralToken {
    #[error]
    Error,

    #[regex(r"\\[\\nrt$`]", priority = 5)]
    Escape,

    #[token("`")]
    Delimiter,

    #[regex(r"[^\$`]+")]
    Text,

    #[token("${")]
    ExprStart,
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum DoubleStringToken {
    #[error]
    Error,

    #[regex(r#"\\[\\nrt"]"#, priority = 5)]
    Escape,

    #[token("\"")]
    Delimiter,

    #[regex(r#"[^\\"]+"#)]
    Text,
}

// #[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum SingleStringToken {
//     #[error]
//     Error,

//     #[regex(r"\\[\\nrt']", priority = 5)]
//     Escape,

//     #[token("'")]
//     Delimiter,

//     #[regex(r"[^\\']+")]
//     Text,
// }
