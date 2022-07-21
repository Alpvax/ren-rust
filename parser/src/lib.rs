mod grammar;
mod parser;
mod syntax;

pub use self::parser::Parsed;
pub(crate) use self::parser::Parser;
pub use grammar::{parse_expression, parse_module};
mod ast;

pub fn parse_expr_ast(input: &str) -> Result<elm_ast::expr::Expr, ()> {
    let parsed = parse_expression(input);
    ast::to_ast_expr(parsed.syntax().into())
}

//TODO: pub fn parse_module_ast(input: &str) -> Result<ast::expr::Expr, ()> {
//     let parsed = parse_expression(input);
//     to_ast::to_ast_expr(parsed.syntax())
// }
