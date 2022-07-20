#[derive(Debug, PartialEq)]
pub struct Expr(pub ExprF<Expr>);

#[derive(Debug, PartialEq)]
pub enum ExprF<T> {
    EAbs(String, Box<T>),
    EApp(Box<(T, T)>),
    ELet(String, Box<(T, T)>),
    ELit(Literal<T>),
    EVar(String),
    EPat(Box<T>, Vec<(Pattern, Option<T>, T)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    PAny,
    PLit(Literal<Pattern>),
    PTyp(String, Box<Pattern>),
    PVar(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<T> {
    LArr(Vec<T>),
    LBool(bool),
    LCon(String, Vec<T>),
    LNum(f64),
    LRec(Vec<(String, T)>),
    LStr(String),
    LUnit,
}

use ExprF::*;
// use Pattern::*;
use Literal::*;

impl Expr {
    pub fn abs<T>(args: T, body: Self) -> Self
    where
        T: IntoIterator<Item = String>,
        T::IntoIter: DoubleEndedIterator,
    {
        args.into_iter()
            .rfold(body, |e, arg| Self(EAbs(arg, Box::new(e))))
    }

    pub fn app<T>(fun: Self, args: T) -> Self
    where
        T: IntoIterator<Item = Self>,
    {
        args.into_iter()
            .fold(fun, |e, arg| Self(EApp(Box::new((e, arg)))))
    }

    pub fn let_<T>(bindings: T, body: Self) -> Self
    where
        T: IntoIterator<Item = (String, Self)>,
        T::IntoIter: DoubleEndedIterator,
    {
        bindings.into_iter().rfold(body, |e, (pattern, expr)| {
            Self(ELet(pattern, Box::new((expr, e))))
        })
    }

    pub fn arr(elements: Vec<Self>) -> Self {
        Self(ELit(LArr(elements)))
    }

    pub fn bool(b: bool) -> Self {
        Self(ELit(LBool(b)))
    }

    pub fn con(tag: String, args: Vec<Self>) -> Self {
        Self(ELit(LCon(tag, args)))
    }

    pub fn num(n: f64) -> Self {
        Self(ELit(LNum(n)))
    }

    pub fn rec(fields: Vec<(String, Self)>) -> Self {
        Self(ELit(LRec(fields)))
    }

    pub fn str(s: String) -> Self {
        Self(ELit(LStr(s)))
    }

    pub fn unit() -> Self {
        Self(ELit(LUnit))
    }

    pub fn var(name: String) -> Self {
        Self(EVar(name))
    }

    pub fn pattern(expr: Self, cases: Vec<(Pattern, Option<Self>, Self)>) -> Self {
        if cases.len() < 1 {
            expr
        } else {
            Self(EPat(Box::new(expr), cases))
        }
    }
}

// -- MANIPULATIONS --------------------------------------------------------

// map : (a -> b) -> ExprF a -> ExprF b
pub fn map<T, U>(f: &dyn Fn(T) -> U, expr_f: ExprF<T>) -> ExprF<U> {
    match expr_f {
        EAbs(pattern, expr) => EAbs(pattern, Box::new(f(*expr))),
        EApp(b) => {
            let (fun, arg) = *b;
            EApp(Box::new((f(fun), f(arg))))
        }
        ELet(pattern, b) => {
            let (expr, body) = *b;
            ELet(pattern, Box::new((f(expr), f(body))))
        }
        ELit(LArr(elements)) => ELit(LArr(elements.into_iter().map(f).collect())),
        ELit(LBool(b)) => ELit(LBool(b)),
        ELit(LCon(tag, args)) => ELit(LCon(tag, args.into_iter().map(f).collect())),
        ELit(LNum(n)) => ELit(LNum(n)),
        ELit(LRec(fields)) => ELit(LRec(fields.into_iter().map(|(k, v)| (k, f(v))).collect())),
        ELit(LStr(s)) => ELit(LStr(s)),
        ELit(LUnit) => ELit(LUnit),
        EVar(name) => EVar(name),
        EPat(expr, cases) => EPat(
            Box::new(f(*expr)),
            cases
                .into_iter()
                .map(|(pattern, guard, body)| (pattern, guard.map(f), f(body)))
                .collect(),
        ),
    }
}

// fold : (ExprF a -> a) -> Expr -> a
//     exprF |> map (fold f) |> f
pub fn fold<T>(f: &dyn Fn(ExprF<T>) -> T, Expr(expr_f): Expr) -> T {
    f(map(&|e| fold(f, e), expr_f))
}

// foldWith : (Expr -> ExprF a -> a) -> Expr -> a
// foldWith f ((Expr exprF) as expr) =
//     exprF |> map (foldWith f) |> f expr
// pub fn foldWith<T>(f: &dyn Fn(Expr, ExprF<T>) -> T, Expr(expr_f): Expr) -> T {
//     f(Expr(expr_f), map(foldWith(f,), expr_f))
// }

// unfold : (a -> ExprF a) -> a -> Expr
// unfold f a =
//     f a |> map (unfold f) |> Expr
// pub fn unfold<T>(f: &dyn Fn(T) -> ExprF<T>, t: T) -> Expr {
//     Expr(f(map(unfold(f, t))))
// }

// impl <T> From<Literal<T>> for ExprF<T> {
//     fn from(l: Literal<T>) -> Self {
//         ELit(l)
//     }
// }

// impl From<Literal<Pattern>> for Pattern {
//     fn from(l: Literal<Pattern>) -> Self {
//         PLit(l)
//     }
// }

impl std::iter::FromIterator<Expr> for Expr {
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        Expr(ELit(LArr(iter.into_iter().collect())))
    }
}

impl<T> From<bool> for Literal<T> {
    fn from(b: bool) -> Self {
        LBool(b)
    }
}
impl<T> From<f64> for Literal<T> {
    fn from(n: f64) -> Self {
        LNum(n)
    }
}
// Utility to help with not requiring i.0 suffix when creating
impl<T> From<i32> for Literal<T> {
    fn from(n: i32) -> Self {
        LNum(n.into())
    }
}
impl<T> From<String> for Literal<T> {
    fn from(s: String) -> Self {
        LStr(s)
    }
}
impl<T> From<&str> for Literal<T> {
    fn from(s: &str) -> Self {
        LStr(s.to_owned())
    }
}
impl<T> From<()> for Literal<T> {
    fn from(_: ()) -> Self {
        LUnit
    }
}
impl<T: Into<Literal<Expr>>> From<T> for Expr {
    fn from(l: T) -> Self {
        Expr(ELit(l.into()))
    }
}
