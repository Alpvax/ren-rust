//mod ast;
//mod parser;
#[allow(dead_code)]
mod value;

mod repl;

#[cfg(test)]
mod test;

#[cfg(feature = "reqwest")]
fn parse_remote_file(url: &str) -> parser::Parsed {
    use reqwest;
    reqwest::blocking::get(url)
        .and_then(|r| r.text())
        .map(|input| parser::parse_module(input.as_str()))
}

// const SAMPLE: &str = r#"import "foo/bar" as Foo.Bar exposing { foo, bar, baz }

// pub fun foo = x y z => undefined

// fun foo = _ { x } [ z ] => undefined

// fun foo = x y z => if z == b then (x + a) * z else (x + a) * y
//     where a = "some string"
//     and b = a * 3

// pub let baz = 3600

// let baz = 34 |> a >> b
//     where a = fun x => x * 2
//     and b = fun x => x / 2

// let a = fun c => c * 1000"#;

fn main() -> Result<(), impl std::error::Error> {
    repl::init_repl("./.repl_history")
    //println!("{:?}", parser::parse(SAMPLE));
}
