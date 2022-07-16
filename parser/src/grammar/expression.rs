use ast::expr::Operator;

use crate::{
    syntax::{Context, StringToken, Token, TokenType},
    Parser,
};

pub(super) fn expr(p: &mut Parser) {
    parse_subexpression(p, 0);
}

fn parse_term(p: &mut Parser, checkpoint: rowan::Checkpoint) {
    match p.peek() {
        TokenType::Token(tok) => match tok {
            Token::Number | Token::VarName => p.bump(),
            Token::OpSub => {
                let right_binding_power = prefix_binding_power(Operator::Sub).unwrap();

                // Eat the operator’s token.
                p.bump();

                p.start_node_at(checkpoint, Context::PrefixOp);
                parse_subexpression(p, right_binding_power);
                p.finish_node();
            }
            Token::ParenOpen => {
                p.bump();
                expr(p);
                if p.peek() == TokenType::Token(Token::ParenClose) {
                    p.bump();
                } //else error
            }
            Token::DoubleQuote => parse_string(p),
            _ => {}
        },
        TokenType::String(_) => todo!(),
        TokenType::None => (),
    }
}

fn parse_string(p: &mut Parser) {
    assert_eq!(p.peek(), TokenType::Token(Token::DoubleQuote));
    p.start_node(Context::String);
    p.bump();
    loop {
        while let TokenType::String(StringToken::Text | StringToken::Escape) = p.peek() {
            p.bump()
        }
        if p.bump_matching(StringToken::ExprStart) {
            p.start_node(Context::Expr);
            expr(p);
            p.finish_node();
            if !p.bump_matching(Token::CurlyClose) {
                todo!("ERROR");
            }
        }
        if p.bump_matching(StringToken::Delimiter) {
            p.finish_node();
            break;
        }
        if let TokenType::Token(_) | TokenType::None = p.peek() {
            todo!("ERROR");
        }
    }
}

fn parse_subexpression(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    parse_term(p, checkpoint);

    loop {
        let op = match p.peek() {
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
            _ => return, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = infix_binding_power(op);

        if left_binding_power < minimum_binding_power {
            return;
        }

        // Eat the operator’s token.
        p.bump();

        p.start_node_at(checkpoint, Context::BinOp);
        parse_subexpression(p, right_binding_power);
        p.finish_node();
    }
}

fn infix_binding_power(operator: Operator) -> (u8, u8) {
    match operator {
        //Left associativity
        Operator::Pipe => (1, 2),
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
