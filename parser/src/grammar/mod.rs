use crate::{Parsed, Parser};

mod declaration;
mod expression;
mod module;

pub fn parse_module(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    p.start_node(crate::syntax::SyntaxPart::Context(
        crate::syntax::Context::Module,
    ));
    module::module(&mut p);
    p.finish_node();
    p.parse()
}

pub fn parse_expression(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    p.start_node(crate::syntax::SyntaxPart::Context(
        crate::syntax::Context::Expr,
    ));
    expression::expr(&mut p);
    p.finish_node();
    p.parse()
}

#[cfg(test)]
mod tests;
