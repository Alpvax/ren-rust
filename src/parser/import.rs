use crate::ast::import::Import;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
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
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTokens => write!(f, "No tokens to parse"),
            Error::NoPath => write!(f, "Missing path"),
            Error::MissingImportWhitespace => write!(f, "The \"import\" keyword must be followed by whitespace, then a string path"),
            Error::InvalidStart => write!(f, "import statements must start with the \"import\" keyword"),
            Error::MissingAsPrecWhitespace => write!(f, "The \"as\" keyword must be preceded by whitespace"),
            Error::MissingAsWhitespace => write!(f, "The \"as\" keyword must be followed by whitespace, then a '.' seperated namespace"),
            Error::NamespaceError(e) => write!(f, "{:?}", e),
            Error::MissingExpPrecWhitespace => write!(f, "The \"exposing\" keyword must be preceded by whitespace"),
            Error::MissingCurlyOpen => write!(f, "The \"exposing\" keyword must be followed by '{{' (whitespace allowed optionally)"),
            Error::MissingCurlyClose => write!(f, "Non-closed exposing block"),
            Error::InvalidExposedBlockToken => write!(f, "Invalid token in exposing block"),
            Error::EmptyExposed => write!(f, "the contents of the exposing block must be a comma seperated list of VarName tokens"),
        }
    }
}
impl From<NamespaceError> for Error {
    fn from(e: NamespaceError) -> Self {
        Self::NamespaceError(e)
    }
}

pub fn parse_import(lexer: &mut Lexer) -> Result<Import, Error> {
    let path = match (lexer.next_token(), lexer.next_token(), lexer.next_token()) {
        (None, ..) => Err(Error::NoTokens),
        (Some(Token::KWImport), Some(Token::Whitespace), Some(Token::StringLit(s))) => Ok(s),
        (Some(Token::KWImport), Some(Token::Whitespace), _t) => Err(Error::NoPath),
        (Some(Token::KWImport), ..) => Err(Error::MissingImportWhitespace),
        (_, ..) => Err(Error::InvalidStart),
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
            _ => Err(Error::MissingAsWhitespace),
        }
    } else if let Some(Token::KWAs) = lexer.peek_token() {
        Err(Error::MissingAsPrecWhitespace)
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
                    None => return Err(Error::MissingCurlyClose),
                    _ => return Err(Error::InvalidExposedBlockToken),
                }
            }
            if names.len() > 0 {
                Ok(Some(names))
            } else {
                Err(Error::EmptyExposed)
            }
        } else {
            Err(Error::MissingCurlyOpen)
        }
    } else if let Some(Token::KWAs) = lexer.peek_token() {
        Err(Error::MissingExpPrecWhitespace)
    } else {
        Ok(None)
    }?;
    Ok(Import::new_from_owned(path, namespace, exposing))
}
