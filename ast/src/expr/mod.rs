use array_init::array_init;
use ren_json_derive::RenJson;
use serde::{Deserialize, Serialize};

use crate::ren_type::Type;
use crate::span::Span;

pub mod literal;
pub mod operator;
pub mod pattern;
// mod pattern_expanded;
// pub use pattern_expanded::pattern;
#[cfg(test)]
mod tests;

pub use literal::Literal;
pub use literal::StringPart;
pub use operator::Operator;
pub use pattern::Pattern;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "type")]
    typ: Type,
    span: Span,
    comment: Vec<String>,
}
impl Meta {
    pub fn push_comment(&mut self, comment: String) {
        self.comment.push(comment);
    }
    pub fn set_span<S>(&mut self, span: S)
    where
        S: Into<Span>,
    {
        self.span = span.into();
    }
}

#[derive(Debug, Clone, PartialEq, RenJson)]
pub enum Expr {
    Access(Meta, Box<Expr>, String),
    Annotated(Meta, Box<Expr>, Type),
    Binop(Meta, Box<Expr>, Operator, Box<Expr>),
    Call(Meta, Box<Expr>, Vec<Expr>),
    If(Meta, Box<Expr>, Box<Expr>, Box<Expr>),
    Lambda(Meta, Vec<Pattern>, Box<Expr>),
    Let(Meta, Pattern, Box<Expr>, Box<Expr>),
    #[ren_json(tag = "Lit")]
    Literal(Meta, Literal<Expr>),
    Placeholder(Meta),
    Scoped(Meta, Vec<String>, String),
    Switch(Meta, Box<Expr>, Vec<(Pattern, Option<Expr>, Expr)>),
    Var(Meta, String),
}
impl<T: Into<Literal<Expr>>> From<T> for Expr {
    fn from(l: T) -> Self {
        Self::Literal(Meta::default(), l.into())
    }
}
impl Default for Expr {
    fn default() -> Self {
        Self::literal(())
    }
}
impl crate::ASTLiteralType for Expr {}

impl Expr {
    // fn meta(&self) -> Meta {
    //     match self {
    //         Expr::Access(meta, _, _) => meta.clone(),
    //         Expr::Annotated(meta, _, _) => meta.clone(),
    //         Expr::Binop(meta, _, _, _) => meta.clone(),
    //         Expr::Call(meta, _, _) => meta.clone(),
    //         Expr::If(meta, _, _, _) => meta.clone(),
    //         Expr::Lambda(meta, _, _) => meta.clone(),
    //         Expr::Let(meta, _, _, _) => meta.clone(),
    //         Expr::Literal(meta, _) => meta.clone(),
    //         Expr::Placeholder(meta) => meta.clone(),
    //         Expr::Scoped(meta, _, _) => meta.clone(),
    //         Expr::Switch(meta, _, _) => meta.clone(),
    //         Expr::Var(meta, _) => meta.clone(),
    //     }
    // }
    fn meta_mut(&mut self) -> &mut Meta {
        match self {
            Expr::Access(meta, _, _) => meta,
            Expr::Annotated(meta, _, _) => meta,
            Expr::Binop(meta, _, _, _) => meta,
            Expr::Call(meta, _, _) => meta,
            Expr::If(meta, _, _, _) => meta,
            Expr::Lambda(meta, _, _) => meta,
            Expr::Let(meta, _, _, _) => meta,
            Expr::Literal(meta, _) => meta,
            Expr::Placeholder(meta) => meta,
            Expr::Scoped(meta, _, _) => meta,
            Expr::Switch(meta, _, _) => meta,
            Expr::Var(meta, _) => meta,
        }
    }
    // #[must_use]
    // pub fn with_meta_values<T, S, C>(mut self, typ: Option<T>, span: Option<S>, comments: C) -> Self where T: Into<Type>, S: Into<Span>, C: IntoIterator, C::Item: ToString {
    //     let meta = self.meta_mut();
    //     if let Some(t) = typ {
    //         meta.typ = t.into();
    //     }
    //     if let Some(s) = span {
    //         meta.span = s.into();
    //     }
    //     meta.comment.extend(comments.into_iter().map(|s| s.to_string()));
    //     self
    // }
    #[must_use]
    pub fn with_span<S>(mut self, span: S) -> Self
    where
        S: Into<Span>,
    {
        let meta = self.meta_mut();
        meta.span = span.into();
        self
    }
    //TODO fn references(&self) -> ...
    //TODO fn shadows(&self) -> ...
    fn is_placeholder(&self) -> bool {
        if let Self::Placeholder(_) = self {
            true
        } else {
            false
        }
    }
    #[allow(dead_code)] //XXX
    fn replace_placeholders(self) -> Self {
        use Expr::*;
        /// Creates a valid JavaScript variable name from a placholder.
        fn name(i: usize) -> String {
            format!("$temp{}", i)
        }
        fn map_placeholders(exprs: Vec<Expr>, expr_factory: &dyn Fn(Vec<Expr>) -> Expr) -> Expr {
            let (names, args) = exprs.into_iter().enumerate().fold(
                (Vec::new(), Vec::new()),
                |(mut names, mut args), (i, e)| {
                    args.push(if let Placeholder(meta) = e {
                        let name = name(i);
                        names.push(Pattern::Var(name.clone()));
                        Var(meta, name)
                    } else {
                        e
                    });
                    (names, args)
                },
            );
            if names.len() > 0 {
                Expr::lambda(names, expr_factory(args))
            } else {
                expr_factory(args)
            }
        }
        fn map_arr<const N: usize>(
            exprs: [Expr; N],
            expr_factory: &dyn Fn([Expr; N]) -> Expr,
        ) -> Expr {
            let (names, args) = IntoIterator::into_iter(exprs).enumerate().fold(
                (Vec::new(), array_init(|_| Expr::default())),
                |(mut names, mut args), (i, e)| {
                    args[i] = if let Placeholder(meta) = e {
                        let name = name(i);
                        names.push(Pattern::Var(name.clone()));
                        Var(meta, name)
                    } else {
                        e
                    };
                    (names, args)
                },
            );
            if names.len() > 0 {
                Expr::lambda(names, expr_factory(args))
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
                    args[arg_index] = if let Placeholder(meta) = e {
                        let name = name(i);
                        names.push(Pattern::Var(name.clone()));
                        Var(meta, name)
                    } else {
                        e
                    };
                    (names, args)
                },
            );
            if names.len() > 0 {
                Expr::lambda(names, expr_factory(args))
            } else {
                expr_factory(args)
            }
        }
        match self {
            Access(meta, rec, key) if rec.is_placeholder() => Lambda(
                meta,
                vec![Pattern::Var(name(0))],
                Box::new(Expr::access(Expr::var(name(0)), key)),
            ),
            Binop(_, lhs, op, rhs) => map_positional([(0, *lhs), (2, *rhs)], &|[lhs, rhs]| {
                Expr::binop(lhs, op, rhs)
            }),
            Call(_, fun, args) => map_placeholders([vec![*fun], args].concat(), &|exprs| {
                let mut iter = exprs.into_iter();
                Expr::apply_many(iter.next().unwrap(), iter)
            }),
            If(_, cond, then_, else_) => {
                map_arr([*cond, *then_, *else_], &|[cond, then_, else_]| {
                    Expr::conditional(cond, then_, else_)
                })
            }
            Switch(meta, expr_, cases) if expr_.is_placeholder() => Lambda(
                meta,
                vec![Pattern::Var(name(0))],
                Box::new(Expr::switch(Expr::var(name(0)), cases)),
            ),
            _ => self,
        }
    }

    // CONSTRUCTORS ============================================================
    pub fn access<S: ToString>(obj: Expr, key: S) -> Self {
        Self::Access(Meta::default(), Box::new(obj), key.to_string())
    }
    pub fn annotated() -> Self {
        todo!()
    }
    pub fn binop(lhs: Self, op: Operator, rhs: Self) -> Self {
        Self::Binop(Meta::default(), Box::new(lhs), op, Box::new(rhs))
    }
    pub fn apply(func: Expr, arg: Expr) -> Self {
        if let Self::Call(meta, f, mut args) = func {
            args.push(arg);
            Self::Call(meta, f, args)
        } else {
            Self::Call(Meta::default(), Box::new(func), vec![arg])
        }
    }
    pub fn apply_many<T>(func: Expr, args: T) -> Self
    where
        T: IntoIterator<Item = Expr>,
    {
        if let Self::Call(meta, f, mut prev) = func {
            prev.extend(args);
            Self::Call(meta, f, prev)
        } else {
            Self::Call(Meta::default(), Box::new(func), args.into_iter().collect())
        }
    }
    pub fn conditional(condition: Self, then_: Self, else_: Self) -> Self {
        Self::If(
            Meta::default(),
            Box::new(condition),
            Box::new(then_),
            Box::new(else_),
        )
    }
    pub fn lambda(params: Vec<Pattern>, body: Expr) -> Self {
        Self::Lambda(Meta::default(), params, Box::new(body))
    }
    pub fn binding(pattern: Pattern, binding_value: Expr, body: Expr) -> Self {
        Self::Let(
            Meta::default(),
            pattern,
            Box::new(binding_value),
            Box::new(body),
        )
    }
    pub fn literal<V>(value: V) -> Self
    where
        V: Into<Literal<Expr>>,
    {
        Self::Literal(Meta::default(), value.into())
    }
    pub fn placeholder() -> Self {
        Self::Placeholder(Meta::default())
    }
    pub fn scoped(namespace: Vec<String>, var: String) -> Self {
        Self::Scoped(Meta::default(), namespace, var)
    }
    pub fn switch(expr: Expr, arms: Vec<(Pattern, Option<Expr>, Expr)>) -> Self {
        Self::Switch(Meta::default(), Box::new(expr), arms)
    }
    pub fn var<S: ToString>(name: S) -> Self {
        Self::Var(Meta::default(), name.to_string())
    }
}
