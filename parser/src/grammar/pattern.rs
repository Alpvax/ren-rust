use crate::{
    syntax::{Context, Token, TokenType},
    Parser,
};

use super::parse_literal;

const NESTED_PATTERN: super::NestedParser = super::NestedParser {
    func: |p| {
        pattern(p);
    },
    record_value_required: false,
    record_allow_empty: false,
};

pub(super) fn parse_pattern(p: &mut Parser) -> bool {
    let m = p.start("pattern");
    if pattern(p) {
        m.complete(p, Context::Pattern);
        true
    } else {
        m.discard();
        false
    }
}

fn pattern(p: &mut Parser) -> bool {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number
            // | Token::Bool
            | Token::SymUnderscore
            | Token::IdLower
            | Token::OpSub
            | Token::SymDoubleQuote
            | Token::SymLBrace
            | Token::SymLBracket
            | Token::SymLParen => parse_literal(p, NESTED_PATTERN),
            Token::SymAt => {
                let typ_m = p.start("type_match");
                p.bump();
                if p.bump_matching(Token::IdUpper) {
                    if p.bump_whitespace() {
                        let arg_m = p.start("type_match_arg");
                        if pattern(p) {
                            arg_m.complete(p, Context::Pattern);
                        } else {
                            arg_m.discard(); // TODO: Is this correct? or should it error?
                        }
                    }
                    typ_m.complete(p, Context::TypeMatch);
                } else {
                    todo!("ERROR: invalid type name token")
                }
            }
            Token::SymHash => {
                let con_m = p.start("variant_pattern");
                p.bump();
                if p.bump_matching(Token::IdLower) {
                    if p.bump_whitespace() {
                        let args = p.start("args");
                        loop {
                            pattern(p); //TODO: better parsing for constructor
                            if !p.bump_whitespace() || p.peek().is(Token::SymArrow) {
                                break;
                            }
                        }
                        args.complete(p, Context::Args);
                    }
                    con_m.complete(p, Context::Constructor);
                } else {
                    todo!("ERROR")
                }
            }
            _ => todo!("ERROR: invalid Pattern start token: {:?}", tok),
        },
        TokenType::None => {
            return false;
        }
        TokenType::String(_) => unreachable!("ERROR: recieved string token outside of string."),
    };
    true
}
