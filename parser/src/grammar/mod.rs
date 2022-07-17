use crate::{Parsed, Parser};

mod expression;
mod module;
mod pattern;
use pattern::parse_pattern;

pub fn parse_module(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    let root = p.start();
    module::module(&mut p);
    root.complete(&mut p, crate::syntax::Context::Module);
    p.parse()
}

pub fn parse_expression(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    let root = p.start();
    expression::expr(&mut p);
    root.complete(&mut p, crate::syntax::Context::Expr);
    p.parse()
}

#[cfg(test)]
mod tests;
