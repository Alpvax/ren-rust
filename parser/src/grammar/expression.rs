use higher_ast::Operator;

use crate::{
    syntax::{Context, Token, TokenType},
    Parser,
};

use super::{parse_literal, pattern::parse_pattern};

const NESTED_EXPR: super::NestedParser = super::NestedParser {
    func: expr,
    record_value_required: false,
    record_allow_empty: true,
};

pub(super) fn expr(p: &mut Parser) {
    if !parse_subexpression(p, 0) {
        parse_term(p);
    }
}

fn parse_single_term(p: &mut Parser) -> bool {
    match p.peek() {
        TokenType::Token(tok) => {
            match tok {
                // Token::Undefined => p.bump(),
                Token::Number
                // | Token::Bool
                | Token::SymUnderscore
                | Token::IdLower
                | Token::OpSub
                | Token::SymDoubleQuote
                | Token::SymLBrace
                | Token::SymLBracket
                | Token::SymLParen => parse_literal(p, NESTED_EXPR),
                Token::IdUpper => parse_scoped(p),
                //TODO: single-term constructors (uncommenting conflicts with Application)
                // Token::Hash => {
                //     let mark = p.start("single_constructor");
                //     p.bump();
                //     if p.bump_matching(Token::IdLower) {
                //         mark.complete(p, Context::Constructor);
                //     } else {
                //         todo!("ERROR: ")
                //     }
                // },
                _ => {
                    return false;
                }
            }
            true
        }
        TokenType::None => false,
        TokenType::String(_) => unreachable!("ERROR: recieved string token outside of string."),
    }
}

fn parse_term(p: &mut Parser) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            // Token::Undefined => p.bump(),
            Token::Number
            // | Token::Bool
            | Token::SymUnderscore
            | Token::IdLower
            | Token::OpSub
            | Token::SymDoubleQuote
            | Token::SymLBrace
            | Token::SymLBracket
            | Token::SymLParen => parse_literal(p, NESTED_EXPR),
            Token::IdUpper => parse_scoped(p),
            Token::SymHash => {
                let m = p.start("constructor");
                p.bump();
                if p.bump_matching(Token::IdLower) {
                    if p.bump_whitespace() {
                        let args = p.start("args");
                        loop {
                            if !parse_single_term(p) || !p.bump_whitespace() {
                                break;
                            }
                        }
                        args.complete(p, Context::Args);
                    }
                    m.complete(p, Context::Constructor);
                } else {
                    todo!("ERROR")
                }
            }
            Token::KWIf => {
                let conditional_m = p.start("conditional");
                p.bump();
                let condition_m = p.start("condition");
                expr(p);
                condition_m.complete(p, Context::Condition);
                assert!(p.peek().is(Token::KWThen));
                p.bump();
                let then_m = p.start("then");
                expr(p);
                then_m.complete(p, Context::Then);
                assert!(p.peek().is(Token::KWElse));
                p.bump();
                let else_m = p.start("else");
                expr(p);
                else_m.complete(p, Context::Else);
                conditional_m.complete(p, Context::Conditional);
            }
            Token::KWLet => parse_let(p),
            Token::KWSwitch => parse_switch(p),
            Token::KWFun => parse_lambda(p),
            _ => {}
        },
        TokenType::String(_) => unreachable!(),
        TokenType::None => todo!("ERROR: EOF"),
    }
}

fn parse_scoped(p: &mut Parser) {
    let m = p.start("scoped");
    loop {
        if p.bump_matching(Token::IdUpper) && p.bump_matching(Token::SymDot) {
            match p.peek() {
                TokenType::Token(Token::IdLower) => {
                    p.bump();
                    m.complete(p, Context::Scoped);
                    break;
                }
                TokenType::Token(Token::IdUpper) => continue,
                _ => todo!("ERROR"),
            }
        } else {
            todo!("ERROR");
        }
    }
}

fn parse_subexpression(p: &mut Parser, minimum_binding_power: u8) -> bool {
    let mut start = p.start("subexpr");
    if parse_single_term(p) {
        if loop {
            let op = match p.peek() {
                TokenType::None => break true,
                TokenType::Token(Token::OpAdd) => Operator::Add,
                TokenType::Token(Token::OpAnd) => Operator::And,
                TokenType::Token(Token::OpConcat) => Operator::Concat,
                // TokenType::Token(Token::OpCons) => Operator::Cons,
                TokenType::Token(Token::OpDiv) => Operator::Div,
                TokenType::Token(Token::OpEq) => Operator::Eq,
                TokenType::Token(Token::OpGte) => Operator::Gte,
                TokenType::Token(Token::OpGt) => Operator::Gt,
                TokenType::Token(Token::OpLte) => Operator::Lte,
                TokenType::Token(Token::OpLt) => Operator::Lt,
                TokenType::Token(Token::OpMod) => Operator::Mod,
                TokenType::Token(Token::OpMul) => Operator::Mul,
                TokenType::Token(Token::OpNeq) => Operator::Neq,
                TokenType::Token(Token::OpOr) => Operator::Or,
                TokenType::Token(Token::OpPipe) => Operator::Pipe,
                TokenType::Token(Token::OpSub) => Operator::Sub,
                _ => break true, // we’ll handle errors later.
            };
            let (left_binding_power, right_binding_power) = infix_binding_power(op);
            if left_binding_power < minimum_binding_power {
                break false;
            }
            // Eat the operator’s token.
            p.bump();
            parse_subexpression(p, right_binding_power);
            start.commit(p, Context::BinOp);
        } {
            loop {
                if p.bump_whitespace() {
                    if parse_single_term(p) {
                        start.commit(p, Context::Application);
                    } else {
                        // todo!("ERROR: Application args must be single terms. peek = {:?}", p.peek());
                    }
                } else if p.bump_matching(Token::SymDot) {
                    if p.bump_matching(Token::IdLower) {
                        start.commit(p, Context::Access);
                    } else {
                        todo!("ERROR: Access key is not IdLower");
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

pub(crate) fn parse_prefix_op(p: &mut Parser, operator: Operator) {
    let right_binding_power = prefix_binding_power(operator).unwrap();
    let m = p.start("prefix");
    p.bump();
    parse_subexpression(p, right_binding_power);
    m.complete(p, Context::PrefixOp);
}

fn parse_let(p: &mut Parser) {
    let declaration = p.start("let_expr");
    if p.bump_matching(Token::KWLet) && parse_pattern(p) && p.bump_matching(Token::SymEquals) {
        let expr_m = p.start("let_body");
        expr(p);
        if p.peek().is(Token::OpSeq) {
            expr_m.complete(p, Context::Expr);
            p.bump();
            expr(p);
            declaration.complete(p, Context::Declaration);
        } else {
            todo!("ERROR: Missing semi")
        }
    } else {
        todo!("ERROR");
    }
}

fn parse_lambda(p: &mut Parser) {
    assert!(p.peek().is(Token::KWFun));
    let lambda = p.start("lambda");
    p.bump();
    let params = p.start("lambda_params");
    loop {
        super::parse_pattern(p);
        if p.bump_matching(Token::SymArrow) {
            params.complete(p, Context::Params);
            expr(p);
            lambda.complete(p, Context::Lambda);
            break;
        }
    }
}

fn parse_switch(p: &mut Parser) {
    assert!(p.peek().is(Token::KWSwitch));
    let where_m = p.start("switch");
    p.bump();
    let mut expr_m = p.start("switch_expr");
    expr(p);
    expr_m.complete(p, Context::Expr);
    if !p.bump_matching(Token::KWOn) {
        todo!("ERROR expected KWOn");
    }
    loop {
        let branch_m = p.start("case");
        if p.bump_matching(Token::KWCase) {
            super::parse_pattern(p);
            if p.peek().is(Token::KWIf) {
                let guard_m = p.start("case_guard");
                p.bump();
                expr(p);
                guard_m.complete(p, Context::Guard);
            }
            if !p.bump_matching(Token::SymArrow) {
                todo!("ERROR");
            }
            expr_m = p.start("branch_expr");
            expr(p);
            expr_m.complete(p, Context::Expr);
            branch_m.complete(p, Context::Branch);
        } else {
            branch_m.discard();
            break;
        }
    }
    where_m.complete(p, Context::Switch);
}

fn infix_binding_power(operator: Operator) -> (u8, u8) {
    match operator {
        //Left associativity
        Operator::Pipe => (2, 3),
        Operator::Eq => (4, 5),
        Operator::Gt => (4, 5),
        Operator::Gte => (4, 5),
        Operator::Lt => (4, 5),
        Operator::Lte => (4, 5),
        Operator::Neq => (4, 5),
        Operator::Add => (6, 7),
        Operator::Sub => (6, 7),
        Operator::Div => (8, 9),
        Operator::Mul => (8, 9),

        //Right associativity
        Operator::Or => (2, 1),
        Operator::And => (3, 2),
        Operator::Concat => (5, 4),
        Operator::Cons => (5, 4),

        //Unknown
        Operator::Mod => (8, 9),
    }
}

fn prefix_binding_power(operator: Operator) -> Option<u8> {
    if let Operator::Sub = operator {
        Some(10)
    } else {
        None
    }
}
