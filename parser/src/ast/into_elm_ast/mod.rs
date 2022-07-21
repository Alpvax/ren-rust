use elm_ast::{
    core::{Literal, Pattern},
    expr::{Expr, Operator},
    Module,
};

use crate::syntax::{Context, RenLang, SyntaxNode, SyntaxPart, Token};

type SyntaxElement = rowan::SyntaxElement<RenLang>;

#[cfg(test)]
mod tests;

fn is_not_trivia(node: &SyntaxElement) -> bool {
    if let SyntaxPart::Token(Token::Whitespace | Token::Comment) = node.kind() {
        false
    } else {
        true
    }
}

trait RenNodeIterator: Iterator<Item = SyntaxElement> {
    fn strip_trivia(self) -> std::iter::Filter<Self, fn(&SyntaxElement) -> bool>
    where
        Self: Sized,
    {
        self.filter(is_not_trivia)
    }
    fn find_kind<T: Into<SyntaxPart> + Copy>(&mut self, kind: T) -> Option<SyntaxElement>
    where
        Self: Sized,
    {
        self.find(|node| node.kind() == kind.into())
    }
    fn find_subexpr(&mut self, kind: Context) -> Option<SyntaxElement>
    where
        Self: Sized,
    {
        self.find_kind(kind).and_then(|node| {
            node.as_node()
                .and_then(|n| n.children_with_tokens().strip_trivia().next())
        })
    }
    fn subexpr_ast(&mut self, kind: Context) -> Result<Expr, <SyntaxNode as ToAstExpr>::Error>
    where
        Self: Sized,
    {
        ToAstExpr::to_ast(self.find_subexpr(kind))
    }
}
impl<T> RenNodeIterator for T where T: Iterator<Item = SyntaxElement> {}

trait ToAstExpr {
    type Error;
    fn to_ast(node: Self) -> Result<Expr, Self::Error>;
}
impl ToAstExpr for SyntaxNode {
    type Error = ();

    fn to_ast(node: SyntaxNode) -> Result<Expr, Self::Error> {
        to_ast_expr(SyntaxElement::Node(node))
    }
}
impl ToAstExpr for Option<SyntaxNode> {
    type Error = <SyntaxNode as ToAstExpr>::Error;

    fn to_ast(opt: Option<SyntaxNode>) -> Result<Expr, Self::Error> {
        if let Some(node) = opt {
            ToAstExpr::to_ast(node)
        } else {
            Err(())
        }
    }
}
impl ToAstExpr for SyntaxElement {
    type Error = ();

    fn to_ast(element: SyntaxElement) -> Result<Expr, Self::Error> {
        to_ast_expr(element)
    }
}
impl ToAstExpr for Option<SyntaxElement> {
    type Error = <SyntaxElement as ToAstExpr>::Error;

    fn to_ast(opt: Option<SyntaxElement>) -> Result<Expr, Self::Error> {
        if let Some(element) = opt {
            ToAstExpr::to_ast(element)
        } else {
            Err(())
        }
    }
}

pub(crate) fn to_ast_expr(element: SyntaxElement) -> Result<Expr, ()> {
    match element {
        rowan::NodeOrToken::Token(tok) => {
            if let SyntaxPart::Token(tok_type) = tok.kind() {
                match tok_type {
                    Token::KWType => todo!("ren types"),
                    Token::VarName => Ok(Expr::Var(tok.text().to_string())),
                    Token::Placeholder => Ok(Expr::Placeholder),
                    Token::Number => Ok(Expr::Literal(Literal::LNum(
                        tok.text().to_string().parse().unwrap(),
                    ))),
                    Token::Bool => Ok(Expr::Literal(Literal::LBool(
                        tok.text().to_string().parse().unwrap(),
                    ))),
                    Token::Undefined => Ok(Expr::Literal(Literal::LUnit)),
                    _ => Err(()),
                }
            } else {
                unreachable!("Token element, but non-token kind!")
            }
        }
        rowan::NodeOrToken::Node(node) => {
            if let SyntaxPart::Context(ctx) = node.kind() {
                match ctx {
                    Context::Declaration => {
                        let mut iter = node.children_with_tokens().strip_trivia();
                        let pattern_node = iter.find_kind(Context::Pattern).ok_or(())?;
                        iter.next(); // Skip '='
                        let binding_value = ToAstExpr::to_ast(iter.next())?;
                        iter.next(); // Skip ';'
                        let body = ToAstExpr::to_ast(iter.next())?;
                        Ok(Expr::binding(
                            to_ast_pattern(pattern_node)?,
                            binding_value,
                            body,
                        ))
                    }
                    Context::Expr => ToAstExpr::to_ast(
                        node.children_with_tokens()
                            .find_map(|n| match n.kind() {
                                SyntaxPart::Token(Token::Whitespace)
                                | SyntaxPart::Token(Token::Comment)
                                | SyntaxPart::StringToken(_)
                                | SyntaxPart::EOF
                                | SyntaxPart::Error => None,
                                // Fix parentheses (first child is Token::ParenOpen)
                                SyntaxPart::Token(Token::ParenOpen) => None,
                                _ => Some(n),
                            })
                            .ok_or(())?,
                    ),
                    Context::String => todo!("Ren string"),
                    Context::Scoped => {
                        let mut ns = node
                            .children_with_tokens()
                            .strip_trivia()
                            .filter_map(|n| match n.kind() {
                                SyntaxPart::Token(Token::Namespace | Token::VarName) => {
                                    Some(n.as_token().unwrap().text().to_string())
                                }
                                _ => None,
                            })
                            .collect::<Vec<_>>();
                        let name = ns.pop().unwrap();
                        Ok(Expr::Scoped(ns, name))
                    }
                    Context::Constructor => todo!(),
                    Context::Array => Ok(Expr::Literal(Literal::LArr(
                        node.children_with_tokens()
                            .strip_trivia()
                            .filter_map(|n| ToAstExpr::to_ast(n).ok())
                            .collect(),
                    ))),
                    Context::Record => node
                        .children_with_tokens()
                        .filter(|n| n.kind() == SyntaxPart::Context(Context::Field))
                        .map(|f| {
                            let mut iter =
                                f.as_node().unwrap().children_with_tokens().strip_trivia();
                            let name = iter
                                .find_kind(Token::VarName)
                                .unwrap()
                                .as_token()
                                .unwrap()
                                .text()
                                .to_string();
                            ToAstExpr::to_ast(iter.last())
                                .or_else(|_| Ok(Expr::Var(name.clone())))
                                .map(|expr| (name, expr))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(|fields| Expr::Literal(Literal::LRec(fields))),
                    Context::Conditional => {
                        let mut iter = node.children_with_tokens();
                        let condition = iter.subexpr_ast(Context::Condition)?;
                        let then_ = iter.subexpr_ast(Context::Then)?;
                        let else_ = iter.subexpr_ast(Context::Else)?;
                        Ok(Expr::conditional(condition, then_, else_))
                    }
                    Context::Where => todo!(),
                    Context::Lambda => todo!(),
                    Context::Application => {
                        let mut iter = node.children_with_tokens().strip_trivia();
                        let func = ToAstExpr::to_ast(iter.next().unwrap())?;
                        let arg = ToAstExpr::to_ast(iter.last().unwrap())?;
                        Ok(Expr::apply(func, arg))
                    }
                    Context::Access => {
                        let mut iter = node.children_with_tokens().strip_trivia();
                        let obj = iter.next().ok_or(()).and_then(ToAstExpr::to_ast)?;
                        let key = iter
                            .last()
                            .ok_or(())?
                            .as_token()
                            .unwrap()
                            .text()
                            .to_string();
                        Ok(Expr::access(obj, key))
                    }
                    Context::PrefixOp => {
                        let mut iter = node.children_with_tokens().strip_trivia();
                        let op = Operator::from_symbol(
                            iter.next()
                                .unwrap()
                                .as_token()
                                .unwrap()
                                .text()
                                .to_string()
                                .as_str(),
                        )
                        .ok_or(())?;
                        let expr = ToAstExpr::to_ast(iter.next().unwrap())?;
                        Ok(Expr::binop(Expr::literal(0.0), op, expr))
                    }
                    Context::BinOp => {
                        let mut iter = node.children_with_tokens().strip_trivia();
                        let lhs = ToAstExpr::to_ast(iter.next().unwrap())?;
                        let op = Operator::from_symbol(
                            iter.next()
                                .unwrap()
                                .as_token()
                                .unwrap()
                                .text()
                                .to_string()
                                .as_str(),
                        )
                        .ok_or(())?;
                        let rhs = ToAstExpr::to_ast(iter.next().unwrap())?;
                        Ok(Expr::binop(lhs, op, rhs))
                    }
                    _ => Err(()),
                }
            } else {
                unreachable!("Node element, but non-Context kind!")
            }
        }
    }
}

fn to_ast_pattern(_node: SyntaxElement) -> Result<Pattern, ()> {
    todo!("Pattern AST") //TODO
}
