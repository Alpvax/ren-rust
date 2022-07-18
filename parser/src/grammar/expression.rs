use ast::expr::Operator;

use crate::{
    syntax::{Context, StringToken, Token, TokenType},
    Parser,
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
                Token::VarName
                | Token::Placeholder
                | Token::Number
                | Token::Bool
                | Token::Undefined => p.bump(),
                Token::OpSub => parse_prefix_op(p, Operator::Sub),
                Token::DoubleQuote => parse_string(p),
                Token::Namespace => parse_scoped(p),
                Token::ParenOpen => parse_parenthesised(p),
                Token::CurlyOpen => parse_record(p),
                Token::SquareOpen => parse_array(p),
                // Token::Hash => {
                //     p.start_node(Context::Constructor);
                //     p.bump();
                //     p.finish_node();
                // },
                // Token::KWLet => {
                //     parse_let_to_semi(p);
                //     parse_single_term(p);
                //     p.finish_node();
                // },
                // Token::KWFun => {
                //     parse_lambda_to_arrow(p);
                //     parse_single_term(p);
                //     p.finish_node();
                // },
                // Token::KWIf => todo!(),
                // Token::KWWhere => todo!(),
                _ => {
                    return false;
                }
            }
            true
        }
        TokenType::None => false,
        TokenType::String(_) => todo!("ERROR"),
    }
}

fn parse_term(p: &mut Parser) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number | Token::VarName | Token::Placeholder => p.bump(),
            Token::DoubleQuote => parse_string(p),
            Token::OpSub => parse_prefix_op(p, Operator::Sub),
            Token::Namespace => parse_scoped(p),
            Token::Hash => {
                let m = p.start("constructor");
                p.bump();
                if p.bump_matching(Token::VarName) {
                    let args = p.start("args");
                    loop {
                        if !parse_single_term(p) {
                            break;
                        }
                    }
                    args.complete(p, Context::Args);
                    m.complete(p, Context::Constructor);
                } else {
                    todo!("ERROR")
                }
            }
            Token::ParenOpen => parse_parenthesised(p),
            Token::SquareOpen => parse_array(p),
            Token::CurlyOpen => parse_record(p),
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
            Token::KWWhere => parse_where(p),
            Token::KWFun => parse_lambda(p),
            _ => {}
        },
        TokenType::String(_) => unreachable!(),
        TokenType::None => todo!("ERROR: EOF"),
    }
}

fn parse_string(p: &mut Parser) {
    assert_eq!(p.peek(), TokenType::Token(Token::DoubleQuote));
    let str_m = p.start("string");
    p.bump();
    loop {
        while let TokenType::String(StringToken::Text | StringToken::Escape) = p.peek() {
            p.bump()
        }
        if p.bump_matching(StringToken::ExprStart) {
            let expr_m = p.start("string_expression");
            expr(p);
            expr_m.complete(p, Context::Expr);
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

fn parse_scoped(p: &mut Parser) {
    let m = p.start("scoped");
    loop {
        if p.bump_matching(Token::Namespace) && p.bump_matching(Token::Period) {
            match p.peek() {
                TokenType::Token(Token::VarName) => {
                    p.bump();
                    m.complete(p, Context::Scoped);
                    break;
                }
                TokenType::Token(Token::Namespace) => continue,
                _ => todo!("ERROR"),
            }
        } else {
            todo!("ERROR");
        }
    }
}

fn parse_parenthesised(p: &mut Parser) {
    let m = p.start("paren");
    p.bump();
    expr(p);
    if p.peek() == TokenType::Token(Token::ParenClose) {
        p.bump();
        m.complete(p, Context::Expr);
    } //else error
}

fn parse_record(p: &mut Parser) {
    assert!(p.peek().is(Token::CurlyOpen));
    let rec_m = p.start("record");
    p.bump();
    loop {
        let field = p.start("field");
        p.bump_matching(Token::VarName);
        if p.bump_matching(Token::Colon) {
            expr(p)
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

fn parse_array(p: &mut Parser) {
    let m = p.start("array");
    p.bump();
    loop {
        expr(p);
        if p.bump_matching(Token::Comma) {
            continue; // No dangling comma
        }
        if p.bump_matching(Token::SquareClose) {
            m.complete(p, Context::Array);
            break;
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
                TokenType::Token(Token::OpJoin) => Operator::Concat,
                TokenType::Token(Token::OpCons) => Operator::Cons,
                TokenType::Token(Token::OpDiv) => Operator::Div,
                TokenType::Token(Token::OpEq) => Operator::Eq,
                TokenType::Token(Token::OpGte) => Operator::Gte,
                TokenType::Token(Token::OpGt) => Operator::Gt,
                TokenType::Token(Token::OpLte) => Operator::Lte,
                TokenType::Token(Token::OpLt) => Operator::Lt,
                TokenType::Token(Token::OpMod) => Operator::Mod,
                TokenType::Token(Token::OpMul) => Operator::Mul,
                TokenType::Token(Token::OpNotEq) => Operator::Neq,
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
                        todo!("ERROR: Application args must be single terms");
                    }
                } else if p.bump_matching(Token::Period) {
                    if p.bump_matching(Token::VarName) {
                        start.commit(p, Context::Access);
                    } else {
                        todo!("ERROR: Access key is not VarName");
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

fn parse_prefix_op(p: &mut Parser, operator: Operator) {
    let right_binding_power = prefix_binding_power(operator).unwrap();
    let m = p.start("prefix");
    p.bump();
    parse_subexpression(p, right_binding_power);
    m.complete(p, Context::PrefixOp);
}

fn parse_let(p: &mut Parser) {
    let declaration = p.start("let_expr");
    if p.bump_matching(Token::KWLet)
        && p.bump_matching(Token::VarName)
        && p.bump_matching(Token::OpAssign)
    {
        let expr_m = p.start("let_body");
        expr(p);
        if p.peek().is(Token::SemiColon) {
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
        if p.bump_matching(Token::OpArrow) {
            params.complete(p, Context::Params);
            expr(p);
            lambda.complete(p, Context::Lambda);
            break;
        }
    }
}

fn parse_where(p: &mut Parser) {
    assert!(p.peek().is(Token::KWWhere));
    let where_m = p.start("where");
    p.bump();
    let mut expr_m = p.start("where_expr");
    expr(p);
    expr_m.complete(p, Context::Expr);
    loop {
        let branch_m = p.start("where_branch");
        if p.bump_matching(Token::KWIs) {
            super::parse_pattern(p);
            if p.peek().is(Token::KWIf) {
                let guard_m = p.start("where_guard");
                p.bump();
                expr(p);
                guard_m.complete(p, Context::Guard);
            }
            if !p.bump_matching(Token::OpArrow) {
                todo!("ERROR");
            }
            expr_m = p.start("branch_expr");
            expr(p);
            expr_m.complete(p, Context::Expr);
            branch_m.complete(p, Context::Branch);
        } else {
            break;
        }
    }
    where_m.complete(p, Context::Where);
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
