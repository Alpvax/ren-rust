use crate::ast::expression::Expression;

use super::{consume_whitespace, parse_expression, parse_fun_args, Lexer, Token};

pub fn parse_lambda(lexer: &mut Lexer) -> Result<Expression, super::Error> {
    if let Some(Token::KWFun) = lexer.peek_token() {
        lexer.next(); //Consume "fun"
        consume_whitespace(lexer);
        parse_fun_args(lexer).map_or_else(
            |e| todo!("Convert params error to expression error"),
            |args| Ok(Expression::Lambda(args, Box::new(parse_expression(lexer)?))),
        )
    } else {
        todo!("Invalid start token")
    }
}

pub fn parse_conditional(lexer: &mut Lexer) -> Result<Expression, super::Error> {
    if let Some(Token::KWIf) = lexer.peek_token() {
        lexer.next(); //Consume "if"
        consume_whitespace(lexer);
        todo!("Parse <expr> \"then\" <expr> \"else\" <expr>")
    } else {
        todo!("Invalid start token")
    }
}
