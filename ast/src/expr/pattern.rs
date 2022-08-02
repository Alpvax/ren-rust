use std::collections::HashSet;

use super::literal::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Any,
    Literal(Literal<Pattern>),
    Spread(String),
    Type(String, Box<Pattern>),
    Var(String),
}
#[allow(dead_code)] //XXX
impl Pattern {
    /// Check if a particular variable name is bound by a pattern. Names are bound
    /// no matter how deeply nested a pattern is, so this recursively checks any nested
    /// patterns too.
    fn binds(&self, name: &str) -> bool {
        match self {
            Self::Literal(Literal::Array(items)) => items.iter().any(|pat| pat.binds(name)),
            Self::Literal(Literal::Enum(_, args)) => args.iter().any(|pat| pat.binds(name)),
            Self::Literal(Literal::Record(fields)) => fields.iter().any(|(_, pat)| pat.binds(name)),
            Self::Spread(n) => n == name,
            Self::Type(_, pat) => pat.binds(name),
            Self::Var(n) => n == name,
            _ => false,
        }
    }

    fn bindings(&self) -> HashSet<String> {
        match self {
            Self::Literal(Literal::Array(items)) => {
                items.into_iter().flat_map(Self::bindings).collect()
            }
            Self::Literal(Literal::Enum(_, args)) => {
                args.into_iter().flat_map(Self::bindings).collect()
            }
            Self::Literal(Literal::Record(fields)) => fields
                .into_iter()
                .flat_map(|(_, pat)| pat.bindings())
                .collect(),
            Self::Type(_, pat) => pat.bindings(),
            Self::Spread(name) | Self::Var(name) => {
                let mut set = HashSet::new();
                set.insert(name.clone());
                set
            }
            _ => HashSet::new(),
        }
    }
}
impl From<Literal<Pattern>> for Pattern {
    fn from(l: Literal<Pattern>) -> Self {
        Self::Literal(l)
    }
}
