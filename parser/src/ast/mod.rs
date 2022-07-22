// mod into_elm_ast;

// pub(crate) use into_elm_ast::to_ast_expr;

mod expr;
mod extensions;
#[cfg(test)]
mod tests;
use crate::syntax::{Context, RenLang, SyntaxNode, SyntaxPart, Token};

// use expr::Expr;

type SyntaxElement = rowan::SyntaxElement<RenLang>;
type SyntaxToken = rowan::SyntaxToken<RenLang>;

trait FromSyntaxElement {
    fn from_token(token_type: Token, token: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
    fn from_node(context: Context, node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn from_element(element: SyntaxElement) -> Option<Self>
    where
        Self: Sized,
    {
        match element.kind() {
            SyntaxPart::Token(
                Token::Whitespace | Token::Comment | Token::ParenOpen | Token::Error,
            ) => None,
            SyntaxPart::Token(token_type) => {
                Self::from_token(token_type, element.into_token().unwrap())
            }
            SyntaxPart::Context(context) => Self::from_node(context, element.into_node().unwrap()),
            _ => None,
        }
    }
}

fn skip_trivia(node: &SyntaxElement) -> bool {
    if let SyntaxPart::Token(Token::Whitespace | Token::Comment) = node.kind() {
        false
    } else {
        true
    }
}

// pub trait ToHIR {
//     type HIRType;
//     type ValidationError;
//     fn to_higher_ast(&self) -> Self::HIRType;
//     fn validate(&self) -> Option<Self::ValidationError>;
// }
// pub enum ASTRoot {
//     Module(/*Module*/),
//     Expr(Expr),
// }
// impl Into<ASTRoot> for Module {
//     fn into(self) -> ASTRoot {
//         ASTRoot::Module(self)
//     }
// }
// impl Into<ASTRoot> for Expr {
//     fn into(self) -> ASTRoot {
//         ASTRoot::Expr(self)
//     }
// }
