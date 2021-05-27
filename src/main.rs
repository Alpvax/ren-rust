#![allow(dead_code)]

mod ast;
mod parser;
mod value;

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

fn main() {
    let f = value::Value::function_n(2);
    println!("{:?}", f.get_num_args());

    println!("{:?}", parser::parse(SAMPLE));
    println!("\nGood:");
    parser::parse(r#"import "foo/bar" as Foo.Bar exposing { foo, bar, baz }"#);
    println!("\nBad:");
    parser::parse(r#"import"foo/bar" as someFoo . _Barexposing { foo, bar, baz }"#);
}
