use super::{
    decl::Decl, extensions::SyntaxNodeExtension, import::Import, FromSyntaxElement, SyntaxToken,
    ToHIR,
};
use crate::syntax::{Context, SyntaxNode, SyntaxPart, Token};

pub struct Module(SyntaxNode);

impl FromSyntaxElement for Module {
    fn from_token(_token_type: Token, _token: SyntaxToken) -> Option<Self> {
        None
    }
    fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
        match context {
            Context::Module => Some(Self(node)),
            _ => None,
        }
    }
    fn from_root_node(node: SyntaxNode) -> Option<Self> {
        Self::from_node(Context::Module, node)
    }
}
impl Module {
    fn imports(&self) -> Vec<Import> {
        self.0
            .find_node(Context::Imports)
            .map(|imports| {
                imports
                    .children()
                    .filter_map(|node| match node.kind() {
                        SyntaxPart::Context(Context::Import) => Import::from_root_node(node),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn decls(&self) -> Vec<Decl> {
        self.0
            .find_node(Context::Declarations)
            .map(|decls| {
                decls
                    .children()
                    .filter_map(|node| match node.kind() {
                        SyntaxPart::Context(Context::Declaration) => Decl::from_root_node(node),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
impl ToHIR for Module {
    type HIRType = higher_ast::Module;
    type ValidationError = ();
    fn to_higher_ast(&self) -> Self::HIRType {
        higher_ast::Module::new(
            Default::default(),
            self.imports().into_iter().map(|i| i.to_higher_ast()),
            self.decls().into_iter().map(|d| d.to_higher_ast()),
        )
    }
    fn validate(&self) -> Option<Self::ValidationError> {
        todo!("Module::validate")
    }
}
