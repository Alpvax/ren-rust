use array_init::array_init;

use super::core;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Access(Box<Expr>, String),
    Binop(Box<Expr>, Operator, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Lambda(Vec<String>, Box<Expr>),
    Let(core::Pattern, Box<Expr>, Box<Expr>),
    Literal(core::Literal<Expr>),
    Placeholder,
    Scoped(Vec<String>, String),
    Switch(Box<Expr>, Vec<(core::Pattern, Option<Expr>, Expr)>),
    Var(String),
}
impl Default for Expr {
    fn default() -> Self {
        Self::Literal(core::Literal::LUnit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,    //    +
    And,    //    and
    Concat, // ++
    Cons,   //   ::
    Div,    //    /
    Eq,     //     ==
    Gte,    //    >=
    Gt,     //     >
    Lte,    //    <=
    Lt,     //     <
    Mod,    //    %
    Mul,    //    *
    Neq,    //    !=
    Or,     //     or
    Pipe,   //   |>
    Sub,    //    -
}
#[allow(dead_code)]//XXX
impl Operator {
    fn from_name(name: &str) -> Option<Self> {
        OPERATORS_FULL
            .iter()
            .find(|(_, op_name, _)| *op_name == name)
            .map(|(op, ..)| *op)
    }
    fn from_symbol(symbol: &str) -> Option<Self> {
        OPERATORS_FULL
            .iter()
            .find(|(.., sym)| *sym == symbol)
            .map(|(op, ..)| *op)
    }
    fn find_data(&self) -> [&'static str; 2] {
        OPERATORS_FULL
            .iter()
            .find(|(op, ..)| op == self)
            .map(|(_, name, sym)| [*name, *sym])
            .expect(&format!(
                "OPERATOR {:?} is not in {:?}",
                self, OPERATORS_FULL
            ))
    }
    fn name(&self) -> &'static str {
        self.find_data()[0]
    }
    fn symbol(&self) -> &'static str {
        self.find_data()[1]
    }
}

macro_rules! make_operator_constants {
    ($(($op:expr, $name:literal, $symbol:literal)),+ $(,)?) => {
        #[allow(dead_code)]//XXX
        /// This is guaranteed to be in the same order as `OPERATOR_NAMES` and `OPERATOR_SYMBOLS`
        static OPERATORS: &'static [Operator] = &[$($op),+];
        #[allow(dead_code)]//XXX
        /// This is guaranteed to be in the same order as `OPERATORS` and `OPERATOR_SYMBOLS`
        static OPERATOR_NAMES: &'static [&'static str] = &[$($name),+];
        #[allow(dead_code)]//XXX
        /// This is guaranteed to be in the same order as `OPERATORS` and `OPERATOR_NAMES`
        static OPERATOR_SYMBOLS: &'static [&'static str] = &[$($symbol),+];
        /// This is guaranteed to be in the same order as `OPERATORS`, `OPERATOR_NAMES` and `OPERATOR_SYMBOLS`
        static OPERATORS_FULL: &'static [(Operator, &'static str, &'static str)] = &[$(($op, $name, $symbol)),+];
    };
}
make_operator_constants![
    (Operator::Add, "add", "+"),
    (Operator::And, "and", "and"),
    (Operator::Concat, "concat", "++"),
    (Operator::Cons, "cons", "::"),
    (Operator::Div, "div", "/"),
    (Operator::Eq, "eq", "=="),
    (Operator::Gte, "gte", ">="),
    (Operator::Gt, "gt", ">"),
    (Operator::Lte, "lte", "<="),
    (Operator::Lt, "lt", "<"),
    (Operator::Mod, "mod", "%"),
    (Operator::Mul, "mul", "*"),
    (Operator::Neq, "neq", "!="),
    (Operator::Or, "or", "or"),
    (Operator::Pipe, "pipe", "|>"),
    (Operator::Sub, "sub", "-"),
];

// -- CONSTRUCTORS ----------------------------------------------------------------

// {-| Take an expression from our core 𝝺-calculus representation and raise it up
// to the higher-level `Expr` type. This will take some of the magical variables
// used and expand them into more useful forms. For example, binary operators are
// represented in the core as:

//     Expr
//         (EApp
//             (Expr
//                 (EApp
//                     (Expr
//                         (EApp
//                             (Expr (EVar "<binop>"))
//                             (Expr (ELit (LStr "add")))
//                         )
//                     )
//                     (Expr (EVar "x"))
//                 )
//             )
//             (Expr (EVar "y"))
//         )

// -}
pub fn raise(expr: core::Expr) -> Expr {
    use super::core::{ExprF::*, Literal::*};
    use Expr::*;
    core::fold(
        &|expr_f| match expr_f {
            EAbs(arg, fun) => match *fun {
                Lambda(mut args, body) => {
                    args.insert(0, arg);
                    Lambda(args, body)
                }
                f => Lambda(vec![arg], Box::new(f)),
            },
            EApp(boxed) => match *boxed {
                (Call(f, mut args), expr) => match (*f, &args[..]) {
                    (Var(ref s), [Literal(LStr(key))]) if s == "<access>" => {
                        Access(Box::new(expr), key.to_owned())
                    }
                    (Var(ref s), [Literal(LStr(op_str)), lhs]) if s == "<binop>" => {
                        match Operator::from_name(op_str) {
                            Some(op) => Binop(Box::new(lhs.to_owned()), op, Box::new(expr)),
                            None => Call(
                                Box::new(Literal(LStr(op_str.to_owned()))),
                                vec![lhs.to_owned(), expr],
                            ),
                        }
                    }
                    (Var(ref s), [cond, then_]) if s == "<if>" => If(
                        Box::new(cond.to_owned()),
                        Box::new(then_.to_owned()),
                        Box::new(expr),
                    ),
                    (fun, _) => {
                        args.push(expr);
                        Call(Box::new(fun), args)
                    }
                },
                (fun, arg) => Call(Box::new(fun), vec![arg]),
            },
            ELet(name, boxed) => {
                let (expr, body) = *boxed;
                Let(core::Pattern::PVar(name), Box::new(expr), Box::new(body))
            }
            ELit(l) => Literal(l),
            EVar(ref s) if s == "<placeholder>" => Placeholder,
            EVar(var) => {
                let mut names = var.split('$').map(|s| s.to_owned()).collect::<Vec<_>>();
                let name = names.pop();
                match name {
                    None => Placeholder,
                    Some(name) => {
                        if names.len() < 1 {
                            Var(name)
                        } else {
                            Scoped(names, name)
                        }
                    }
                }
            }
            EPat(expr, cases) => Switch(expr, cases),
        },
        expr,
    )
}

// -- MANIPULATIONS ---------------------------------------------------------------

pub fn replace_placeholders(expr: Expr) -> Expr {
    use Expr::*;
    /// Creates a valid JavaScript variable name from a placholder.
    fn name(i: usize) -> String {
        format!("$temp{}", i)
    }
    fn map_placeholders(exprs: Vec<Expr>, expr_factory: &dyn Fn(Vec<Expr>) -> Expr) -> Expr {
        let (names, args) = exprs.into_iter().enumerate().fold(
            (Vec::new(), Vec::new()),
            |(mut names, mut args), (i, e)| {
                args.push(if let Placeholder = e {
                    let name = name(i);
                    names.push(name.clone());
                    Var(name)
                } else {
                    e
                });
                (names, args)
            },
        );
        if names.len() > 0 {
            Lambda(names, Box::new(expr_factory(args)))
        } else {
            expr_factory(args)
        }
    }
    fn map_arr<const N: usize>(exprs: [Expr; N], expr_factory: &dyn Fn([Expr; N]) -> Expr) -> Expr {
        let (names, args) = IntoIterator::into_iter(exprs).enumerate().fold(
            (Vec::new(), array_init(|_| Expr::default())),
            |(mut names, mut args), (i, e)| {
                args[i] = if let Placeholder = e {
                    let name = name(i);
                    names.push(name.clone());
                    Var(name)
                } else {
                    e
                };
                (names, args)
            },
        );
        if names.len() > 0 {
            Lambda(names, Box::new(expr_factory(args)))
        } else {
            expr_factory(args)
        }
    }
    fn map_positional<const N: usize>(
        exprs: [(usize, Expr); N],
        expr_factory: &dyn Fn([Expr; N]) -> Expr,
    ) -> Expr {
        let (names, args) = IntoIterator::into_iter(exprs).enumerate().fold(
            (Vec::new(), array_init(|_| Expr::default())),
            |(mut names, mut args), (arg_index, (i, e))| {
                args[arg_index] = if let Placeholder = e {
                    let name = name(i);
                    names.push(name.clone());
                    Var(name)
                } else {
                    e
                };
                (names, args)
            },
        );
        if names.len() > 0 {
            Lambda(names, Box::new(expr_factory(args)))
        } else {
            expr_factory(args)
        }
    }
    match expr {
        Access(rec, key) if *rec == Placeholder => {
            Lambda(vec![name(0)], Box::new(Access(Box::new(Var(name(0))), key)))
        }
        Binop(lhs, op, rhs) => map_positional([(0, *lhs), (2, *rhs)], &|[lhs, rhs]| {
            Binop(Box::new(lhs), op, Box::new(rhs))
        }),
        Call(fun, args) => map_placeholders([vec![*fun], args].concat(), &|exprs| {
            let mut iter = exprs.into_iter();
            Call(Box::new(iter.next().unwrap()), iter.collect())
        }),
        If(cond, then_, else_) => map_arr([*cond, *then_, *else_], &|[cond, then_, else_]| {
            If(Box::new(cond), Box::new(then_), Box::new(else_))
        }),
        Switch(expr_, cases) if *expr_ == Placeholder => Lambda(
            vec![name(0)],
            Box::new(Switch(Box::new(Var(name(0))), cases)),
        ),
        _ => expr,
    }
}

// -- CONVERSIONS -----------------------------------------------------------------

// {-| Lower a Ren expression to a core representation based on the 𝝺-calculus. Some
// constructs are represented using special internal variables, for example:

//     Access (Var "foo") "bar"

// is represented as:

//     Expr
//         (EApp
//             (Expr
//                 (EApp
//                     (Expr (EVar "<access>"))
//                     (Expr (ELit (LStr "bar")))
//                 )
//             )
//             (Expr (EVar "foo"))
//         )

// This is the reverse of `raise`. In fact, you if you call `raise (lower expr)` you
// should get back exactly the same expression.

// -}
pub fn lower(expr: Expr) -> core::Expr {
    match replace_placeholders(expr) {
        Expr::Access(expr, key) => core::Expr::app(
            core::Expr::var("<access>".to_owned()),
            [core::Expr::str(key), lower(*expr)],
        ),
        Expr::Binop(lhs, op, rhs) => core::Expr::app(
            core::Expr::var("<binop>".to_owned()),
            [
                core::Expr::str(op.name().to_owned()),
                lower(*lhs),
                lower(*rhs),
            ],
        ),
        Expr::Call(fun, args) => core::Expr::app(lower(*fun), args.into_iter().map(lower)),
        Expr::If(cond, then_, else_) => core::Expr::app(
            core::Expr::var("<if>".to_owned()),
            [lower(*cond), lower(*then_), lower(*else_)],
        ),
        Expr::Lambda(args, body) => core::Expr::abs(args, lower(*body)),
        Expr::Let(core::Pattern::PVar(name), expr, body) => {
            core::Expr::let_([(name, lower(*expr))], lower(*body))
        }
        Expr::Let(pattern, expr, body) => {
            core::Expr::pattern(lower(*expr), vec![(pattern, None, lower(*body))])
        }
        Expr::Literal(core::Literal::LArr(elements)) => {
            core::Expr::arr(elements.into_iter().map(lower).collect())
        }
        Expr::Literal(core::Literal::LBool(b)) => core::Expr::bool(b),
        Expr::Literal(core::Literal::LCon(tag, args)) => {
            core::Expr::con(tag, args.into_iter().map(lower).collect())
        }
        Expr::Literal(core::Literal::LNum(n)) => core::Expr::num(n),
        Expr::Literal(core::Literal::LRec(fields)) => {
            core::Expr::rec(fields.into_iter().map(|(k, e)| (k, lower(e))).collect())
        }
        Expr::Literal(core::Literal::LStr(s)) => core::Expr::str(s),
        Expr::Literal(core::Literal::LUnit) => core::Expr::unit(),
        Expr::Placeholder => core::Expr::unit(),
        Expr::Scoped(scope, name) => core::Expr::var(format!("{}${}", scope.join("$"), name)),
        Expr::Switch(expr, cases) => core::Expr::pattern(
            lower(*expr),
            cases
                .into_iter()
                .map(|(pattern, guard, body)| (pattern, guard.map(lower), lower(body)))
                .collect(),
        ),
        Expr::Var(name) => core::Expr::var(name),
    }
}
