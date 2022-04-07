use crate::partial::ImportError::{self, *};
use ast::import::Import;

super::make_assert_funcs!(Import, ImportError: crate::parse_import => test_import);

#[test]
fn parse_empty() {
    assert_err("", NonStart);
}

#[test]
fn parse_path() {
    fn assert_path(input: &str, path: &str) {
        match test_import(input) {
            Ok(import) => assert_eq!(import.get_path(), path),
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_path("import 'a path'", "a path");
    assert_err("import", MissingPath);
    assert_err("import ", MissingPath);
    assert_path("import'a path'", "a path");
    assert_path("import 'a path' followed by other stuff", "a path");
}

#[test]
fn parse_alias() {
    fn assert_alias(input: &str, alias: Vec<&str>) {
        let owned_alias = alias.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        match test_import(input) {
            Ok(import) => {
                if let Some(a) = import.get_alias() {
                    assert_eq!(a, &owned_alias);
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_alias("import 'a path' as Name", vec!["Name"]);
    assert_alias("import 'a path'as Name", vec!["Name"]);
    //TODO: fix trailing: assert_err(test_import("import 'a path' asName"), MissingAsWhitespace, 0,);
    assert_err("import 'a path' as", MissingAlias);
    assert_err("import 'a path' as ", MissingAlias);
    assert_alias("import 'a path' as Name.Space", vec!["Name", "Space"]);
    //TODO: should ask about trailing: assert_err(test_import("import 'a path' as Name . Space"), , 0);
    assert_err("import 'a path' as Name.Space.not", MissingAliasSegment);
}

#[test]
fn parse_exposing() {
    fn assert_exposing(input: &str, bindings: Vec<&str>) {
        println!("input: {} -> {:?}", input, test_import(input)); //XXX
        let owned_exp = bindings
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        match test_import(input) {
            Ok(import) => {
                if let Some(b) = import.get_bindings() {
                    assert_eq!(b, &owned_exp);
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_exposing("import 'a path' exposing { foo }", vec!["foo"]);
    assert_exposing("import 'a path' exposing { foo, bar }", vec!["foo", "bar"]);
    assert_exposing("import 'a path' exposing { foo , bar }", vec!["foo", "bar"]);
    assert_err("import 'a path' exposing { foo , bar , }", MissingBinding);
    assert_err("import 'a path' exposing { foo,bar, }", MissingBinding);
    assert_err("import 'a path' exposing { foo, bar, }", MissingBinding);
    assert_exposing("import 'a path' exposing{ foo }", vec!["foo"]);
    assert_exposing("import 'a path' exposing{foo }", vec!["foo"]);
    assert_exposing("import 'a path' exposing{foo}", vec!["foo"]);
    assert_exposing("import 'a path' exposing {foo}", vec!["foo"]);
    assert_exposing("import 'a path' exposing {foo }", vec!["foo"]);
    assert_err("import 'a path' exposing { foo, Name }", MissingBinding);
    assert_err(
        "import 'a path' exposing { foo, Name.spaced }",
        MissingBinding,
    );
    assert_err("import 'a path' exposing foo", MissingBindings);
    assert_err("import 'a path' exposing { foo", MissingCloseExpose);
    assert_err("import 'a path' exposing { foo, ", MissingBinding);
}

#[test]
fn parse_full() {
    let check = ast::Import::from_owned(
        ast::import::Qualifier::None,
        "some/thing",
        vec!["Foo".to_owned(), "Thing".to_owned()],
        vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
    );
    assert_ok(
        "import 'some/thing' as Foo.Thing exposing { foo, bar, baz }",
        check,
    );
}
