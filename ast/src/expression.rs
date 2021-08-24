use std::{collections::HashMap, mem};

use crate::{declaration::Declaration, VarName};

#[derive(Debug, Clone)]
pub enum Expression {
    Access(Box<Expression>, Vec<Accessor>),
    Application(Box<Expression>, Vec<Expression>),
    Block(Vec<Declaration>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    Identifier(Identifier),
    Infix(Operator, Box<Expression>, Box<Expression>),
    Lambda(Vec<Pattern>, Box<Expression>),
    Literal(Literal),
    Match(
        Box<Expression>,
        Vec<(Pattern, Option<Expression>, Expression)>,
    ),
    SubExpression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Accessor {
    Computed(Expression),
    Fixed(String),
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Local(String),
    Constructor(String),
    Scoped(Vec<String>, Box<Identifier>),
    Operator(Operator),
    Field(String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Array(Vec<Expression>),
    Boolean(bool),
    Number(f64),
    Object(HashMap<String, Expression>),
    String(String),
    Template(Vec<TemplateSegment>),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum TemplateSegment {
    Text(String),
    Expr(Expression),
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
            Expression::Access(expr, accessors) => {
                expr.references(namespace, name)
                    || accessors.iter().any(|acc| {
                        if let Accessor::Computed(expr) = acc {
                            expr.references(namespace, name)
                        } else {
                            false
                        }
                    })
            }
            Expression::Application(expr, args) => {
                expr.references(namespace, name) || args.iter().any(match_ref)
            }
            Expression::Block(bindings, expr) => {
                expr.references(namespace, name)
                    || bindings.iter().any(|dec| dec.references(namespace, name))
            }
            Expression::Conditional(condition, t, f) => {
                condition.references(namespace, name)
                    || t.references(namespace, name)
                    || f.references(namespace, name)
            }
            Expression::Identifier(Identifier::Local(n))
            | Expression::Identifier(Identifier::Constructor(n)) => {
                if namespace.is_none() {
                    // Checked at top of function
                    n == name.unwrap()
                } else {
                    false
                }
            }
            Expression::Identifier(Identifier::Scoped(ns, n)) => {
                if let Some(namespace) = namespace {
                    ns == namespace && name.map_or(true, |name| if let Identifier::Local(n) | Identifier::Constructor(n) = &**n {
                        n == name
                    } else {false})
                } else {
                    false
                }
            },
            Expression::Identifier(_) => false,
            Expression::Infix(_, lhs, rhs) => {
                lhs.references(namespace, name) || rhs.references(namespace, name)
            }
            Expression::Lambda(_, expr) => expr.references(namespace, name),
            Expression::Literal(Literal::Array(elements)) => elements.iter().any(match_ref),
            Expression::Literal(Literal::Object(entries)) => entries.values().any(match_ref),
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
            Expression::SubExpression(expr) => expr.references(namespace, name),
        }
    }
    pub fn add_declaration(&mut self, binding: Declaration) {
        if let Self::Block(bindings, _) = self {
            bindings.push(binding);
        } else {
            let body = Box::new(mem::take(self));
            *self = Self::Block(vec![binding], body);
        }
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
            Literal::Array(_) | Literal::Object(_) | Literal::Template(_) => None,
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
            Literal::Array(_) | Literal::Object(_) | Literal::Template(_) => None,
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
            Literal::Array(_) | Literal::Object(_) | Literal::Undefined => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    ArrayDestructure(Vec<Pattern>),
    Name(String),
    ObjectDestructure(Vec<(String, Option<Pattern>)>),
    Value(Literal),
    VariantDestructure(String, Vec<Pattern>),
    Wildcard(Option<String>),
}
impl Pattern {
    pub fn names(&self) -> Vec<VarName> {
        match self {
            Pattern::ArrayDestructure(patterns) => {
                patterns.iter().map(Pattern::names).flatten().collect()
            }
            Pattern::Name(n) => vec![n.clone()],
            Pattern::ObjectDestructure(m) => m.iter().fold(Vec::new(), |mut v, (k, p)| {
                v.extend(p.as_ref().map_or_else(|| vec![k.clone()], Pattern::names));
                v
            }),
            Pattern::VariantDestructure(tag, patterns) => {
                patterns.iter().fold(vec![tag.clone()], |mut v, pat| {
                    v.extend(pat.names());
                    v
                })
            }
            Pattern::Value(_) | Pattern::Wildcard(_) => Vec::new(),
        }
    }
}
