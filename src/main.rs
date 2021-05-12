mod import;
mod names;
mod parser;
mod token;

use logos::Logos;
pub use token::Token;

fn main() {
    let mut lexer = Token::lexer(
        r#"import "foo/bar" as Foo.Bar exposing { foo, bar, baz }

        pub fun foo = x y z => undefined

        fun foo = _ { x } [ z ] => undefined

        fun foo = x y z => if z == b then (x + a) * z else (x + a) * y
            where a = "some string"
            and b = a * 3

        pub let baz = 3600

        let baz = 34 |> a >> b
            where a = fun x => x * 2
            and b = fun x => x / 2

        let a = fun c => c * 1000"#,
    );
    while let Some(tok) = lexer.next() {
        println!("{:?}: {:?}; {}", tok, lexer.span(), lexer.slice())
    }
    /*println!(
        "{:?}",
        parser::parse(lexer)
    );*/
}
