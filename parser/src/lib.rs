pub(crate) type Token = lexer::Token;

pub(crate) mod import;
pub use import::parse_import;

pub(crate) mod declaration;
pub use declaration::{parse_block_declaration, parse_module_declaration};

// pub type ParserFunc<'source, T/*, P: TryInto<T>*/> = fn(&'source mut lexer::Lexer<'source, Token>) -> T;

#[cfg(test)]
mod tests;
