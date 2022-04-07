mod import_tests;

macro_rules! make_assert_funcs {
    ($ok:ty, $err:ty: $func:path => $t_func:ident) => {
        fn $t_func<'s>(input: &'s str) -> Result<$ok, $err> {
            let mut lexer = lexer::Lexer::new(input);
            $func(&mut lexer)
        }
        fn assert_err<'s>(input: &'s str, error: $err) {
            let res = $t_func(input);
            assert_eq!(res.unwrap_err(), error);
        }
        fn assert_ok<'s>(input: &'s str, value: $ok) {
            let res = $t_func(input);
            assert_eq!(res.unwrap(), value);
        }
    };
    ($ok:ty, $err:ty: $func:path) => {
        mod __internal_assert {
            make_assert_funcs!($ok, $err: $func => do_parse);
        }
        use __internal_assert::{assert_err, assert_ok};
    };
}
use make_assert_funcs;
