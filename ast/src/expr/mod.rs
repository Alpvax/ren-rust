use crate::ren_type::Type;

pub mod literal;
pub mod operator;
pub mod pattern;

pub use literal::Literal;
pub use operator::Operator;
pub use pattern::Pattern;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Meta {
    typ: Type,
    span: (),
    comment: Vec<String>,
}
impl Meta {
    pub fn push_comment(&mut self, comment: String) {
        self.comment.push(comment);
    }
    pub fn set_span(&mut self, span: ()) {
        self.span = span;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Access(Meta, Box<Expr>, String),
    Annotated(Meta, Box<Expr>, Type),
    Binop(Meta, Box<Expr>, Operator, Box<Expr>),
    Call(Meta, Box<Expr>, Vec<Expr>),
    If(Meta, Box<Expr>, Box<Expr>, Box<Expr>),
    Lambda(Meta, Vec<Pattern>, Box<Expr>),
    Let(Meta, Pattern, Box<Expr>, Box<Expr>),
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
    //TODO fn references(&self) -> ...
    //TODO fn shadows(&self) -> ...

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
    pub fn var(name: String) -> Self {
        Self::Var(Meta::default(), name)
    }
}