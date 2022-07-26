use super::FromSyntaxElement;

pub struct Pattern {}

impl FromSyntaxElement for Pattern {
    fn from_token(token_type: crate::syntax::Token, token: super::SyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_node(context: crate::syntax::Context, node: crate::syntax::SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
