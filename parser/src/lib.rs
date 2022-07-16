mod grammar;
mod parser;
mod syntax;

pub use self::parser::Parsed;
pub(crate) use self::parser::Parser;
pub use grammar::{parse_expression, parse_module};
