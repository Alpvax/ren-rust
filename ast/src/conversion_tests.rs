use crate::{
    core::{Expr, ExprF::*, Literal::*},
    expr::{Expr::*, Operator::*},
};

macro_rules! conversion_test {
    ($name:ident, $e:expr, $c:expr) => {
        mod $name {
            use super::*;
            #[test]
            fn lower() {
                assert_eq!(crate::expr::lower($e), $c);
            }
            #[test]
            fn raise() {
                assert_eq!(crate::expr::raise($c), $e);
            }
        }
    };
}

conversion_test!{ binop,
    Binop(Box::new(Literal(LNum( 3.0))), Add, Box::new(Literal (LNum( 2.0)))),
    Expr(EApp(Box::new((
        Expr(EApp(Box::new((
            Expr(EApp(Box::new((
                Expr(EVar("<binop>".to_owned())),
                Expr(ELit(LStr("add".to_owned()))),
            )))),
            (Expr(ELit(LNum(3.0)))),
        )))),
        (Expr(ELit(LNum (2.0))))
    ))))
}

conversion_test!{ access,
    Access(Box::new(Var("foo".to_owned())), "bar".to_owned()),
    Expr(EApp(Box::new((
        Expr(EApp(Box::new((
            Expr(EVar("<access>".to_owned())),
            Expr(ELit(LStr("bar".to_owned()))),
        )))),
        (Expr(EVar("foo".to_owned()))),
    ))))
}
