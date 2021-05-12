mod import_parser;
mod names;
mod string_parser;

use import_parser::{parse_import, Import};
use names::*;

enum Statement {
    Import(Import),
}
impl From<Import> for Statement {
    fn from(i: Import) -> Self {
        Statement::Import(i)
    }
}

fn main() {
    println!(
        "Namespace: {:?}",
        "Some.Other.Namespace".parse::<Identifier>()
    );
    println!(
        "Namespaced: {:?}",
        "Some.Other.Namespace.foo".parse::<Identifier>()
    );
    println!("Namespace: {:?}", "bar".parse::<Identifier>());
    println!("Ren parser in rust\n");

    assert_eq!(
        Import::new("./some/thing", None, None,),
        parse_import(r#"import "./some/thing""#).unwrap().1
    );
    assert_eq!(
        Import::new("./some/thing", Some(vec!["Some", "Thing"]), None,),
        parse_import(r#"import "./some/thing" as Some.Thing"#)
            .unwrap()
            .1
    );
    assert_eq!(
        Import::new("./some/thing", None, Some(vec!["foo", "bar"]),),
        parse_import(r#"import "./some/thing" exposing { foo, bar }"#)
            .unwrap()
            .1
    );
    assert_eq!(
        Import::new(
            "./some/thing",
            Some(vec!["Some", "Thing"]),
            Some(vec!["foo", "bar"]),
        ),
        parse_import(r#"import "./some/thing" as Some.Thing exposing { foo, bar }"#)
            .unwrap()
            .1
    );
}
