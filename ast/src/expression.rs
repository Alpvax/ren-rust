use std::{collections::HashMap, mem};

use crate::{
    declaration::{BlockDeclaration, Declaration},
    pattern::Pattern,
    Ident, Type,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Access(Box<Expression>, Vec<Ident>),
    Application(Box<Expression>, Vec<Expression>),
    Cast(Box<Expression>, Type),
    Block(Vec<BlockDeclaration>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    Identifier(Identifier), //TODO: Check
    Infix(Operator, Box<Expression>, Box<Expression>),
    Lambda(Vec<Pattern>, Box<Expression>),
    Literal(Literal),
    Match(
        Box<Expression>,
        Vec<(Pattern, Option<Expression>, Expression)>,
    ),
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Local(String),
    Scoped(Vec<String>, Box<Identifier>),
    Placeholder(Option<String>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Array(Vec<Expression>),
    Boolean(bool),
    Number(f64),
    Record(HashMap<String, Expression>),
    String(String),
    Template(Vec<TemplateSegment<Expression>>),
    Variant(String, Vec<Expression>),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum TemplateSegment<T> {
    Text(String),
    Expr(T),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Pipe,
    Compose,
    // MATHS
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    // COMPARISON
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
    // LOGIC
    And,
    Or,
    // ARRAYS
    Cons,
    Join,
}

impl Default for Expression {
    fn default() -> Self {
        Self::Literal(Literal::Undefined)
    }
}
impl Expression {
    pub fn references(&self, namespace: Option<&Vec<String>>, name: Option<&str>) -> bool {
        if namespace.is_none() && name.is_none() {
            return false;
        }
        let match_ref = |expr: &Expression| expr.references(namespace, name);
        match self {
            Expression::Access(expr, _accessors) => {
                expr.references(namespace, name)
                // || accessors.iter().any(|acc| {
                //     if let Accessor::Computed(expr) = acc {
                //         expr.references(namespace, name)
                //     } else {
                //         false
                //     }
                // })
            }
            Expression::Application(expr, args) => {
                expr.references(namespace, name) || args.iter().any(match_ref)
            }
            Expression::Cast(expr, _) => expr.references(namespace, name),
            Expression::Block(bindings, expr) => {
                expr.references(namespace, name)
                    || bindings.iter().any(|dec| dec.references(namespace, name))
            }
            Expression::Conditional(condition, t, f) => {
                condition.references(namespace, name)
                    || t.references(namespace, name)
                    || f.references(namespace, name)
            }
            Expression::Identifier(Identifier::Local(n)) => {
                if namespace.is_none() {
                    // Checked at top of function
                    n == name.unwrap()
                } else {
                    false
                }
            }
            Expression::Identifier(Identifier::Scoped(ns, n)) => {
                if let Some(namespace) = namespace {
                    ns == namespace
                        && name.map_or(true, |name| {
                            if let Identifier::Local(n) = &**n {
                                n == name
                            } else {
                                false
                            }
                        })
                } else {
                    false
                }
            }
            Expression::Identifier(_) => false,
            Expression::Infix(_, lhs, rhs) => {
                lhs.references(namespace, name) || rhs.references(namespace, name)
            }
            Expression::Lambda(_, expr) => expr.references(namespace, name),
            Expression::Literal(Literal::Array(elements)) => elements.iter().any(match_ref),
            Expression::Literal(Literal::Record(entries)) => entries.values().any(match_ref),
            Expression::Literal(Literal::Template(segments)) => segments.iter().any(|seg| {
                if let TemplateSegment::Expr(expr) = seg {
                    expr.references(namespace, name)
                } else {
                    false
                }
            }),
            Expression::Literal(_) => false,
            Expression::Match(expr, cases) => {
                expr.references(namespace, name)
                    || cases.iter().any(|(_, guard, expr)| {
                        expr.references(namespace, name)
                            || guard
                                .as_ref()
                                .map_or(false, |g| g.references(namespace, name))
                    })
            }
        }
    }
    pub fn add_declaration(&mut self, binding: BlockDeclaration) {
        if let Self::Block(bindings, _) = self {
            bindings.push(binding);
        } else {
            let body = Box::new(mem::take(self));
            *self = Self::Block(vec![binding], body);
        }
    }

    // ------------ Literal Constructor Helpers ----------------
    pub fn string<T: Into<String>>(s: T) -> Self {
        Self::Literal(Literal::String(s.into()))
    }
    pub fn number<T: Into<f64>>(num: T) -> Self {
        Self::Literal(Literal::Number(num.into()))
    }
    pub fn boolean<T: Into<bool>>(b: T) -> Self {
        Self::Literal(Literal::Boolean(b.into()))
    }
    pub fn undefined() -> Self {
        Self::Literal(Literal::Undefined)
    }
}

impl Literal {
    pub fn coerce_to_float(&self) -> Option<f64> {
        match self {
            Literal::Boolean(true) => Some(1.0),
            Literal::Boolean(false) => Some(0.0),
            Literal::Number(f) => Some(*f),
            Literal::String(s) => s.parse().ok(),
            Literal::Undefined => Some(0.0),
            Literal::Array(_)
            | Literal::Record(_)
            | Literal::Template(_)
            | Literal::Variant(_, _) => None,
        }
    }
    pub fn coerce_to_int(&self) -> Option<i64> {
        match self {
            Literal::Boolean(true) => Some(1),
            Literal::Boolean(false) => Some(0),
            Literal::Number(f) => {
                if f.trunc() == *f {
                    Some(*f as i64)
                } else {
                    None
                }
            }
            Literal::String(s) => s.parse().ok(),
            Literal::Undefined => Some(0),
            Literal::Array(_)
            | Literal::Record(_)
            | Literal::Template(_)
            | Literal::Variant(_, _) => None,
        }
    }
    pub fn coerce_to_str(&self) -> Option<String> {
        match self {
            Literal::Boolean(true) => Some("true".to_owned()),
            Literal::Boolean(false) => Some("false".to_owned()),
            Literal::Number(f) => Some(f.to_string()),
            Literal::String(s) => Some(s.clone()),
            Literal::Template(segments) => segments
                .iter()
                .try_fold(String::new(), |s, seg| match seg {
                    TemplateSegment::Text(t) => Ok(s + t),
                    TemplateSegment::Expr(Expression::Literal(l)) => {
                        l.coerce_to_str().map(|t| s + &t).ok_or(())
                    }
                    TemplateSegment::Expr(_) => Err(()),
                })
                .ok()
                .map(|s| s.clone()),
            Literal::Array(_)
            | Literal::Record(_)
            | Literal::Undefined
            | Literal::Variant(_, _) => None,
        }
    }
}
