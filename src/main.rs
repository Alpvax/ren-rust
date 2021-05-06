mod import;
mod names;
mod parser;
mod token;

use logos::Logos;
pub use token::Token;


fn main() {
    println!(
        "{:?}",
        parser::parse(Token::lexer(
            r#"import "./some/thing" as Some.Thing exposing {foo, bar}"#
        )) //.collect::<Vec<_>>()
    );
}
