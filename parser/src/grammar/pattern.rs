use crate::{
    syntax::{Context, Token, TokenType},
    Parser,
};

use super::parse_literal;

pub(super) fn parse_pattern(p: &mut Parser) {
    let m = p.start("pattern");
    pattern(p);
    m.complete(p, Context::Pattern);
}

fn pattern(p: &mut Parser) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number
            | Token::Bool
            | Token::Placeholder
            | Token::VarName
            | Token::OpSub
            | Token::DoubleQuote
            | Token::CurlyOpen
            | Token::SquareOpen
            | Token::ParenOpen => parse_literal(p, pattern),
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
            _ => todo!("ERROR: invalid Pattern start token"),
        },
        TokenType::None => {}
        TokenType::String(_) => unreachable!("ERROR: recieved string token outside of string."),
    }
}
