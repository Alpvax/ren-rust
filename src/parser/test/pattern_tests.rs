use super::{
    helper::*,
    parse_pattern, Lexer,
    PatternParseError::{self, *},
    Token,
};
use crate::ast::expression::Pattern;

make_test_fn!(test_pattern<Pattern, PatternParseError> = parse_pattern);

#[test]
fn parse_empty() {
    assert_err(test_pattern(""), NoTokens, 0);
}

#[test]
fn parse_name() {
    assert_eq!(
        ok_remaining(test_pattern("named"), 0),
        Pattern::Name("named".to_owned())
    );
    assert_err(test_pattern("Namespace"), InvalidPattern, 0);
}

#[test]
fn parse_wildcard() {
    assert_eq!(
        ok_remaining(test_pattern("_wildcardName"), 0),
        Pattern::Wildcard(Some("wildcardName".to_owned()))
    );
    assert_eq!(ok_remaining(test_pattern("_"), 0), Pattern::Wildcard(None));
}

#[test]
fn parse_array_destructure_simple() {
    let v = vec![Pattern::Name("foo".to_owned())];
    assert_eq!(
        ok_remaining(test_pattern("[foo]"), 0),
        Pattern::ArrayDestructure(v.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("[ foo ]"), 0),
        Pattern::ArrayDestructure(v.clone())
    );
    assert_err(test_pattern("[ Foo ]"), InvalidPattern, 2);

    let v2 = vec![
        Pattern::Name("foo".to_owned()),
        Pattern::Name("bar".to_owned()),
    ];
    assert_eq!(
        ok_remaining(test_pattern("[foo,bar]"), 0),
        Pattern::ArrayDestructure(v2.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("[ foo, bar ]"), 0),
        Pattern::ArrayDestructure(v2.clone())
    );
    // Trailing comma consumes closing ']'
    assert_err(test_pattern("[ foo, bar, ]"), TrailingComma, 0);

    assert_eq!(
        ok_remaining(test_pattern("[ foo, _, _bar ]"), 0),
        Pattern::ArrayDestructure(vec![
            Pattern::Name("foo".to_owned()),
            Pattern::Wildcard(None),
            Pattern::Wildcard(Some("bar".to_owned())),
        ])
    );

    assert_err(test_pattern("[]"), EmptyDestructure, 0);
}

#[test]
fn parse_array_destructure_nested() {
    assert_eq!(
        ok_remaining(test_pattern("[foo, [_, bar], {baz}]"), 0),
        Pattern::ArrayDestructure(vec![
            Pattern::Name("foo".to_owned()),
            Pattern::ArrayDestructure(vec![
                Pattern::Wildcard(None),
                Pattern::Name("bar".to_owned()),
            ]),
            Pattern::ObjectDestructure(vec![("baz".to_owned(), None)]),
        ])
    );
}

#[test]
fn parse_object_destructure_simple() {
    let v = vec![("foo".to_owned(), None)];
    assert_eq!(
        ok_remaining(test_pattern("{foo}"), 0),
        Pattern::ObjectDestructure(v.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("{ foo }"), 0),
        Pattern::ObjectDestructure(v.clone())
    );
    assert_err(test_pattern("{ Foo }"), InvalidObjKey, 2);

    let v2 = vec![("foo".to_owned(), None), ("bar".to_owned(), None)];
    assert_eq!(
        ok_remaining(test_pattern("{foo,bar}"), 0),
        Pattern::ObjectDestructure(v2.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("{ foo, bar }"), 0),
        Pattern::ObjectDestructure(v2.clone())
    );
    // Trailing comma consumes closing '}'
    assert_err(test_pattern("{ foo, bar, }"), TrailingComma, 0);

    assert_err(test_pattern("{ foo, _bar }"), InvalidObjKey, 2);
}

#[test]
fn parse_object_destructure_subpattern() {
    let v = vec![(
        "foo".to_owned(),
        Some(Pattern::ArrayDestructure(vec![
            Pattern::Name("bar".to_owned()),
            Pattern::Wildcard(Some("baz".to_owned())),
        ])),
    )];
    assert_eq!(
        ok_remaining(test_pattern("{foo:[bar, _baz]}"), 0),
        Pattern::ObjectDestructure(v.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("{ foo : [ bar, _baz ] }"), 0),
        Pattern::ObjectDestructure(v.clone())
    );

    let v2 = vec![
        (
            "foo".to_owned(),
            Some(Pattern::ArrayDestructure(vec![
                Pattern::Wildcard(None),
                Pattern::Name("bar".to_owned()),
            ])),
        ),
        ("x".to_owned(), Some(Pattern::Name("y".to_owned()))),
        (
            "baz".to_owned(),
            Some(Pattern::ArrayDestructure(vec![Pattern::Wildcard(Some(
                "foz".to_owned(),
            ))])),
        ),
    ];
    println!(
        "Multiple: {:?}",
        test_pattern("{foo:[_, bar],x:y,baz:[_foz]}").0
    );
    assert_eq!(
        ok_remaining(test_pattern("{foo:[_, bar],x:y,baz:[_foz]}"), 0),
        Pattern::ObjectDestructure(v2.clone())
    );
    assert_eq!(
        ok_remaining(test_pattern("{foo : [_, bar], x: y, baz: [ _foz ] }"), 0),
        Pattern::ObjectDestructure(v2.clone())
    );
}
