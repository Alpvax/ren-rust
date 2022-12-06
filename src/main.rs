//mod ast;
//mod parser;
#[allow(dead_code)]
mod value;

mod repl;

#[cfg(test)]
mod test;
mod cli;

use repl::Modes as ReplModes;

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

fn main() -> Result<(), String> {//impl std::error::Error> {
    let cli = cli::parse();
    match cli.cmd {
        cli::Cmd::Repl { mode, histfile } => repl::init_repl(&histfile, mode).map_err(|e| e.to_string()),
        cli::Cmd::Parse { infile, ofile, format, stdinput } => {
            let input = if let Some(ipath) = infile {
                std::fs::read_to_string(ipath).map_err(|e| e.to_string())?
            } else if let Some(stdin) = stdinput {
                stdin
            } else {
                return Err("No source to parse".to_owned());
            };
            let mut output = Vec::new();
            parser::parse_stmt_ast(&input).and_then(|(s, ll)| format.handle_stmt(&mut output, s, &ll))?;
            if let Some(opath) = ofile {
                std::fs::write(opath, output).map_err(|e| e.to_string())?
            }
            Ok(())
        },
    }
    //println!("{:?}", parser::parse(SAMPLE));
}
