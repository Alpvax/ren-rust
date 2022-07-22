use crate::{
    syntax::{Context, Token, TokenType},
    Parser,
};

use super::parse_literal;

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
            | Token::Placeholder
            | Token::VarName
            | Token::OpSub
            | Token::DoubleQuote
            | Token::CurlyOpen
            | Token::SquareOpen
            | Token::ParenOpen => parse_literal(p, |p| {
                pattern(p);
            }),
            Token::At => {
                let typ_m = p.start("type_match");
                p.bump();
                if p.bump_matching(Token::Namespace) {
                    if p.bump_whitespace() {
                        todo!("Type pattern arguments");
                    }
                    typ_m.complete(p, Context::TypeMatch);
                } else {
                    todo!("ERROR: invalid type name token")
                }
            }
            Token::Hash => {
                let con_m = p.start("variant_pattern");
                p.bump();
                if p.bump_matching(Token::VarName) {
                    if p.bump_whitespace() {
                        let args = p.start("args");
                        loop {
                            pattern(p); //TODO: better parsing for constructor
                            if !p.bump_whitespace() || p.peek().is(Token::OpArrow) {
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
