mod declaration;
mod expression;
mod import;
mod lexer;

mod test;

use crate::ast;
pub use lexer::{Lexer, Token};

pub use self::declaration::{
    parse_declaration, parse_toplevel_declaration, Error as DeclarationError,
};
pub use self::expression::{
    literal::{parse_literal, Error as LiteralError},
    parse_expression, parse_pattern, Error as ExpressionError, PatternParseError,
};
pub use self::import::{parse_import, Error as ImportError};

fn consume_whitespace(lexer: &mut Lexer) -> bool {
    if let Some(Token::Whitespace) = lexer.peek_token() {
        lexer.next();
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceError {
    None,
    InvalidTokenNs,
    InvalidTokenSep,
    TrailingPeriod,
}
impl std::fmt::Display for NamespaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamespaceError::None => write!(f, "No namespace token"),
            NamespaceError::InvalidTokenNs => {
                write!(f, "Invalid segment. Expected a Namespace token")
            }
            NamespaceError::InvalidTokenSep => {
                write!(f, "Invalid segment. Expected a Period token")
            }
            NamespaceError::TrailingPeriod => {
                write!(f, "A trailing '.' is not allowed in a namespace")
            }
        }
    }
}
pub fn parse_namespace(lexer: &mut Lexer) -> Result<Vec<ast::Namespace>, NamespaceError> {
    match lexer.next_token() {
        Some(Token::Namespace(s)) => {
            let mut ns = vec![s];
            loop {
                match lexer.peek_token() {
                    Some(Token::Whitespace) | None => break,
                    Some(Token::Period) => match (lexer.next(), lexer.next_token()) {
                        (_, Some(Token::Namespace(s))) => ns.push(s),
                        (_, Some(_)) => return Err(NamespaceError::InvalidTokenNs),
                        (_, None) => return Err(NamespaceError::TrailingPeriod),
                    },
                    _ => return Err(NamespaceError::InvalidTokenSep),
                }
            }
            Ok(ns)
        }
        _ => Err(NamespaceError::None),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleParseError {
    ImportErr(ImportError),
    ImportBelowDeclaration,
    DeclarationErr(DeclarationError),
    UnexpectedToken,
}
impl std::fmt::Display for ModuleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleParseError::ImportErr(e) => write!(f, "Import error: {}", e),
            ModuleParseError::ImportBelowDeclaration => write!(f, "import statements are not allowed after declarations (must be at the top of the file)"),
            ModuleParseError::DeclarationErr(e) => write!(f, "Declaration error:{}", e),
            ModuleParseError::UnexpectedToken => write!(f, "Unexpected token!"),
        }
    }
}
impl From<ImportError> for ModuleParseError {
    fn from(e: ImportError) -> Self {
        Self::ImportErr(e)
    }
}
impl From<DeclarationError> for ModuleParseError {
    fn from(e: DeclarationError) -> Self {
        Self::DeclarationErr(e)
    }
}

pub fn parse<'s>(input: &str) -> Result<ast::Module, Vec<ModuleParseError>> {
    let mut lexer = Lexer::new(input);
    let mut builder = ast::Module::new();
    let mut errors = Vec::new();
    loop {
        if let Some(tok) = lexer.peek_token() {
            match tok {
                Token::KWImport => {
                    if builder.has_declarations() {
                        errors.push(ModuleParseError::ImportBelowDeclaration)
                    } else {
                        match parse_import(&mut lexer) {
                            Ok(i) => builder.add_import(i),
                            Err(e) => errors.push(e.into()),
                        }
                    }
                }
                Token::KWLet | Token::KWFun | Token::KWPub => {
                    match parse_toplevel_declaration(&mut lexer) {
                        Ok(d) => builder.add_declaration(d),
                        Err(e) => errors.push(e.into()),
                    }
                }
                _ => errors.push(ModuleParseError::UnexpectedToken),
            }
        } else {
            break;
        }
    }
    if errors.len() < 1 {
        Ok(builder)
    } else {
        Err(errors)
    }
}
