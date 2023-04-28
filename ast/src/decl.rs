use serde::{Deserialize, Serialize};

use crate::{expr::Expr, ren_type::Type, Span};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "type")]
    typ: Type,
    inferred: bool,
    span: Span,
    comment: Vec<String>,
}
impl Meta {
    pub fn new<S>(type_annotation: Option<Type>, span: S) -> Self
    where
        S: Into<Span>,
    {
        let inferred = type_annotation.is_none();
        Self {
            typ: type_annotation.unwrap_or_default(),
            inferred,
            span: span.into(),
            comment: Default::default(),
        }
    }
    pub fn push_comment(&mut self, comment: String) {
        self.comment.push(comment);
    }
    pub fn set_span<S>(&mut self, span: S)
    where
        S: Into<Span>,
    {
        self.span = span.into();
    }
    pub fn get_type(&self) -> &Type {
        &self.typ
    }
}
impl Default for Meta {
    fn default() -> Self {
        Self {
            typ: Default::default(),
            inferred: true,
            span: Default::default(),
            comment: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Let {
        exposed: bool,
        var: String,
        typ: Type,
        expr: Expr,
    },
    Ext {
        exposed: bool,
        var: String,
        typ: Type,
        name: String,
    },
    Type {
        exposed: bool,
        name: String,
        vars: Vec<String>,
        typ: Type,
    },
}
impl Decl {
    // CONSTRUCTORS ============================================================
    pub fn local<N, S>(
        type_annotation: Option<Type>,
        _span: S,
        exposed: bool,
        var: N,
        expr: Expr,
    ) -> Self
    where
        N: ToString,
        S: Into<Span>,
    {
        Self::Let {
            exposed,
            var: var.to_string(),
            typ: type_annotation.unwrap_or_default(),
            expr,
        }
    }
    pub fn external<N, E, S>(
        type_annotation: Option<Type>,
        _span: S,
        exposed: bool,
        var: N,
        name: E,
    ) -> Self
    where
        N: ToString,
        E: ToString,
        S: Into<Span>,
    {
        Self::Ext {
            exposed,
            var: var.to_string(),
            typ: type_annotation.unwrap_or_default(),
            name: name.to_string(),
        }
    }
    pub fn typ<N, S>(type_annotation: Type, _span: S, exposed: bool, name: N) -> Self
    where
        N: ToString,
        S: Into<Span>,
    {
        Self::Type {
            exposed,
            name: name.to_string(),
            vars: Vec::new(),
            typ: type_annotation,
        }
    }

    // QUERIES ============================================================
    pub fn name(&self) -> &str {
        match self {
            Decl::Let { var, .. } | Decl::Ext { var, .. } | Decl::Type { name: var, .. } => var,
        }
    }
    pub fn is_exposed(&self) -> bool {
        match self {
            Decl::Let { exposed, .. } | Decl::Ext { exposed, .. } | Decl::Type { exposed, .. } => {
                *exposed
            }
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            Decl::Let { .. } | Decl::Type { .. } => true,
            Decl::Ext { .. } => false,
        }
    }
    pub fn is_external(&self) -> bool {
        match self {
            Decl::Let { .. } | Decl::Type { .. } => false,
            Decl::Ext { .. } => true,
        }
    }
}
