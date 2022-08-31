use smol_str::SmolStr;

use super::{expr::Expr, extensions::SyntaxNodeExtension, FromSyntaxElement, SyntaxToken, ToHIR};
use crate::syntax::{Context, SyntaxNode, Token};

pub struct Decl(SyntaxNode);

impl FromSyntaxElement for Decl {
    fn from_token(_token_type: Token, _token: SyntaxToken) -> Option<Self> {
        None
    }
    fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
        match context {
            Context::Declaration => Some(Self(node)),
            _ => None,
        }
    }
    fn from_root_node(node: SyntaxNode) -> Option<Self> {
        Self::from_node(Context::Declaration, node)
    }
}
impl Decl {
    fn is_public(&self) -> bool {
        self.0.find_token(Token::KWPub).is_some()
    }
    fn is_local(&self) -> bool {
        self.0.find_token(Token::KWLet).is_some()
    }
    fn name(&self) -> Option<SmolStr> {
        self.0
            .find_token(Token::VarName)
            .map(|tok| SmolStr::new(tok.text()))
    }
    fn expr(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Expr)
            .and_then(Expr::from_root_node)
    }
    fn ext_name(&self) -> Option<SmolStr> {
        if self.is_local() {
            None
        } else {
            self.0
                .find_node(Context::String)
                .and_then(super::simple_str)
        }
    }
}
impl ToHIR for Decl {
    type HIRType = higher_ast::Decl;
    type ValidationError = ();
    fn to_higher_ast(&self) -> Self::HIRType {
        if self.is_local() {
            higher_ast::Decl::local(
                Default::default(),
                self.is_public(),
                self.name().unwrap(),
                self.expr().unwrap().to_higher_ast(),
            )
        } else {
            higher_ast::Decl::external(
                Default::default(),
                self.is_public(),
                self.name().unwrap(),
                self.ext_name().unwrap(),
            )
        }
    }
    fn validate(&self) -> Option<Self::ValidationError> {
        todo!("Decl::validate")
    }
}
