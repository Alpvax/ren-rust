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
impl<T> REPLStmt<T, T, T> {
    fn map_all<F, U>(self, f: F) -> REPLStmt<U, U, U>
    where
        F: Fn(T) -> U,
    {
        match self {
            REPLStmt::Decl(d) => REPLStmt::Decl(f(d)),
            REPLStmt::Expr(e) => REPLStmt::Expr(f(e)),
            REPLStmt::Import(i) => REPLStmt::Import(f(i)),
            REPLStmt::Comment(c) => REPLStmt::Comment(c),
            REPLStmt::Empty => REPLStmt::Empty,
        }
    }
}
impl<D, E, I> REPLStmt<D, E, I> {
    pub fn map_each<DF, DR, EF, ER, IF, IR>(self, d_f: DF, e_f: EF, i_f: IF) -> REPLStmt<DR, ER, IR>
    where
        DF: Fn(D) -> DR,
        EF: Fn(E) -> ER,
        IF: Fn(I) -> IR,
    {
        match self {
            REPLStmt::Decl(d) => REPLStmt::Decl(d_f(d)),
            REPLStmt::Expr(e) => REPLStmt::Expr(e_f(e)),
            REPLStmt::Import(i) => REPLStmt::Import(i_f(i)),
            REPLStmt::Comment(c) => REPLStmt::Comment(c),
            REPLStmt::Empty => REPLStmt::Empty,
        }
    }
    pub fn map_ok<DF, DR, EF, ER, IF, IR, Err>(
        self,
        d_f: DF,
        e_f: EF,
        i_f: IF,
    ) -> Result<REPLStmt<DR, ER, IR>, Err>
    where
        DF: Fn(D) -> Result<DR, Err>,
        EF: Fn(E) -> Result<ER, Err>,
        IF: Fn(I) -> Result<IR, Err>,
    {
        Ok(match self {
            REPLStmt::Decl(d) => REPLStmt::Decl(d_f(d)?),
            REPLStmt::Expr(e) => REPLStmt::Expr(e_f(e)?),
            REPLStmt::Import(i) => REPLStmt::Import(i_f(i)?),
            REPLStmt::Comment(c) => REPLStmt::Comment(c),
            REPLStmt::Empty => REPLStmt::Empty,
        })
    }
}

/// Parse a single REPL statement (import, declaration or expression)
pub fn parse_stmt_ast<'source>(
    input: &'source str,
) -> Result<
    (
        REPLStmt<lower_ast::Decl, lower_ast::Expr, lower_ast::Import>,
        ::line_col::LineColLookup<'source>,
    ),
    &'static str,
> {
    let line_lookup = ::line_col::LineColLookup::new(input);
    parse_repl_stmt(input)?
        .map_all(|parsed| parsed.syntax())
        .map_ok(
            |syntax| lower_ast::decl_ast(syntax).ok_or("error convertirng parsed to Decl"),
            |syntax| lower_ast::expr_ast(syntax).ok_or("error convertirng parsed to Expr"),
            |syntax| lower_ast::import_ast(syntax).ok_or("error convertirng parsed to Import"),
        )
        .map(|stmt| (stmt, line_lookup))
}
