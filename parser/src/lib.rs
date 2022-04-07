pub(crate) type Token = lexer::Token;

pub(crate) mod import;
pub use import::parse_import;

// pub type ParserFunc<'source, T/*, P: TryInto<T>*/> = fn(&'source mut lexer::Lexer<'source, Token>) -> T;

#[cfg(test)]
mod tests;
