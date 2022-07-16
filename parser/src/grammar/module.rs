use crate::{
    parser::Parser,
    syntax::{Context, StringToken, Token, TokenType},
};

pub(super) fn module(p: &mut Parser) {
    if p.peek().is(Token::KWImport) {
        p.start_node(Context::Imports);
        while let TokenType::Token(Token::KWImport) = p.peek() {
            parse_import(p);
        }
        p.finish_node();
    }
    if let TokenType::Token(Token::KWPub | Token::KWLet) = p.peek() {
        p.start_node(Context::Declarations);
        while let TokenType::Token(Token::KWPub | Token::KWLet) = p.peek() {
            parse_declaration(p);
        }
        p.finish_node();
    }
}

fn parse_import(p: &mut Parser) {
    assert_eq!(p.peek(), TokenType::Token(Token::KWImport));

    p.start_node(Context::Import);
    p.bump();

    if p.peek().is(Token::DoubleQuote) {
        p.start_node(Context::String);
        p.bump();
        loop {
            match p.peek() {
                TokenType::String(StringToken::Text | StringToken::Escape) => p.bump(),
                TokenType::String(StringToken::Delimiter) => {
                    p.bump();
                    break;
                }
                _ => todo!("ERROR"),
            }
        }
        p.finish_node();

        if p.bump_matching(Token::KWAs) {
            if p.peek().is(Token::Namespace) {
                p.start_node(Context::NameSpace);
                loop {
                    if p.bump_matching(Token::Namespace) {
                        if !p.bump_matching(Token::Period) {
                            p.finish_node();
                            break;
                        }
                    } else {
                        todo!("ERROR");
                    }
                }
            } else {
                todo!("ERROR");
            }
        }

        if p.bump_matching(Token::KWExposing) && p.bump_matching(Token::CurlyOpen) {
            p.start_node(Context::ExposingBlock);
            loop {
                if p.bump_matching(Token::VarName) {
                    if p.bump_matching(Token::CurlyClose) {
                        p.finish_node();
                        break;
                    }
                    if !p.bump_matching(Token::Comma) {
                        todo!("ERROR");
                    }
                } else {
                    todo!("ERROR");
                }
            }
        }
        p.finish_node();
    }
}

fn parse_declaration(p: &mut Parser) {
    p.bump_matching(Token::KWPub);
    super::declaration::declaration(p);
}
