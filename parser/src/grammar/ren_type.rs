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
        TokenType::Token(Token::CurlyOpen) => super::parse_record(p, NESTED_TYPE), // Rec
        TokenType::Token(Token::OpMul) => p.bump(),                                // Any
        _ => {
            parse_subtype(p, 0);
        }
    }
}

fn parse_single_term(p: &mut Parser) -> bool {
    match p.peek() {
        TokenType::Token(Token::Hash) => {
            // Variant
            let var_m = p.start("variant");
            p.bump();
            if p.bump_matching(Token::VarName) {
                var_m.complete(p, Context::Variant);
            } else {
                todo!("ERROR! variant name must be varname");
            }
        }
        TokenType::Token(Token::Namespace) => p.bump(), // Con
        TokenType::Token(Token::ParenOpen) => super::parse_parenthesised(p, NESTED_TYPE), // Parenthesised
        TokenType::Token(Token::TypeQuestion) => p.bump(),                                // Hole
        TokenType::Token(Token::VarName) => p.bump(),                                     // Var
        _ => return false,
    }
    true
}

fn parse_subtype(p: &mut Parser, minimum_binding_power: u8) -> bool {
    let mut start = p.start("subtyp");
    if parse_single_term(p) {
        if loop {
            let (left_binding_power, right_binding_power, ctx) = match p.peek() {
                TokenType::Token(Token::TypeBar) => (2, 3, Context::SumType),
                TokenType::Token(Token::TypeArrow) => (2, 1, Context::FunType),
                TokenType::None => break true,
                _ => break true, // we’ll handle errors later.
            };
            if left_binding_power < minimum_binding_power {
                break false;
            }
            // Eat the operator’s token.
            p.bump();
            parse_subtype(p, right_binding_power);
            start.commit(p, ctx);
        } {
            loop {
                if p.bump_whitespace() {
                    if parse_single_term(p) {
                        start.commit(p, Context::Application);
                    } else {
                        // todo!("ERROR: Application args must be single terms. peek = {:?}", p.peek());
                    }
                } else {
                    break;
                }
            }
        }
        start.discard();
        true
    } else {
        start.discard();
        false
    }
}
