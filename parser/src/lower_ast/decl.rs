use smol_str::SmolStr;

use super::{
    expr::Expr, extensions::SyntaxNodeExtension, ren_type::Type, FromSyntaxElement, RangeLookup,
    SyntaxToken, ToHIR,
};
use crate::syntax::{Context, SyntaxNode, Token};

#[derive(Debug)]
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
    fn get_range(&self) -> rowan::TextRange {
        self.0.text_range()
    }
}
impl Decl {
    fn is_public(&self) -> bool {
        self.0.find_token(Token::KWPub).is_some()
    }
    fn is_local(&self) -> bool {
        self.0.find_token(Token::KWLet).is_some()
    }
    pub fn is_type(&self) -> bool {
        self.0.find_token(Token::KWType).is_some()
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
    fn type_annotation(&self) -> Option<Type> {
        self.0
            .find_node(Context::Type)
            .and_then(Type::from_root_node)
    }
}
impl ToHIR for Decl {
    type HIRType = higher_ast::Decl;
    type ValidationError = ();
    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        if self.is_type() {
            higher_ast::Decl::typ(
                self.type_annotation()
                    .map(|t| t.to_higher_ast(line_lookup))
                    .unwrap_or_default(),
                RangeLookup(line_lookup, self.0.text_range()),
                self.is_public(),
                self.0
                    .find_token(Token::Namespace)
                    .map(|tok| SmolStr::new(tok.text()))
                    .unwrap(),
            )
        } else if self.is_local() {
            higher_ast::Decl::local(
                self.type_annotation().map(|t| t.to_higher_ast(line_lookup)),
                RangeLookup(line_lookup, self.0.text_range()),
                self.is_public(),
                self.name().unwrap(),
                self.expr().unwrap().to_higher_ast(line_lookup),
            )
        } else {
            higher_ast::Decl::external(
                self.type_annotation().map(|t| t.to_higher_ast(line_lookup)),
                RangeLookup(line_lookup, self.0.text_range()),
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

// impl core::fmt::Debug for Decl {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.is_local() {
//             f.debug_struct("Decl")
//                 .field("name", &self.name())
//                 .field("public", &self.is_public())
//                 .field("type", &"local")
//                 .field("expr", &self.expr())
//                 .finish()
//         } else {
//             f.debug_struct("Decl")
//                 .field("name", &self.name())
//                 .field("public", &self.is_public())
//                 .field("type", &"external")
//                 .field("ext_name", &self.ext_name())
//                 .finish()
//         }
//     }
// }
