use ren_json_derive::RenJson;
use serde::{Deserialize, Serialize};

use crate::{expr::Expr, ren_type::Type, Span};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "type")]
    typ: Type,
    inferred: bool,
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
pub enum Decl {
    Let(Meta, bool, String, Expr),
    Ext(Meta, bool, String, String),
}
impl Decl {
    // CONSTRUCTORS ============================================================
    pub fn local<N>(meta: Meta, public: bool, name: N, expr: Expr) -> Self
    where
        N: ToString,
    {
        Self::Let(meta, public, name.to_string(), expr)
    }
    pub fn external<N, E>(meta: Meta, public: bool, name: N, ext_name: E) -> Self
    where
        N: ToString,
        E: ToString,
    {
        Self::Ext(meta, public, name.to_string(), ext_name.to_string())
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
