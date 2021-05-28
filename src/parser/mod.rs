mod lexer;

mod test;

use crate::ast;
pub use lexer::{Lexer, Token};

#[derive(Debug)]
pub struct ModuleBuilder {
    imports: Vec<ast::import::Import>,
    declarations: Vec<() /*ast::declaration::Declaration*/>,
}
impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

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
impl Into<String> for NamespaceError {
    fn into(self) -> String {
        match self {
            NamespaceError::None => "No namespace token",
            NamespaceError::InvalidTokenNs => "Invalid segment. Expected a Namespace token",
            NamespaceError::InvalidTokenSep => "Invalid segment. Expected a Period token",
            NamespaceError::TrailingPeriod => "A trailing '.' is not allowed in a namespace",
        }
        .to_owned()
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
pub enum ImportError {
    NoTokens,
    NoPath,
    MissingImportWhitespace,
    InvalidStart,
    MissingAsPrecWhitespace,
    MissingAsWhitespace,
    NamespaceError(NamespaceError),
    MissingExpPrecWhitespace,
    MissingCurlyOpen,
    MissingCurlyClose,
    InvalidExposedBlockToken,
    EmptyExposed,
}
impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::NoTokens => write!(f, "no tokens to parse"),
            ImportError::NoPath => write!(f, "Missing path"),
            ImportError::MissingImportWhitespace => write!(f, "The \"import\" keyword must be followed by whitespace, then a string path"),
            ImportError::InvalidStart => write!(f, "import statements must start with the \"import\" keyword"),
            ImportError::MissingAsPrecWhitespace => write!(f, "The \"as\" keyword must be preceded by whitespace"),
            ImportError::MissingAsWhitespace => write!(f, "The \"as\" keyword must be followed by whitespace, then a '.' seperated namespace"),
            ImportError::NamespaceError(e) => write!(f, "{:?}", e),
            ImportError::MissingExpPrecWhitespace => write!(f, "The \"exposing\" keyword must be preceded by whitespace"),
            ImportError::MissingCurlyOpen => write!(f, "The \"exposing\" keyword must be followed by '{{' (whitespace allowed optionally)"),
            ImportError::MissingCurlyClose => write!(f, "Non-closed exposing block"),
            ImportError::InvalidExposedBlockToken => write!(f, "Invalid token in exposing block"),
            ImportError::EmptyExposed => write!(f, "the contents of the exposing block must be a comma seperated list of VarName tokens"),
        }
    }
}
impl From<NamespaceError> for ImportError {
    fn from(e: NamespaceError) -> Self {
        Self::NamespaceError(e)
    }
}

pub fn parse_import(lexer: &mut Lexer) -> Result<ast::import::Import, ImportError> {
    let path = match (lexer.next_token(), lexer.next_token(), lexer.next_token()) {
        (None, ..) => Err(ImportError::NoTokens),
        (Some(Token::KWImport), Some(Token::Whitespace), Some(Token::StringLit(s))) => Ok(s),
        (Some(Token::KWImport), Some(Token::Whitespace), _t) => Err(ImportError::NoPath),
        (Some(Token::KWImport), ..) => Err(ImportError::MissingImportWhitespace),
        (_, ..) => Err(ImportError::InvalidStart),
    }?;
    let namespace = if let [Some(Token::Whitespace), Some(Token::KWAs)] =
        lexer.peek_n::<2>().as_token_array()
    {
        lexer.next();
        lexer.next();
        match lexer.next_token() {
            Some(Token::Whitespace) => parse_namespace(lexer)
                .map(Some)
                .map_err(NamespaceError::into),
            _ => Err(ImportError::MissingAsWhitespace),
        }
    } else if let Some(Token::KWAs) = lexer.peek_token() {
        Err(ImportError::MissingAsPrecWhitespace)
    } else {
        Ok(None)
    }?;
    let exposing = if let [Some(Token::Whitespace), Some(Token::KWExposing)] =
        lexer.peek_n::<2>().as_token_array()
    {
        lexer.next();
        lexer.next();
        consume_whitespace(lexer);
        if let Some(Token::CurlyOpen) = lexer.peek_token() {
            lexer.next();
            let mut names = Vec::new();
            loop {
                consume_whitespace(lexer);
                match lexer.peek_token() {
                    Some(Token::VarName(_)) => {
                        if let Some(Token::VarName(s)) = lexer.next_token() {
                            names.push(s);
                            consume_whitespace(lexer);
                            if let Some(Token::Comma) = lexer.peek_token() {
                                lexer.next();
                                consume_whitespace(lexer);
                            }
                        }
                    }
                    Some(Token::CurlyClose) => {
                        lexer.next();
                        break;
                    }
                    None => return Err(ImportError::MissingCurlyClose),
                    _ => return Err(ImportError::InvalidExposedBlockToken),
                }
            }
            if names.len() > 0 {
                Ok(Some(names))
            } else {
                Err(ImportError::EmptyExposed)
            }
        } else {
            Err(ImportError::MissingCurlyOpen)
        }
    } else if let Some(Token::KWAs) = lexer.peek_token() {
        Err(ImportError::MissingExpPrecWhitespace)
    } else {
        Ok(None)
    }?;
    Ok(ast::import::Import::new_from_owned(
        path, namespace, exposing,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleParseError {
    ImportErr(ImportError),
    ImportBelowDeclaration,
    DeclarationErr(),
    UnexpectedToken,
}
impl std::fmt::Display for ModuleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleParseError::ImportErr(e) => write!(f, "Import error: {}", e),
            ModuleParseError::ImportBelowDeclaration => write!(f, "import statements are not allowed after declarations (must be at the top of the file)"),
            ModuleParseError::DeclarationErr() => write!(f, "Declaration error: UNKNOWN"),
            ModuleParseError::UnexpectedToken => write!(f, "Unexpected token!"),
        }
    }
}
impl From<ImportError> for ModuleParseError {
    fn from(e: ImportError) -> Self {
        Self::ImportErr(e)
    }
}

pub fn parse<'s>(input: &str) -> Result<ModuleBuilder, Vec<ModuleParseError>> {
    let mut lexer = Lexer::new(input);
    let mut builder = ModuleBuilder::new();
    let mut errors = Vec::new();
    loop {
        if let Some(tok) = lexer.peek_token() {
            match tok {
                Token::KWImport => {
                    if builder.declarations.len() < 1 {
                        match parse_import(&mut lexer) {
                            Ok(i) => builder.imports.push(i),
                            Err(e) => errors.push(e.into()),
                        }
                    } else {
                        errors.push(ModuleParseError::ImportBelowDeclaration)
                    }
                }
                Token::KWLet => {}
                Token::KWFun => {}
                Token::KWPub => {}
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
