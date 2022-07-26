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
    fn find_node(&self, kind: crate::syntax::Context) -> Option<SyntaxNode>;
    fn find_token<T>(&self, kind: T) -> Option<SyntaxToken>
    where
        T: Into<crate::syntax::TokenType>;
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
    fn find_node(&self, kind: crate::syntax::Context) -> Option<SyntaxNode> {
        let kind = kind.into();
        self.children().find(|e| e.kind() == kind)
    }
    fn find_token<T>(&self, kind: T) -> Option<SyntaxToken>
    where
        T: Into<crate::syntax::TokenType>,
    {
        let kind = kind.into().into();
        self.child_tokens().find(|e| e.kind() == kind)
    }
}

pub(super) trait TokenTypeWrapper {
    fn is_trivia(&self) -> bool {
        use crate::syntax::{StringToken, SyntaxPart, Token};
        match self.token_type() {
            SyntaxPart::Token(
                Token::Whitespace | Token::Comment | Token::DoubleQuote | Token::ParenOpen,
            )
            | SyntaxPart::StringToken(StringToken::ExprStart | StringToken::Delimiter) => true,
            _ => false,
        }
    }
    fn is_not_trivia(&self) -> bool {
        !self.is_trivia()
    }
    fn token_type(&self) -> <RenLang as rowan::Language>::Kind;
    fn kind_matches<K>(&self, kind: K) -> bool
    where
        K: Into<<RenLang as rowan::Language>::Kind>,
    {
        self.token_type() == kind.into()
    }
}
impl TokenTypeWrapper for SyntaxElement {
    fn token_type(&self) -> <RenLang as rowan::Language>::Kind {
        self.kind()
    }
}
impl TokenTypeWrapper for SyntaxToken {
    fn token_type(&self) -> <RenLang as rowan::Language>::Kind {
        self.kind()
    }
}

pub(super) trait SyntaxIterator: Iterator + Sized {
    fn skip_trivia_filter(item: &Self::Item) -> bool;
    fn skip_trivia(self) -> std::iter::Filter<Self, fn(&Self::Item) -> bool> {
        self.filter(Self::skip_trivia_filter)
    }
}
impl<T, I> SyntaxIterator for T
where
    T: Iterator<Item = I>,
    I: TokenTypeWrapper,
{
    fn skip_trivia_filter(item: &Self::Item) -> bool {
        item.is_not_trivia()
    }
}
