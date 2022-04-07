use std::convert::{TryFrom, TryInto};

use crate::Token;

pub fn parse_import<'source>(
    lexer: &'source mut lexer::Lexer<'source, Token>,
) -> Result<ast::Import, ImportError> {
    use lexer::Token::*;
    use PartialImport::*;
    lexer.skip_whitespace(true);
    let mut partial = Start;
    while let Some(tok) = lexer.read() {
        partial = match (partial, tok.clone_token()) {
            (Start, KWImport) => tok.consume_and_return(ExpectingQualifier),
            (ExpectingQualifier, KWPkg) => tok.consume_and_return(ExpectingPath {
                qualifier: ast::import::Qualifier::Pkg,
            }),
            (ExpectingQualifier, KWExt) => tok.consume_and_return(ExpectingPath {
                qualifier: ast::import::Qualifier::Ext,
            }),
            (ExpectingQualifier, SingleQuote)
            | (ExpectingQualifier, DoubleQuote)
            | (ExpectingQualifier, StringSegment(..)) => ExpectingPath {
                qualifier: ast::import::Qualifier::None,
            },
            // (ExpectingPath { qualifier }, SingleQuote) |
            // (ExpectingPath { qualifier }, DoubleQuote) => todo!("Parse path"),
            (
                ExpectingPath { qualifier },
                StringSegment(lexer::token::string::StringSegment::Full(s, _)),
            ) => tok.consume_and_return(ExpectingAs { qualifier, path: s }),
            (ExpectingAs { qualifier, path }, KWAs) => {
                tok.consume_and_return(ExpectingAlias { qualifier, path })
            }
            (ExpectingAs { qualifier, path }, KWExposing) => ExpectingExp {
                qualifier: qualifier,
                path,
                alias: Vec::new(),
            },
            (ExpectingAlias { qualifier, path }, Namespace(s)) => {
                tok.consume_and_return(ExpectingExp {
                    qualifier,
                    path,
                    alias: vec![s],
                })
            }
            (
                ExpectingAliasSegment {
                    qualifier,
                    path,
                    mut alias,
                },
                Namespace(s),
            ) => {
                alias.push(s);
                tok.consume_and_return(ExpectingExp {
                    qualifier,
                    path,
                    alias,
                })
            }
            (
                ExpectingExp {
                    qualifier,
                    path,
                    alias,
                },
                Period,
            ) => tok.consume_and_return(ExpectingAliasSegment {
                qualifier,
                path,
                alias,
            }),
            (
                ExpectingExp {
                    qualifier,
                    path,
                    alias,
                },
                KWExposing,
            ) => tok.consume_and_return(ExpectingBindingsOpen {
                qualifier,
                path,
                alias,
            }),
            (
                ExpectingBindingsOpen {
                    qualifier,
                    path,
                    alias,
                },
                CurlyOpen,
            ) => tok.consume_and_return(ExpectingBinding {
                qualifier,
                path,
                alias,
                bindings: Vec::new(),
            }),
            (
                ExpectingBinding {
                    qualifier,
                    path,
                    alias,
                    mut bindings,
                },
                VarName(s),
            ) => {
                bindings.push(s);
                tok.consume_and_return(ExpectingBindingDelim {
                    qualifier,
                    path,
                    alias,
                    bindings,
                })
            }
            (
                ExpectingBindingDelim {
                    qualifier,
                    path,
                    alias,
                    bindings,
                },
                Comma,
            ) => tok.consume_and_return(ExpectingBinding {
                qualifier,
                path,
                alias,
                bindings,
            }),
            (
                ExpectingBindingDelim {
                    qualifier,
                    path,
                    alias,
                    bindings,
                },
                CurlyClose,
            ) => {
                return tok
                    .consume_and_return(Ok(ast::Import::new(qualifier, path, alias, bindings)));
            }
            (p, _) => return p.try_into(),
        }
    }
    partial.try_into()
}

#[derive(Debug)]
pub(crate) enum PartialImport<S> {
    //'source> {
    Start,
    ExpectingQualifier,
    ExpectingPath {
        qualifier: ast::import::Qualifier,
    },
    ExpectingAs {
        qualifier: ast::import::Qualifier,
        path: S,
    },
    ExpectingAlias {
        qualifier: ast::import::Qualifier,
        path: S,
    },
    ExpectingAliasSegment {
        qualifier: ast::import::Qualifier,
        path: S,
        alias: Vec<S>,
    },
    ExpectingExp {
        qualifier: ast::import::Qualifier,
        path: S,
        alias: Vec<S>,
    },
    ExpectingBindingsOpen {
        qualifier: ast::import::Qualifier,
        path: S,
        alias: Vec<S>,
    },
    ExpectingBinding {
        qualifier: ast::import::Qualifier,
        path: S,
        alias: Vec<S>,
        bindings: Vec<S>,
    },
    ExpectingBindingDelim {
        qualifier: ast::import::Qualifier,
        path: S,
        alias: Vec<S>,
        bindings: Vec<S>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportError {
    NonStart,
    MissingPath,
    MissingAlias,
    MissingAliasSegment,
    MissingBindings,
    MissingBinding,
    MissingCloseExpose,
}
impl TryFrom<PartialImport<String>> for ast::Import {
    type Error = ImportError;

    fn try_from(value: PartialImport<String>) -> Result<Self, Self::Error> {
        match value {
            PartialImport::Start => Err(ImportError::NonStart),
            PartialImport::ExpectingQualifier => Err(ImportError::MissingPath),
            PartialImport::ExpectingPath { .. } => Err(ImportError::MissingPath),
            PartialImport::ExpectingAs { qualifier, path } => {
                Ok(Self::new(qualifier, path, Vec::new(), Vec::new()))
            }
            PartialImport::ExpectingAlias { .. } => Err(ImportError::MissingAlias),
            PartialImport::ExpectingAliasSegment { .. } => Err(ImportError::MissingAliasSegment),
            PartialImport::ExpectingExp {
                qualifier,
                path,
                alias,
            } => Ok(Self::new(qualifier, path, alias, Vec::new())),
            PartialImport::ExpectingBindingsOpen { .. } => Err(ImportError::MissingBindings),
            PartialImport::ExpectingBinding { .. } => Err(ImportError::MissingBinding),
            PartialImport::ExpectingBindingDelim { .. } => Err(ImportError::MissingCloseExpose),
        }
    }
}
