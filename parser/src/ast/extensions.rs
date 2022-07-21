use std::iter::FilterMap;

use crate::syntax::RenLang;

use super::{FromSyntaxElement, SyntaxElement, SyntaxNode, SyntaxToken};

pub(super) trait SyntaxNodeExtension {
    fn child_tokens(
        &self,
    ) -> FilterMap<rowan::SyntaxElementChildren<RenLang>, fn(SyntaxElement) -> Option<SyntaxToken>>;
    fn map_children<T>(
        &self,
    ) -> FilterMap<rowan::SyntaxElementChildren<RenLang>, fn(SyntaxElement) -> Option<T>>
    where
        T: FromSyntaxElement;
}
impl SyntaxNodeExtension for SyntaxNode {
    fn child_tokens(
        &self,
    ) -> FilterMap<rowan::SyntaxElementChildren<RenLang>, fn(SyntaxElement) -> Option<SyntaxToken>>
    {
        self.children_with_tokens()
            .filter_map(SyntaxElement::into_token)
    }
    fn map_children<T>(
        &self,
    ) -> FilterMap<rowan::SyntaxElementChildren<RenLang>, fn(SyntaxElement) -> Option<T>>
    where
        T: FromSyntaxElement,
    {
        self.children_with_tokens().filter_map(T::from_element)
    }
}
