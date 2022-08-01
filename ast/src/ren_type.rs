use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// any type, e.g. "*"
    Any,
    /// type application, e.g. "Array a"
    App(Box<Type>, Vec<Type>),
    /// concrete type constructor, e.g. "Number"
    Con(String),
    /// function type, e.g. "Number -> Number"
    Fun(Box<Type>, Box<Type>),
    /// unknown (to the user) type, e.g. "?"
    Hole,
    /// record type, e.g. "{x: Number, y: Number}"
    Rec(Row),
    /// sum type, e.g. "#ok a | #err e"
    Sum(Row),
    /// type variable, e.g. "a"
    Var(String),
}
impl Default for Type {
    fn default() -> Self {
        Self::Hole
    }
}

type Row = HashMap<String, Vec<Type>>;
