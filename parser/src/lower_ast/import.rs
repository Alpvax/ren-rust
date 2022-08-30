use higher_ast::module::import::Source;
use smol_str::SmolStr;

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    FromSyntaxElement, SyntaxToken, ToHIR,
};
use crate::syntax::{Context, SyntaxNode, SyntaxPart, Token};

pub struct Import(SyntaxNode);

impl FromSyntaxElement for Import {
    fn from_token(_token_type: Token, _token: SyntaxToken) -> Option<Self> {
        None
    }
    fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
        match context {
            Context::Import => Some(Self(node)),
            _ => None,
        }
    }
    fn from_root_node(node: SyntaxNode) -> Option<Self> {
        Self::from_node(Context::Import, node)
    }
}
impl Import {
    fn source(&self) -> Option<Source> {
        match self.0.child_tokens().skip_trivia().nth(1).unwrap().kind() {
            crate::syntax::SyntaxPart::Token(Token::KWPkg) => Some(Source::Package),
            crate::syntax::SyntaxPart::Token(Token::KWExt) => Some(Source::External),
            crate::syntax::SyntaxPart::Context(Context::String) => Some(Source::Local),
            _ => None,
        }
    }
    fn path(&self) -> Option<SmolStr> {
        self.0
            .find_node(Context::String)
            .and_then(super::simple_str)
    }
    fn name(&self) -> Vec<SmolStr> {
        self.0
            .find_node(Context::NameSpace)
            .map(|node| {
                node.child_tokens()
                    .filter_map(|tok| match tok.kind() {
                        SyntaxPart::Token(Token::Namespace) => Some(SmolStr::new(tok.text())),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn exposing(&self) -> Vec<SmolStr> {
        self.0
            .find_node(Context::ExposingBlock)
            .map(|node| {
                node.child_tokens()
                    .filter_map(|tok| match tok.kind() {
                        SyntaxPart::Token(Token::VarName) => Some(SmolStr::new(tok.text())),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl ToHIR for Import {
    type HIRType = higher_ast::Import;
    type ValidationError = ();
    fn to_higher_ast(&self) -> Self::HIRType {
        higher_ast::Import {
            path: self.path().map(|s| s.to_string()).unwrap(),
            source: self.source().unwrap(),
            name: self.name().into_iter().map(|s| s.to_string()).collect(),
            unqualified: self.exposing().into_iter().map(|s| s.to_string()).collect(),
        }
    }
    fn validate(&self) -> Option<Self::ValidationError> {
        todo!("Import::validate")
    }
}
