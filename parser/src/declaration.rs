use std::convert::{TryFrom, TryInto};

use ast::{
    declaration::{BlockDeclaration, Declaration, ModuleDeclaration},
    Ident, Visibility,
};

use crate::Token;

pub fn parse_module_declaration<'source>(
    lexer: &'source mut lexer::Lexer<'source, Token>,
) -> Result<ModuleDeclaration, DeclarationError> {
    parse_declaration_common(lexer, PartialDeclaration::ModuleStart)
}
pub fn parse_block_declaration<'source>(
    lexer: &'source mut lexer::Lexer<'source, Token>,
) -> Result<BlockDeclaration, DeclarationError> {
    parse_declaration_common(lexer, PartialDeclaration::ExpectingLet { module: None })
}

fn parse_declaration_common<'source, T>(
    lexer: &'source mut lexer::Lexer<'source, Token>,
    mut partial: PartialDeclaration,
) -> Result<T, DeclarationError>
where
    T: Declaration + TryFrom<PartialDeclaration, Error = DeclarationError>,
{
    use lexer::Token::*;
    use PartialDeclaration::*;
    lexer.skip_whitespace(true);
    while let Some(tok) = lexer.read() {
        partial = match (partial, tok.clone_token()) {
            (ModuleStart, KWRun) => tok.consume_and_return(RunExpr),
            (ModuleStart, KWPub) => tok.consume_and_return(ExpectingExt {
                module: Some(ModuleInfo::public()),
            }),
            (ModuleStart, _) => ExpectingExt {
                module: Some(ModuleInfo::private()),
            },
            (ExpectingExt { module }, KWExt) => tok.consume_and_return(ExpectingLet {
                module: module.map(ModuleInfo::external),
            }),
            (ExpectingExt { module }, _) => ExpectingLet {
                module,
            },

            // In module context
            (ExpectingLet { module: Some(m) }, KWLet) => todo!(),
            (ExpectingLet { module: Some(m) }, KWType) => tok.consume_and_return(ExpectingExpr { module: Some(m), ident: None }),
            // In block context
            (ExpectingLet { module: None, .. }, KWLet) => todo!(),
            (ExpectingLet { module: None, .. }, KWRun) => tok.consume_and_return(RunExpr),
            (ExpectingLet { module: None, .. }, KWRet) => todo!(),

            (ExpectingIdent { visibility }, _) => todo!(),
            (ExpectingEq { visibility, is_ext, ident }, _) => todo!(),
            (ExpectingExpr { visibility, ident }, _) => todo!("Parse expression"),
        }
    }
    partial.try_into()
}

#[derive(Debug)]
struct ModuleInfo {
    visibility: Visibility,
    is_ext: bool,
}
impl ModuleInfo {
    fn private() -> Self {
        Self { visibility: Visibility::Private, is_ext: false }
    }
    fn public() -> Self {
        Self { visibility: Visibility::Public, is_ext: false }
    }
    fn external(self) -> Self {
        self.is_ext = true;
        self
    }
}

#[derive(Debug)]
pub(crate) enum PartialDeclaration {
    ModuleStart,
    ExpectingExt {
        module: Option<ModuleInfo>,
    },
    ExpectingLet {
        module: Option<ModuleInfo>
    },
    ExpectingIdent {
        module: Option<ModuleInfo>,
    },
    ExpectingEq {
        module: Option<ModuleInfo>,
        ident: Ident,
    },
    RunExpr,
    ExpectingExpr {
        module: Option<ModuleInfo>,
        ident: Option<Ident>,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationError {
    NonStart,
}
impl TryFrom<PartialDeclaration> for ast::declaration::ModuleDeclaration {
    type Error = DeclarationError;

    fn try_from(value: PartialDeclaration) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<PartialDeclaration> for ast::declaration::BlockDeclaration {
    type Error = DeclarationError;

    fn try_from(value: PartialDeclaration) -> Result<Self, Self::Error> {
        todo!()
    }
}
