use super::{
    helper::*, parse_expression, parse_literal, ExpressionError as ExpErr, Lexer,
    LiteralError as LitErr, Token,
};
use crate::ast::expression::{Expression, Literal};

make_test_fn!(test_expr<Expression, ExpErr> = parse_expression);
make_test_fn!(test_lit<Literal, LitErr> = parse_literal);

#[test]
fn parse_empty() {
    assert_err(test_expr(""), ExpErr::NoTokens, 0);
    assert_err(test_lit(""), LitErr::NoTokens, 0);
}

mod parse_literal {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn string() {
        fn test_str_lit(input: &str, remaining_count: usize, output: &str) {
            if let Literal::String(s) = ok_remaining(test_lit(input), remaining_count) {
                assert_eq!(s, output);
            } else {
                panic!("Not a string literal!");
            }
        }
        test_str_lit(r#"'single string'"#, 0, r#"single string"#);
        test_str_lit(r#""double string""#, 0, r#"double string"#);
        test_str_lit(
            r#"'single string with "contained" \'quotes\' in it'"#,
            0,
            r#"single string with "contained" \'quotes\' in it"#,
        );
        test_str_lit(
            r#""double string with \"contained\" 'quotes' in it""#,
            0,
            r#"double string with \"contained\" 'quotes' in it"#,
        );
        if let (Ok(Literal::String(s)), rem) = test_lit("not a string") {
            panic!(
                "\"not a string\" parsed as string literal: {}; remaining: {:?}",
                s, rem
            );
        }
    }

    #[test]
    fn number() {
        fn test_num_lit(input: &str, remaining_count: usize, output: f64) {
            if let Literal::Number(n) = ok_remaining(test_lit(input), remaining_count) {
                assert_eq!(n, output);
            } else {
                panic!("Not a number literal!");
            }
        }
        test_num_lit("14", 0, 14.0);
        test_num_lit("-26.42", 0, -26.42);
        test_num_lit("- 0xA3", 0, -163.0);

        if let (Ok(Literal::Number(n)), rem) = test_lit("non-numeric") {
            panic!(
                "\"non-numeric\" parsed as number literal: {}; remaining: {:?}",
                n, rem
            );
        }
    }

    #[test]
    fn boolean() {
        fn test_bool_lit(input: &str, remaining_count: usize, output: bool) {
            if let Literal::Boolean(b) = ok_remaining(test_lit(input), remaining_count) {
                assert_eq!(b, output);
            } else {
                panic!("Not a boolean literal!");
            }
        }
        test_bool_lit("true", 0, true);
        test_bool_lit("false", 0, false);

        if let (Ok(Literal::Boolean(b)), rem) = test_lit("other") {
            panic!(
                "\"other\" parsed as boolean literal: {}; remaining: {:?}",
                b, rem
            );
        }
    }

    #[test]
    fn array() {
        fn test_arr_lit(input: &str, remaining_count: usize, output: Vec<Expression>) {
            if let Literal::Array(v) = ok_remaining(test_lit(input), remaining_count) {
                assert_eq!(v, output);
            } else {
                panic!("Not an array literal!");
            }
        }
        test_arr_lit("[]", 0, Vec::new());
        let v = vec![Expression::Literal(Literal::String("foo".to_owned()))];
        test_arr_lit("['foo']", 0, v.clone());
        test_arr_lit("[ 'foo' ]", 0, v.clone());
        test_arr_lit("['foo',]", 0, v.clone());
        let v2 = vec![
            Expression::Literal(Literal::String("foo".to_owned())),
            Expression::Literal(Literal::Number(2.0)),
        ];
        test_arr_lit("['foo', 2]", 0, v2.clone());
        test_arr_lit("['foo' , 2 ,]", 0, v2.clone());
    }

    #[test]
    fn object() {
        fn test_obj_lit(input: &str, remaining_count: usize, output: HashMap<String, Expression>) {
            if let Literal::Object(v) = ok_remaining(test_lit(input), remaining_count) {
                assert_eq!(v, output);
            } else {
                panic!("Not an object literal!");
            }
        }
        let a = ("a".to_owned(), Expression::Literal(Literal::Number(3.0)));
        let b = ("b".to_owned(), Expression::local_var("b"));

        test_obj_lit("{}", 0, HashMap::new());

        let m = vec![a.clone()].into_iter().collect::<HashMap<_, _>>();
        test_obj_lit("{a:3}", 0, m.clone());
        test_obj_lit("{ a : 3 }", 0, m.clone());
        test_obj_lit("{ a : 3, }", 0, m.clone());
        //assert_err(test_lit("{ a: 3, }"), LitErr::InvalidKey, 2);

        let m2 = vec![b.clone()].into_iter().collect::<HashMap<_, _>>();
        test_obj_lit("{b}", 0, m2.clone());
        test_obj_lit("{ b }", 0, m2.clone());
        test_obj_lit("{ b, }", 0, m2.clone());

        let m3 = vec![a, b].into_iter().collect::<HashMap<_, _>>();
        test_obj_lit("{a:3,b}", 0, m3.clone());
        test_obj_lit("{ a : 3 , b }", 0, m3.clone());
        test_obj_lit("{ a:3, b, }", 0, m3.clone());
    }
}
