use crate::{
    syntax::{Context, StringToken, Token, TokenType},
    Parsed, Parser,
};

mod expression;
mod module;
mod pattern;
mod ren_type;
use higher_ast::Operator;
use pattern::parse_pattern;
use ren_type::parse_type;

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
    let stmt = match p.peek() {
        TokenType::Token(Token::KWImport) => {
            module::parse_import(&mut p);
            super::REPLStmt::Import
        }
        TokenType::Token(Token::KWExt | Token::KWLet | Token::KWType) => {
            module::parse_declaration(&mut p);
            super::REPLStmt::Decl
        }
        TokenType::Token(Token::KWPub) => {
            return Err("Cannot use public declarations inside the REPL");
        }
        TokenType::Token(Token::Comment) => {
            return Ok(super::REPLStmt::Comment(input.to_string()));
        }
        TokenType::None => {
            return Ok(super::REPLStmt::Empty);
        }
        _ => {
            let root = p.start("repl_stmt_root");
            expression::expr(&mut p);
            root.complete(&mut p, Context::Expr);
            super::REPLStmt::Expr
        }
    };
    Ok(stmt(p.parse()))
}

pub(crate) struct NestedParser {
    func: fn(&mut Parser),
    /// Whether the value part of a record literal is required
    record_value_required: bool,
    /// Whether {} is a valid record
    record_allow_empty: bool,
}
impl NestedParser {
    pub fn call(&self, p: &mut Parser) {
        (self.func)(p)
    }
}

pub(crate) fn parse_literal(p: &mut Parser, nested: NestedParser) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number | /*Token::Bool |*/ Token::SymUnderscore | Token::IdLower => p.bump(),
            Token::OpSub => expression::parse_prefix_op(p, Operator::Sub),
            Token::SymDoubleQuote => parse_string(p, nested),
            Token::SymLBrace => parse_record(p, nested),
            Token::SymLBracket => parse_array(p, nested),
            Token::SymLParen => parse_parenthesised(p, nested),
            _ => {}
        },
        TokenType::None => {}
        TokenType::String(_) => unreachable!("ERROR: recieved string token outside of string."),
    }
}

fn parse_string(p: &mut Parser, nested: NestedParser) {
    assert_eq!(p.peek(), TokenType::Token(Token::SymDoubleQuote));
    let str_m = p.start("string");
    p.bump();
    loop {
        while let TokenType::String(StringToken::Text | StringToken::Escape) = p.peek() {
            p.bump()
        }
        if p.bump_matching(StringToken::ExprStart) {
            let nested_m = p.start("string_nested");
            nested.call(p);
            nested_m.complete(p, Context::Expr);
            if !p.bump_matching(Token::SymRBrace) {
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

fn parse_parenthesised(p: &mut Parser, nested: NestedParser) {
    let m = p.start("paren");
    p.bump();
    nested.call(p);
    if p.peek() == TokenType::Token(Token::SymRParen) {
        p.bump();
        m.complete(p, Context::Parenthesised);
    } //else error
}

fn parse_record(p: &mut Parser, nested: NestedParser) {
    assert!(p.peek().is(Token::SymLBrace));
    let rec_m = p.start("record");
    p.bump();
    if nested.record_allow_empty && p.peek().is(Token::SymRBrace) {
        p.bump();
        rec_m.complete(p, Context::Record);
    } else {
        loop {
            let field = p.start("field");
            p.bump_matching(Token::IdLower);
            if p.bump_matching(Token::SymColon) {
                nested.call(p)
            } else if nested.record_value_required {
                todo!("Error! required `: value` part of record")
            }
            if p.peek().is(Token::SymComma) {
                field.complete(p, Context::Field);
                p.bump();
                continue; // No dangling comma
            }
            if p.peek().is(Token::SymRBrace) {
                field.complete(p, Context::Field);
                p.bump();
                rec_m.complete(p, Context::Record);
                break;
            } else {
                todo!("ERROR: {:?}", p.peek());
            }
        }
    }
}

fn parse_array(p: &mut Parser, nested: NestedParser) {
    let m = p.start("array");
    p.bump();
    loop {
        if p.bump_matching(Token::SymRBracket) {
            m.complete(p, Context::Array);
            break;
        }
        let item_m = p.start("array_item");
        nested.call(p);
        item_m.complete(p, Context::Item);
        if p.bump_matching(Token::SymComma) {
            continue; // No dangling comma
        } else if !p.peek().is(Token::SymRBracket) {
            todo!("ERROR: non-comma following item in array");
        }
    }
}

#[cfg(test)]
mod tests;
