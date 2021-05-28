#![allow(dead_code)]

pub mod declaration;
pub mod expression;
pub mod import;
pub mod names;
pub use names::{Identifier, Namespace, VarName};
pub mod namespace;
pub use namespace::Module;
