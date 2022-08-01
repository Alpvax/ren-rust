use super::{Expr, Literal, Operator, Pattern};

mod expand_placeholders {
    use super::*;

    fn check(body_factory: fn(&mut dyn FnMut(u8) -> Expr) -> Expr) {
        let mut vars = Vec::new();
        let mut var_f = |index: u8| {
            let var = format!("$temp{}", index);
            vars.push(Pattern::Var(var.clone()));
            Expr::var(var)
        };
        let expected_body = body_factory(&mut var_f);
        assert_eq!(
            body_factory(&mut |_| Expr::placeholder()).replace_placeholders(),
            Expr::lambda(vars, expected_body),
        );
    }

    #[test]
    fn access() {
        check(|var| Expr::access(var(0), "foo"));
    }
    #[test]
    fn binop_lhs() {
        check(|var| Expr::binop(var(0), Operator::Add, Expr::literal(3)));
    }
    #[test]
    fn binop_rhs() {
        check(|var| Expr::binop(Expr::var("foo"), Operator::Mul, var(2)));
    }
    #[test]
    fn binop_both() {
        check(|var| Expr::binop(var(0), Operator::And, var(2)));
    }
    #[test]
    fn call_obj() {
        check(|var| Expr::apply(var(0), Expr::literal(14)));
    }
    #[test]
    fn call_arg() {
        check(|var| Expr::apply(Expr::var("foo"), var(1)));
    }
    #[test]
    fn conditional_cond() {
        check(|var| Expr::conditional(var(0), Expr::literal(1), Expr::literal(2)));
    }
    #[test]
    fn conditional_then() {
        check(|var| Expr::conditional(Expr::literal("true"), var(1), Expr::literal(3)));
    }
    #[test]
    fn conditional_else() {
        check(|var| Expr::conditional(Expr::literal("false"), Expr::literal(4), var(2)));
    }
    #[test]
    fn conditional_all() {
        check(|var| Expr::conditional(var(0), var(1), var(2)));
    }
    #[test]
    fn switch() {
        check(|var| {
            Expr::switch(
                var(0),
                vec![
                    (Literal::Number(10.0).into(), None, Expr::literal(11)),
                    (Pattern::Any, None, Expr::literal(12)),
                ],
            )
        });
    }
}
