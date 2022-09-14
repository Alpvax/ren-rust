use crate::{
    parser::Parser,
    syntax::{Context, Token, TokenType},
};

const NESTED_TYPE: super::NestedParser = super::NestedParser {
    func: typ,
    record_value_required: true,
    record_allow_empty: true,
};

pub(super) fn parse_type(p: &mut Parser) {
    let m = p.start("type");
    typ(p);
    m.complete(p, Context::Type);
}

fn typ(p: &mut Parser) {
    match p.peek() {
        TokenType::Token(Token::OpMul | Token::TypeQuestion) => p.bump(), // Any | Hole
        TokenType::Token(Token::CurlyOpen) => super::parse_record(p, NESTED_TYPE), // Rec
        TokenType::Token(Token::ParenOpen) => super::parse_parenthesised(p, NESTED_TYPE), // Parenthesised
        _ => todo!("ERROR! Parsing type"),
    }
}
