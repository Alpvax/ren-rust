#![allow(dead_code)]

mod ast;
mod parser;
mod token;
mod value;

use derive_tokentype::SimplifiedEnum;
use logos::Logos;
pub use token::Token;

const SAMPLE: &str = r#"import "foo/bar" as Foo.Bar exposing { foo, bar, baz }

pub fun foo = x y z => undefined

fun foo = _ { x } [ z ] => undefined

fun foo = x y z => if z == b then (x + a) * z else (x + a) * y
    where a = "some string"
    and b = a * 3

pub let baz = 3600

let baz = 34 |> a >> b
    where a = fun x => x * 2
    and b = fun x => x / 2

let a = fun c => c * 1000"#;

trait SimplifiedEnum {
    type Simple;
    fn simple_type(&self) -> Self::Simple;
}

#[derive(Debug, SimplifiedEnum)]
enum T {
    A { x: u8 },
    B(char),
    C,
}

fn main() {
    let f = value::Value::function_n(2);
    println!("{:?}", f.get_num_args());

    println!(
        "A:{:?} -> {:?},\nB:{:?} -> {:?},\nC:{:?} -> {:?},",
        T::A { x: 3 },
        <T as SimplifiedEnum>::Simple::from(T::A { x: 3 }),
        T::B('c'),
        <T as SimplifiedEnum>::Simple::from(T::B('c')),
        T::C,
        <T as SimplifiedEnum>::Simple::from(T::C),
    );

    println!("{:?}", parser::parse(Token::lexer(SAMPLE).spanned()));
}
