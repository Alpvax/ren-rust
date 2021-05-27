mod lexer;

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

type ImportError = String;
pub fn parse_import(lexer: &mut Lexer) -> Result<ast::import::Import, ImportError> {
    let path = match (lexer.next_token(), lexer.next_token(), lexer.next_token()) {
        (None, ..) => Err("no tokens to parse".to_owned()),
        (Some(Token::KWImport), Some(Token::Whitespace), Some(Token::String(s))) => Ok(s),
        (Some(Token::KWImport), Some(Token::Whitespace), t) => {
            Err(format!("Missing path: {:?}", t))
        }
        (Some(Token::KWImport), ..) => Err(
            "The \"import\" keyword must be followed by whitespace, then a string path".to_owned(),
        ),
        (_, ..) => Err("import statements must start with the \"import\" keyword".to_owned()),
    }?;
    let namespace = if let [Some(Token::Whitespace), Some(Token::KWAs)] =
        lexer.peek_n::<2>().as_token_array()
    {
        lexer.next();
        lexer.next();
        match lexer.next_token() {
            Some(Token::Whitespace) => parse_namespace(lexer).map(Some).map_err(NamespaceError::into),
            _ => Err("The \"as\" keyword in an import statement must be followed by a '.' seperated namespace".to_owned())
        }
        /*match (lexer.next(), lexer.next_token(), lexer.next_token()) {
            (_, Some(Token::Whitespace), Some(Token::Namespace(s))) => {
                let mut ns = vec![s];
                loop {
                    match lexer.peek_token() {
                        Some(Token::Whitespace) | None => break,
                        Some(Token::Period) => {
                            match (lexer.next(), lexer.next_token()) {
                                (_, Some(Token::Namespace(s))) => ns.push(*s),
                                (_, Some(_)) => return Err("A namespace must consist of a series of Namespace tokens seperated by '.' tokens".to_owned()),
                                (_, None) => return Err("A trailing '.' is not allowed in a namespace".to_owned()),
                            }
                        },
                        _ => return Err("The \"as\" keyword in an import statement must be followed by a '.' seperated namespace with no spaces".to_owned())
                    }
                }
                Ok(Some(ns))
            },
            _ => Err("The \"as\" keyword in an import statement must be followed by a '.' seperated namespace".to_owned())
        }*/
    } else {
        Ok(None)
    }?;
    let exposing = if let [Some(Token::Whitespace), Some(Token::KWExposing)] =
        lexer.peek_n::<2>().as_token_array()
    {
        lexer.next();
        lexer.next();
        consume_whitespace(lexer);
        if let Some(Token::CurlyOpen) = lexer.next_token() {
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
                    None => return Err("Non-closed exposing block".to_owned()),
                    _ => return Err("Invalid token in exposing block".to_owned()),
                }
            }
            if names.len() > 0 {
                Ok(Some(names))
            } else {
                Err("the contents of the exposing block must be a comma seperated list of VarName tokens".to_owned())
            }
        } else {
            Err(
                "The \"exposing\" keyword must be followed by '{' (whitespace allowed optionally)"
                    .to_owned(),
            )
        }
    } else {
        Ok(None)
    }?;
    Ok(ast::import::Import::new_from_owned(
        path, namespace, exposing,
    ))
}

type ModuleParseError = String;
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
                            Err(e) => errors.push(e),
                        }
                    } else {
                        errors.push("import statements are not allowed after declarations (must be at the top of the file)".to_owned())
                    }
                }
                Token::KWLet => {}
                Token::KWFun => {}
                Token::KWPub => {}
                _ => errors.push("Unexpected token!".to_owned()),
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
