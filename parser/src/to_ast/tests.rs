use ast::{
    core::{Literal, Pattern},
    expr::{Expr, Operator},
};

fn check(input: &str, expected: Expr) {
    let parsed = crate::parse_expr_ast(input);
    if parsed.is_err() {
        panic!(
            "ERROR converting {} to ast::Expr. Parsed:\n{}",
            input,
            crate::parse_expression(input).debug_tree()
        );
    } else {
        assert_eq!(parsed.unwrap(), expected);
    }
}

mod literal {
    use super::*;

    #[test]
    fn number() {
        check("143", Expr::literal(143));
    }

    mod string {
        use super::*;

        #[test]
        #[ignore = "unimplemented"]
        fn simple_string() {
            check(r#""Hello World""#, Expr::literal("Hello World"));
        }

        #[test]
        #[ignore = "unimplemented"]
        fn string_with_escapes() {
            check(r#""Hello\n\tWorld""#, Expr::literal("Hello\n\tWorld"));
        }

        #[test]
        #[ignore = "unimplemented"]
        fn nested_string() {
            check(
                r#""Hello\n${"\tworld \${text}"}""#,
                Expr::literal("Hello\n\tworld ${text}"),
            );
        }
    }

    #[test]
    fn array() {
        check(
            "[foo, bar]",
            Expr::Literal(Literal::LArr(vec![
                Expr::Var("foo".to_string()),
                Expr::Var("bar".to_string()),
            ])),
        );
    }
    #[test]
    fn record() {
        check(
            "{foo, bar: baz}",
            Expr::Literal(Literal::LRec(
                [("foo", "foo"), ("bar", "baz")]
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), Expr::Var(v.to_string())))
                    .collect(),
            )),
        );
    }
}

mod operator {
    use super::*;

    #[test]
    fn simple_binop() {
        check(
            "2+3",
            Expr::binop(Expr::literal(2), Operator::Add, Expr::literal(3)),
        );
    }

    #[test]
    fn mixed_binop() {
        check(
            "2+3*4-5",
            Expr::binop(
                Expr::binop(
                    Expr::literal(2),
                    Operator::Add,
                    Expr::binop(Expr::literal(3), Operator::Mul, Expr::literal(4)),
                ),
                Operator::Sub,
                Expr::literal(5),
            ),
        );
    }

    #[test]
    fn negate_num() {
        check(
            "-1",
            Expr::binop(Expr::literal(0), Operator::Sub, Expr::literal(1)),
        );
    }

    #[test]
    fn negate_var() {
        check(
            "-foo",
            Expr::binop(
                Expr::literal(0),
                Operator::Sub,
                Expr::Var("foo".to_string()),
            ),
        );
    }

    #[test]
    fn paren_precedence() {
        check(
            "1 + 2 * 3 / (5 - 2)",
            Expr::binop(
                Expr::literal(1),
                Operator::Add,
                Expr::binop(
                    Expr::binop(Expr::literal(2), Operator::Mul, Expr::literal(3)),
                    Operator::Div,
                    Expr::binop(Expr::literal(5), Operator::Sub, Expr::literal(2)),
                ),
            ),
        );
    }

    #[test]
    fn commented_expr() {
        check(
            "1 + 2 // Not applied as a single term
            * 3 // = 6
            /
            (5 - -2) // 5 - (-2) = 6",
            Expr::binop(
                Expr::literal(1),
                Operator::Add,
                Expr::binop(
                    Expr::binop(Expr::literal(2), Operator::Mul, Expr::literal(3)),
                    Operator::Div,
                    Expr::binop(
                        Expr::literal(5),
                        Operator::Sub,
                        Expr::binop(Expr::literal(0), Operator::Sub, Expr::literal(2)),
                    ),
                ),
            ),
        );
    }
}

mod variable {
    use super::*;

    #[test]
    fn local() {
        check("varName1", Expr::Var("varName1".to_string()));
    }

    #[test]
    fn scoped() {
        check(
            "Name.Space.foo",
            Expr::Scoped(
                vec!["Name".to_string(), "Space".to_string()],
                "foo".to_string(),
            ),
        );
    }
}

#[test]
#[ignore = "strings not implemented"]
fn let_expr() {
    check(
        r#"let [foo, _] = [1, "bar"]; foo + 3"#,
        Expr::binding(
            Pattern::PLit(Literal::LArr(vec![
                Pattern::PVar("foo".to_string()),
                Pattern::PAny,
            ])),
            Expr::Literal(Literal::LArr(vec![Expr::literal(1), Expr::literal("bar")])),
            Expr::binop(
                Expr::Var("foo".to_string()),
                ast::expr::Operator::Add,
                Expr::literal(3),
            ),
        ),
    );
}

#[test]
fn access() {
    check(
        "foo.bar.baz",
        Expr::access(Expr::access(Expr::Var("foo".to_string()), "bar"), "baz"),
    );
}

#[test]
fn application() {
    check(
        "foo bar (3 -1)",
        Expr::apply_many(
            Expr::Var("foo".to_string()),
            [
                Expr::Var("bar".to_string()),
                Expr::binop(Expr::literal(3), Operator::Sub, Expr::literal(1)),
            ],
        ),
    );
}

#[test]
#[ignore = "unimplemented"]
fn constructor() {
    check("#foo bar (3 -1)", Expr::default());
}

#[test]
fn conditional() {
    check(
        "if a and b then c + 4 else 2 * c",
        Expr::conditional(
            Expr::binop(
                Expr::Var("a".to_owned()),
                Operator::And,
                Expr::Var("b".to_owned()),
            ),
            Expr::binop(Expr::Var("c".to_owned()), Operator::Add, Expr::literal(4)),
            Expr::binop(Expr::literal(2), Operator::Mul, Expr::Var("c".to_owned())),
        ),
    );
}

#[test]
fn nested_conditional() {
    check(
        "if if a and b then c + 4 else 2 * c then if d or e then 2 else 3 else f",
        Expr::conditional(
            Expr::conditional(
                Expr::binop(
                    Expr::Var("a".to_owned()),
                    Operator::And,
                    Expr::Var("b".to_owned()),
                ),
                Expr::binop(Expr::Var("c".to_owned()), Operator::Add, Expr::literal(4)),
                Expr::binop(Expr::literal(2), Operator::Mul, Expr::Var("c".to_owned())),
            ),
            Expr::conditional(
                Expr::binop(
                    Expr::Var("d".to_owned()),
                    Operator::Or,
                    Expr::Var("e".to_owned()),
                ),
                Expr::literal(2),
                Expr::literal(3),
            ),
            Expr::Var("f".to_string()),
        ),
    );
}

#[test]
#[ignore = "unimplemented"]
fn where_expr() {
    check(
        r#"where foo is 1 => "hello" is 2 if bar > foo => "world" is #just baz => "baz" is _ => "!""#,
        Expr::default(),
    );
}

#[test]
#[ignore = "unimplemented"]
fn lambda() {
    check("fun a b => a * (3 - b)", Expr::lambda());
}
