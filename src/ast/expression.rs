use std::collections::HashMap;

use crate::ast::Identifier;
use crate::value::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralNonContainer {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    ArrayDestructure(Vec<Pattern>),
    Name(String),
    ObjectDestructure(Vec<(String, Option<Pattern>)>),
    Value(LiteralNonContainer),
    Wildcard(Option<String>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    SubExpr(Box<Expression>),
    Accessor(Box<Expression>, Box<Accessor>),
    Application(Box<Expression>, Vec<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    Identifier(Identifier),
    Infix(Operator, Box<Expression>, Box<Expression>),
    Lambda(Vec<Pattern>, Box<Expression>),
    Literal(Literal),
}

impl Expression {
    pub fn value_type(&self) -> ValueType {
        match self {
            Expression::SubExpr(e) => e.value_type(),
            Expression::Accessor(o, key) => {
                if let ValueType::Object(t) = o.value_type() {
                    key.clone()
                        .unwrap()
                        .map(|s| t.get(&s).map(|v| v.clone()))
                        .flatten()
                        .unwrap_or(ValueType::Unknown)
                } else {
                    ValueType::Unknown
                }
            }
            Expression::Application(f, params) => {
                if let Some((p, r)) = f.value_type().function_type() {
                    if p.len() > params.len() {
                        //TODO: Check param types?
                        ValueType::function(
                            params[p.len()..]
                                .iter()
                                .map(Expression::value_type)
                                .collect(),
                            r,
                        )
                    } else {
                        ValueType::Unknown
                    }
                } else {
                    ValueType::Unknown
                }
            }
            Expression::Conditional(condition, t_res, f_res) => {
                if let (ValueType::Boolean, _t, _f) = (
                    condition.value_type(),
                    t_res.value_type(),
                    f_res.value_type(),
                ) {
                    todo!("Conditional type")
                } else {
                    ValueType::Unknown
                }
            }
            Expression::Identifier(_) => {
                todo!("Lookup identifier type")
            }
            Expression::Infix(op, a, b) => op.value_type(&a.value_type(), &b.value_type()),
            Expression::Lambda(params, _e) => {
                ValueType::function_n(params.len()) //TODO: ambda type: parse declaration
            }
            Expression::Literal(l) => match l {
                Literal::Boolean(_) => ValueType::Boolean,
                Literal::Number(_) => ValueType::Number,
                Literal::String(_) => ValueType::String,
                Literal::Array(a) => {
                    ValueType::Array(a.iter().map(|e| e.value_type().clone()).collect())
                }
                Literal::Object(o) => ValueType::Object(
                    o.iter()
                        .map(|(k, e)| (k.clone(), e.value_type().clone()))
                        .collect(),
                ),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Accessor {
    Computed(Expression),
    Fixed(String),
}
impl Accessor {
    pub fn unwrap(self) -> Option<String> {
        match self {
            Accessor::Computed(e) => {
                if ValueType::String == e.value_type() {
                    todo!("Expression.evaluate")
                } else {
                    None
                }
            }
            Accessor::Fixed(s) => Some(s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Pipe,    // a |> b   = b a
    Compose, // a >> b   = new func passing result of a into b
    Discard, // ;        = no-op, value to the left is ignored
    // MATHS
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^
    Mod, // %
    // COMPARISON
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    Lte,   // <=
    Gt,    // >
    Gte,   // >=
    // LOGIC
    And, // &&
    Or,  // ||
    // ARRAYS
    Cons, // a :: b   = [a, ...b]
    Join, // a ++ b   = [...a, ...b]
}
impl Operator {
    fn value_type(self, type_a: &ValueType, type_b: &ValueType) -> ValueType {
        match self {
            Operator::Pipe => {
                if let ValueType::Function(p, r) = type_b {
                    if **p == *type_a {
                        *r.clone()
                    } else {
                        ValueType::Unknown
                    }
                } else {
                    ValueType::Unknown
                }
            }
            Operator::Compose => {
                if let (Some((a_p, a_r)), Some((b_p, b_r))) =
                    (type_a.function_type(), type_b.function_type())
                {
                    if &a_r == &b_p[0] {
                        ValueType::function([a_p, b_p[1..].to_vec()].concat(), b_r)
                    } else {
                        ValueType::Unknown
                    }
                } else {
                    ValueType::Unknown
                }
            }
            Operator::Discard => ValueType::Undefined,
            Operator::Add
            | Operator::Sub
            | Operator::Mul
            | Operator::Div
            | Operator::Pow
            | Operator::Mod => ValueType::Number,
            Operator::Eq
            | Operator::NotEq
            | Operator::Lt
            | Operator::Lte
            | Operator::Gt
            | Operator::Gte
            | Operator::And
            | Operator::Or => ValueType::Boolean,
            Operator::Cons => {
                if let ValueType::Array(v) = type_b {
                    ValueType::Array([vec![type_a.clone()], v.clone()].concat())
                } else {
                    ValueType::Unknown
                }
            }
            Operator::Join => {
                if let (ValueType::Array(a), ValueType::Array(b)) = (type_a, type_b) {
                    ValueType::Array([a.clone(), b.clone()].concat())
                } else {
                    ValueType::Unknown
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Array(Vec<Expression>),
    Boolean(bool),
    Number(f64),
    Object(HashMap<String, Expression>),
    String(String),
}
