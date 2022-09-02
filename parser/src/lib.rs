mod grammar;
mod parser;
mod syntax;

pub use self::parser::Parsed;
pub(crate) use self::parser::Parser;
pub use grammar::{parse_expression, parse_module, parse_repl_stmt};
pub mod lower_ast;

pub fn parse_expr_ast(input: &str) -> Result<lower_ast::Expr, ()> {
    let parsed = parse_expression(input);
    lower_ast::expr_ast(parsed.syntax()).ok_or(())
}

pub fn parse_module_ast(input: &str) -> Result<lower_ast::Module, ()> {
    let parsed = parse_module(input);
    lower_ast::module_ast(parsed.syntax()).ok_or(())
}

#[derive(Debug)]
pub enum REPLStmt<D, E, I> {
    Decl(D),
    Expr(E),
    Import(I),
    Comment(String),
    Empty,
}

/// Parse a single REPL statement (import, declaration or expression)
pub fn parse_stmt_ast(
    input: &str,
) -> Result<REPLStmt<lower_ast::Decl, lower_ast::Expr, lower_ast::Import>, &'static str> {
    match parse_repl_stmt(input)? {
        REPLStmt::Decl(parsed) => lower_ast::decl_ast(parsed.syntax())
            .map(REPLStmt::Decl)
            .ok_or("error converting parsed to Decl"),
        REPLStmt::Expr(parsed) => lower_ast::expr_ast(parsed.syntax())
            .map(REPLStmt::Expr)
            .ok_or("error converting parsed to Expr"),
        REPLStmt::Import(parsed) => lower_ast::import_ast(parsed.syntax())
            .map(REPLStmt::Import)
            .ok_or("error converting parsed to Import"),
        REPLStmt::Comment(text) => Ok(REPLStmt::Comment(text)),
        REPLStmt::Empty => Ok(REPLStmt::Empty),
    }
}
