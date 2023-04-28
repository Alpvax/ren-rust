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

    if p.peek().is(Token::SymDoubleQuote) {
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
            if p.peek().is(Token::IdUpper) {
                let alias = p.start("import_alias");
                loop {
                    if p.bump_matching(Token::IdUpper) {
                        if !p.bump_matching(Token::SymDot) {
                            alias.complete(p, Context::IdUpper);
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

        // if p.bump_matching(Token::KWExposing) && p.bump_matching(Token::SymLBrace) {
        //     let exp_block = p.start("exposing");
        //     loop {
        //         if p.bump_matching(Token::IdLower) {
        //             if p.bump_matching(Token::SymRBrace) {
        //                 exp_block.complete(p, Context::ExposingBlock);
        //                 break;
        //             }
        //             if !p.bump_matching(Token::Comma) {
        //                 todo!("ERROR");
        //             }
        //         } else {
        //             todo!("ERROR");
        //         }
        //     }
        // }
        import.complete(p, Context::Import);
    }
}

pub(super) fn parse_declaration(p: &mut Parser) {
    let dec_m = p.start("declaration");
    p.bump_matching(Token::KWPub);
    if p.bump_matching(Token::KWLet) && p.bump_matching(Token::IdLower) {
        if p.bump_matching(Token::SymColon) {
            super::parse_type(p);
        }
        if p.bump_matching(Token::SymEquals) {
            let expr_m = p.start("declaration_body");
            super::expression::expr(p);
            expr_m.complete(p, Context::Expr);
        } else {
            todo!("ERROR: Missing expression");
        }
    } else if p.bump_matching(Token::KWExt) && p.bump_matching(Token::IdLower) {
        if p.bump_matching(Token::SymColon) {
            super::parse_type(p);
        }
        if p.bump_matching(Token::SymEquals) && p.peek().is(Token::SymDoubleQuote) {
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
            todo!("ERROR: Missing external name");
        }
    } else if p.bump_matching(Token::KWType)
        && p.bump_matching(Token::IdUpper)
        && p.bump_matching(Token::SymEquals)
    {
        super::parse_type(p);
    } else {
        todo!("ERROR: recieved: {:?}", p.peek());
    }
    dec_m.complete(p, Context::Declaration);
}
