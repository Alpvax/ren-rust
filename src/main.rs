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

    /*println!("{:?}", parser::parse(SAMPLE));
    println!("\nGood: {:?}", parser::parse(r#"import "foo/bar" as Foo.Bar exposing { foo, bar, baz }"#));
    println!("\nBad: {:?}", parser::parse(r#"import"foo/bar" as someFoo . _Barexposing { foo, bar, baz }"#));*/

    let import_tests = [
        "",
        "import 'a path'",
        "import",
        "import'a path'",
        "import 'a path' as Name",
        "import 'a path'as Name",
        "import 'a path' asName",
        "import 'a path' as",
        "import 'a path' as Name.Space",
        "import 'a path' as Name . Space",
        "import 'a path' as Name.Space.not",
        "import 'a path' exposing { foo }",
        "import 'a path' exposing { foo, bar }",
        "import 'a path' exposing { foo , bar }",
        "import 'a path' exposing { foo , bar , }",
        "import 'a path' exposing { foo,bar, }",
        "import 'a path' exposing { foo, bar, }",
        "import 'a path' exposing { foo, Name }",
        "import 'a path' exposing { foo, Name.spaced }",
        "import 'a path' exposing foo",
        "import 'a path' exposing{ foo }",
        "import 'a path' exposing{foo }",
        "import 'a path' exposing{foo}",
        "import 'a path' exposing {foo}",
        "import 'a path' exposing {foo }",
        "import 'a path' exposing { foo",
        "import 'a path' exposing { foo, ",
        "import 'a path' as Name.Space exposing { foo, bar, }",
        "import 'a path' as Name.Space exposing { foo, bar }",
    ];
    println!("\n\nImports:");
    for s in import_tests.iter() {
        let mut lexer = parser::Lexer::new(s);
        println!(
            "\n{}:\n  Tokens: {:?}\n  Result: {:?}\n  Remaining: {:?}",
            s,
            parser::Lexer::new(s).map(|l| l.token).collect::<Vec<_>>(),
            parser::parse_import(&mut lexer),
            lexer.map(|l| l.token).collect::<Vec<_>>(),
        );
    }
}
