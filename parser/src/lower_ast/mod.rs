mod decl;
mod expr;
mod extensions;
mod import;
mod literal;
mod macro_impl;
mod module;
mod pattern;

pub use decl::Decl;
pub use expr::Expr;
pub use import::Import;
pub use module::Module;
#[cfg(test)]
mod tests;

use crate::syntax::{Context, RenLang, SyntaxNode, SyntaxPart, Token};

// use expr::Expr;

type SyntaxElement = rowan::SyntaxElement<RenLang>;
type SyntaxToken = rowan::SyntaxToken<RenLang>;

pub trait FromSyntaxElement {
    fn from_token(token_type: Token, token: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
    fn from_node(context: Context, node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn from_root_node(node: SyntaxNode) -> Option<Self>
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

macro_rules! ast_funcs {
    ($($fn_name:ident: $ctx:expr => $typ:ty),+ $(,)?) => {
        $(
            #[allow(dead_code)]
            pub(crate) fn $fn_name(node: SyntaxNode) -> Option<$typ> {
                <$typ>::from_node($ctx, node)
            }
        )+
    };
}

ast_funcs! {
    expr_ast: Context::Expr => expr::Expr,
    decl_ast: Context::Declaration => decl::Decl,
    import_ast: Context::Import => import::Import,
    module_ast: Context::Module => module::Module,
}

pub trait ToHIR {
    type HIRType;
    type ValidationError;
    fn to_higher_ast(&self) -> Self::HIRType;
    fn validate(&self) -> Option<Self::ValidationError>;
}
impl<T> ToHIR for Option<T>
where
    T: ToHIR,
{
    type HIRType = Option<T::HIRType>;

    type ValidationError = T::ValidationError;

    fn to_higher_ast(&self) -> Self::HIRType {
        self.as_ref().map(|val| val.to_higher_ast())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        self.as_ref().and_then(|val| val.validate())
    }
}

trait HigherASTWithVar {
    fn var_value(var: String) -> Self;
}

fn simple_str(node: SyntaxNode) -> Option<::smol_str::SmolStr> {
    use crate::syntax::StringToken;
    use ::smol_str::SmolStr;
    if node.kind() == Context::String.into() {
        let s = node
            .children_with_tokens()
            .filter_map(|e| match e.kind() {
                SyntaxPart::StringToken(StringToken::Text) => {
                    e.into_token().map(|t| SmolStr::new(t.text()))
                }
                SyntaxPart::StringToken(StringToken::Escape) => e.into_token().map(|t| {
                    SmolStr::new(match t.text().chars().last().unwrap() {
                        '$' => "$",
                        '\\' => "\\",
                        'n' => "\n",
                        'r' => "\r",
                        't' => "\t",
                        c => unreachable!("String escape {} should not be possible", c),
                    })
                }),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        if s.len() > 0 {
            Some(SmolStr::new(s))
        } else {
            None
        }
    } else {
        None
    }
}
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
