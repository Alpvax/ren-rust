#![cfg(test)]

use super::*;

macro_rules! make_test_fn {
    ($n:ident<$t:ty, $e:ty> = $f:expr) => {
        fn $n(input: &str) -> (Result<$t, $e>, Vec<Token>) {
            let mut lexer = Lexer::new(input);
            ($f(&mut lexer), lexer.remaining_tokens_vec())
        }
    };
}

mod helper {
    #![allow(dead_code)]
    use super::Token;
    pub fn assert_err<T, E>(res: (Result<T, E>, Vec<Token>), error_type: E, remaining_count: usize)
    where
        T: std::fmt::Debug,
        E: std::fmt::Debug + PartialEq,
    {
        assert_eq!(
            (res.0.unwrap_err(), res.1.len()),
            (error_type, remaining_count)
        );
    }
    pub fn assert_ok<T, E>(res: (Result<T, E>, Vec<Token>), value: T, remaining_count: usize)
    where
        T: std::fmt::Debug + PartialEq,
        E: std::fmt::Debug,
    {
        assert_eq!((res.0.unwrap(), res.1.len()), (value, remaining_count));
    }
    pub fn ok_remaining<T, E>(res: (Result<T, E>, Vec<Token>), remaining_count: usize) -> T
    where
        E: std::fmt::Debug,
    {
        assert_eq!(
            res.1.len(),
            remaining_count,
            "Expected {} remaining tokens, got: {:?}",
            remaining_count,
            res.1
        );
        res.0.unwrap()
    }
}

mod import {
    use super::{
        helper::*,
        parse_import,
        ImportError::{self, *},
        Lexer, Token,
    };
    use crate::ast::import::Import;

    make_test_fn!(test_import<Import, ImportError> = parse_import);

    #[test]
    fn parse_empty() {
        assert_err(test_import(""), NoTokens, 0);
    }

    #[test]
    fn parse_path() {
        assert_eq!(
            ok_remaining(test_import("import 'a path'"), 0).path,
            "a path"
        );
        assert_err(test_import("import"), MissingImportWhitespace, 0);
        assert_err(test_import("import "), NoPath, 0);
        assert_err(test_import("import'a path'"), MissingImportWhitespace, 0);
        assert_eq!(
            ok_remaining(test_import("import 'a path' followed by other stuff"), 8).path,
            "a path"
        );
    }

    #[test]
    fn parse_namespace() {
        use super::NamespaceError as NErr;
        assert_eq!(
            ok_remaining(test_import("import 'a path' as Name"), 0).namespace,
            Some(vec!["Name".to_owned()])
        );
        assert_err(
            test_import("import 'a path'as Name"),
            MissingAsPrecWhitespace,
            3,
        );
        //TODO: fix trailing: assert_err(test_import("import 'a path' asName"), MissingAsWhitespace, 0,);
        assert_err(test_import("import 'a path' as"), MissingAsWhitespace, 0);
        assert_err(
            test_import("import 'a path' as "),
            NamespaceError(NErr::None),
            0,
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' as Name.Space"), 0).namespace,
            Some(vec!["Name".to_owned(), "Space".to_owned()])
        );
        //TODO: should ask about trailing: assert_err(test_import("import 'a path' as Name . Space"), , 0);
        assert_err(
            test_import("import 'a path' as Name.Space.not"),
            NamespaceError(NErr::InvalidTokenNs),
            0,
        );
    }

    #[test]
    fn parse_exposing() {
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo }"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo, bar }"), 0)
                .exposing
                .unwrap(),
            vec!["foo", "bar"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo , bar }"), 0)
                .exposing
                .unwrap(),
            vec!["foo", "bar"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo , bar , }"), 0)
                .exposing
                .unwrap(),
            vec!["foo", "bar"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo,bar, }"), 0)
                .exposing
                .unwrap(),
            vec!["foo", "bar"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing { foo, bar, }"), 0)
                .exposing
                .unwrap(),
            vec!["foo", "bar"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing{ foo }"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing{foo }"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing{foo}"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing {foo}"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_eq!(
            ok_remaining(test_import("import 'a path' exposing {foo }"), 0)
                .exposing
                .unwrap(),
            vec!["foo"]
        );
        assert_err(
            test_import("import 'a path' exposing { foo, Name }"),
            InvalidExposedBlockToken,
            3,
        );
        assert_err(
            test_import("import 'a path' exposing { foo, Name.spaced }"),
            InvalidExposedBlockToken,
            5,
        );
        assert_err(
            test_import("import 'a path' exposing foo"),
            MissingCurlyOpen,
            1,
        );
        assert_err(
            test_import("import 'a path' exposing { foo"),
            MissingCurlyClose,
            0,
        );
        assert_err(
            test_import("import 'a path' exposing { foo, "),
            MissingCurlyClose,
            0,
        );
    }

    #[test]
    fn parse_full() {
        let check = (
            "a path".to_owned(),
            vec!["Name".to_owned(), "Space".to_owned()],
            vec!["foo".to_owned(), "bar".to_owned()],
        );
        let i = ok_remaining(
            test_import("import 'a path' as Name.Space exposing { foo, bar, }"),
            0,
        );
        assert_eq!((i.path, i.namespace.unwrap(), i.exposing.unwrap()), check);
        let i = ok_remaining(
            test_import("import 'a path' as Name.Space exposing { foo, bar }"),
            0,
        );
        assert_eq!((i.path, i.namespace.unwrap(), i.exposing.unwrap()), check);
    }
}
