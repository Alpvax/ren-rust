use crate::{
    parser::Parser,
    syntax::{Context, StringToken, Token, TokenType},
};

pub(super) fn module(p: &mut Parser) {
    if p.peek().is(Token::KWImport) {
        let imports = p.start("imports");
        while let TokenType::Token(Token::KWImport) = p.peek() {
            parse_import(p);
        }
        imports.complete(p, Context::Imports);
    }
    if let TokenType::Token(Token::KWPub | Token::KWLet | Token::KWExt) = p.peek() {
        let declarations = p.start("declarations");
        while let TokenType::Token(Token::KWPub | Token::KWLet | Token::KWExt) = p.peek() {
            parse_declaration(p);
        }
        declarations.complete(p, Context::Declarations);
    }
}

pub(super) fn parse_import(p: &mut Parser) {
    assert_eq!(p.peek(), TokenType::Token(Token::KWImport));

    let import = p.start("import");
    p.bump();

    if !p.bump_matching(Token::KWPkg) {
        p.bump_matching(Token::KWExt);
    }

    if p.peek().is(Token::DoubleQuote) {
        let str_m = p.start("import_path");
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
        str_m.complete(p, Context::String);

        if p.bump_matching(Token::KWAs) {
            if p.peek().is(Token::Namespace) {
                let namespace = p.start("import_ns");
                loop {
                    if p.bump_matching(Token::Namespace) {
                        if !p.bump_matching(Token::Period) {
                            namespace.complete(p, Context::NameSpace);
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
            let exp_block = p.start("exposing");
            loop {
                if p.bump_matching(Token::VarName) {
                    if p.bump_matching(Token::CurlyClose) {
                        exp_block.complete(p, Context::ExposingBlock);
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
        import.complete(p, Context::Import);
    }
}

pub(super) fn parse_declaration(p: &mut Parser) {
    let dec_m = p.start("declaration");
    p.bump_matching(Token::KWPub);
    if p.bump_matching(Token::KWLet) {
        if p.bump_matching(Token::VarName) && p.bump_matching(Token::OpAssign) {
            let expr_m = p.start("declaration_body");
            super::expression::expr(p);
            expr_m.complete(p, Context::Expr);
        } else {
            todo!("ERROR");
        }
    } else if p.bump_matching(Token::KWExt)
        && p.bump_matching(Token::VarName)
        && p.bump_matching(Token::OpAssign)
        && p.peek().is(Token::DoubleQuote)
    {
        let str_m = p.start("ext_name");
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
        str_m.complete(p, Context::String);
    } else {
        todo!("ERROR: recieved: {:?}", p.peek());
    }
    dec_m.complete(p, Context::Declaration);
}
