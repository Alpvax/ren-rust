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

pub mod helper {
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

mod declaration_tests;
mod expression_tests;
mod import_tests;
mod pattern_tests;
