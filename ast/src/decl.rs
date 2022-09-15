use ren_json_derive::RenJson;
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

#[derive(Debug, Clone, PartialEq, RenJson)]
pub enum Decl {
    Let(Meta, bool, String, Expr),
    Ext(Meta, bool, String, String),
}
impl Decl {
    // CONSTRUCTORS ============================================================
    pub fn local<N, S>(
        type_annotation: Option<Type>,
        span: S,
        public: bool,
        name: N,
        expr: Expr,
    ) -> Self
    where
        N: ToString,
        S: Into<Span>,
    {
        Self::Let(
            Meta::new(type_annotation, span),
            public,
            name.to_string(),
            expr,
        )
    }
    pub fn external<N, E, S>(
        type_annotation: Option<Type>,
        span: S,
        public: bool,
        name: N,
        ext_name: E,
    ) -> Self
    where
        N: ToString,
        E: ToString,
        S: Into<Span>,
    {
        Self::Ext(
            Meta::new(type_annotation, span),
            public,
            name.to_string(),
            ext_name.to_string(),
        )
    }

    // QUERIES ============================================================
    pub fn name(&self) -> &str {
        match self {
            Decl::Let(_, _, name, _) | Decl::Ext(_, _, name, _) => name,
        }
    }
    pub fn is_public(&self) -> bool {
        match self {
            Decl::Let(_, public, _, _) | Decl::Ext(_, public, _, _) => *public,
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            Decl::Let(..) => true,
            Decl::Ext(..) => false,
        }
    }
    pub fn is_external(&self) -> bool {
        match self {
            Decl::Let(..) => false,
            Decl::Ext(..) => true,
        }
    }
}
