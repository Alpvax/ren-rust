use crate::{
    syntax::{Context, StringToken, Token, TokenType},
    Parsed, Parser,
};

mod expression;
mod module;
mod pattern;
use higher_ast::Operator;
use pattern::parse_pattern;

pub fn parse_module(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    let root = p.start("module");
    module::module(&mut p);
    root.complete(&mut p, Context::Module);
    p.parse()
}

pub fn parse_expression(input: &str) -> Parsed {
    let mut p = Parser::new(input);
    let root = p.start("expr_root");
    expression::expr(&mut p);
    root.complete(&mut p, Context::Expr);
    p.parse()
}

pub fn parse_repl_stmt(
    input: &str,
) -> Result<super::REPLStmt<Parsed, Parsed, Parsed>, &'static str> {
    let mut p = Parser::new(input);
    let root = p.start("repl_stmt_root");
    let stmt = match p.peek() {
        TokenType::Token(Token::KWImport) => {
            module::parse_import(&mut p);
            root.discard();
            super::REPLStmt::Import
        }
        TokenType::Token(Token::KWExt | Token::KWLet) => {
            module::parse_declaration(&mut p);
            root.discard();
            super::REPLStmt::Decl
        }
        TokenType::Token(Token::KWPub) => {
            return Err("Cannot use public declarations inside the REPL");
        }
        TokenType::Token(Token::Comment) => {
            return Err("COMMENT");
        }
        TokenType::None => {
            return Err("");
        }
        _ => {
            expression::expr(&mut p);
            root.complete(&mut p, Context::Expr);
            super::REPLStmt::Expr
        }
    };
    Ok(stmt(p.parse()))
}

type ExprOrPatFn = fn(&mut Parser);
pub(crate) fn parse_literal(p: &mut Parser, nested: ExprOrPatFn) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number | /*Token::Bool |*/ Token::Placeholder | Token::VarName => p.bump(),
            Token::OpSub => expression::parse_prefix_op(p, Operator::Sub),
            Token::DoubleQuote => parse_string(p, nested),
            Token::CurlyOpen => parse_record(p, nested),
            Token::SquareOpen => parse_array(p, nested),
            Token::ParenOpen => parse_parenthesised(p, nested),
            _ => {}
        },
        TokenType::None => {}
        TokenType::String(_) => unreachable!("ERROR: recieved string token outside of string."),
    }
}

fn parse_string(p: &mut Parser, nested: ExprOrPatFn) {
    assert_eq!(p.peek(), TokenType::Token(Token::DoubleQuote));
    let str_m = p.start("string");
    p.bump();
    loop {
        while let TokenType::String(StringToken::Text | StringToken::Escape) = p.peek() {
            p.bump()
        }
        if p.bump_matching(StringToken::ExprStart) {
            let nested_m = p.start("string_nested");
            nested(p);
            nested_m.complete(p, Context::Expr);
            if !p.bump_matching(Token::CurlyClose) {
                todo!("ERROR");
            }
        }
        if p.bump_matching(StringToken::Delimiter) {
            str_m.complete(p, Context::String);
            break;
        }
        if let TokenType::Token(_) | TokenType::None = p.peek() {
            todo!("ERROR");
        }
    }
}

fn parse_parenthesised(p: &mut Parser, nested: ExprOrPatFn) {
    let m = p.start("paren");
    p.bump();
    nested(p);
    if p.peek() == TokenType::Token(Token::ParenClose) {
        p.bump();
        m.complete(p, Context::Expr);
    } //else error
}

fn parse_record(p: &mut Parser, nested: ExprOrPatFn) {
    assert!(p.peek().is(Token::CurlyOpen));
    let rec_m = p.start("record");
    p.bump();
    loop {
        let field = p.start("field");
        p.bump_matching(Token::VarName);
        if p.bump_matching(Token::Colon) {
            nested(p)
        }
        if p.bump_matching(Token::Comma) {
            field.complete(p, Context::Field);
            continue; // No dangling comma
        }
        if p.peek().is(Token::CurlyClose) {
            field.complete(p, Context::Field);
            p.bump();
            rec_m.complete(p, Context::Record);
            break;
        } else {
            todo!("ERROR");
        }
    }
}

fn parse_array(p: &mut Parser, nested: ExprOrPatFn) {
    let m = p.start("array");
    p.bump();
    loop {
        if p.bump_matching(Token::SquareClose) {
            m.complete(p, Context::Array);
            break;
        }
        let item_m = p.start("array_item");
        nested(p);
        item_m.complete(p, Context::Item);
        if p.bump_matching(Token::Comma) {
            continue; // No dangling comma
        } else if !p.peek().is(Token::SquareClose) {
            todo!("ERROR: non-comma following item in array");
        }
    }
}

#[cfg(test)]
mod tests;
