use crate::ast::expression::{Expression, Pattern};

use self::literal::{parse_literal, Error as LiteralError};

use super::*;

pub mod literal;

/*struct ExpressionBuilder {}

pub fn begin_expression() -> ExpressionBuilder {
    todo!("Expression Builder")
}*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NoTokens,
    InvalidLiteral,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTokens => write!(f, "No tokens to parse"),
            Error::InvalidLiteral => write!(f, "Invalid literal"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternParseError {
    NoTokens,
    EmptyDestructure,
    NonClosedDestructure,
    TrailingComma,
    MissingComma,
    InvalidObjKey,
    InvalidPattern,
}
impl std::fmt::Display for PatternParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternParseError::NoTokens => write!(f, "No tokens to parse"),
            PatternParseError::EmptyDestructure => {
                write!(f, "Invalid destructure pattern with nothing to destructure")
            }
            PatternParseError::NonClosedDestructure => write!(f, "Missing closing punctuation"),
            PatternParseError::TrailingComma => write!(
                f,
                "Trailing commas are not allowed in destructuring patterns"
            ),
            PatternParseError::MissingComma => {
                write!(f, "names / sub-patterns must be seperated by commas")
            }
            PatternParseError::InvalidObjKey => write!(
                f,
                "all keys use for object destructuring must match the pattern [a-z][a-zA-Z0-9]*"
            ),
            PatternParseError::InvalidPattern => write!(f, "Invalid token to begin pattern"),
        }
    }
}

//fn parse_comma_seperated(lexer: &mut Lexer, filter:)

pub fn parse_pattern(lexer: &mut Lexer) -> Result<Pattern, PatternParseError> {
    use Token::{Colon, Comma, CurlyClose, CurlyOpen, SquareClose, SquareOpen, VarName, Wildcard};
    if let Some(tok) = lexer.next_token() {
        match tok {
            SquareOpen => {
                consume_whitespace(lexer);
                let mut sub = Vec::new();
                loop {
                    if let Some(tok) = lexer.peek_token() {
                        if tok == &SquareClose {
                            lexer.next(); //Consume ']'
                            return Err(if sub.len() < 1 {
                                PatternParseError::EmptyDestructure
                            } else {
                                PatternParseError::TrailingComma
                            });
                        }
                        sub.push(parse_pattern(lexer)?);
                        consume_whitespace(lexer);
                        match lexer.next_token() {
                            Some(Comma) => {
                                consume_whitespace(lexer);
                            }
                            Some(SquareClose) => {
                                consume_whitespace(lexer);
                                break;
                            }
                            _ => return Err(PatternParseError::MissingComma),
                        }
                    }
                }
                Ok(Pattern::ArrayDestructure(sub))
            }
            CurlyOpen => {
                consume_whitespace(lexer);
                let mut names = Vec::new();
                loop {
                    if let Some(tok) = lexer.next_token() {
                        match tok {
                            CurlyClose => {
                                return Err(if names.len() < 1 {
                                    PatternParseError::EmptyDestructure
                                } else {
                                    PatternParseError::TrailingComma
                                });
                            }
                            VarName(name) => {
                                consume_whitespace(lexer);
                                if let Some(Colon) = lexer.peek_token() {
                                    lexer.next(); //Consume ':'
                                    consume_whitespace(lexer);
                                    names.push((name, Some(parse_pattern(lexer)?)));
                                    consume_whitespace(lexer);
                                } else {
                                    names.push((name, None));
                                }
                                match lexer.next_token() {
                                    /*Some(Colon) => {
                                        consume_whitespace(lexer);
                                        names.push((name, Some(parse_pattern(lexer)?)));
                                        consume_whitespace(lexer);
                                        match lexer.peek_token() {
                                            Some(CurlyClose) => {
                                                lexer.next(); //Consume '}'
                                                break;
                                            },
                                            Some(Comma) => {
                                                lexer.next(); //Consume ','
                                                consume_whitespace(lexer);
                                            }
                                        }
                                    }*/
                                    Some(Comma) => {
                                        consume_whitespace(lexer);
                                    }
                                    Some(CurlyClose) => {
                                        consume_whitespace(lexer);
                                        break;
                                    }
                                    _ => return Err(PatternParseError::MissingComma),
                                }
                            }
                            _ => return Err(PatternParseError::InvalidObjKey),
                        }
                    } else {
                        return Err(PatternParseError::NonClosedDestructure);
                    }
                }
                Ok(Pattern::ObjectDestructure(names))
            }
            VarName(name) => Ok(Pattern::Name(name)),
            Wildcard(name) => Ok(Pattern::Wildcard(name)),
            //TODO: Value pattern?
            _ => Err(PatternParseError::InvalidPattern),
        }
    } else {
        Err(PatternParseError::NoTokens)
    }
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Expression, Error> {
    parse_literal(lexer).map_or_else(
        |e| match e {
            LiteralError::NoTokens => Err(Error::NoTokens),
            LiteralError::NonLiteral => match lexer.peek_token() {
                Some(Token::ParenOpen) => todo!("Parse subexpression"),
                _ => todo!("Parse other expression types"),
            },
            LiteralError::InvalidStartToken => todo!("Invalid start token: {:?}", lexer.peek()),
            LiteralError::UnclosedObject => todo!("Unclosed object literal"),
            LiteralError::UnclosedArray => todo!("Unclosed array literal"),
            LiteralError::InvalidKey => todo!("Invalid token for object literal key"),
            LiteralError::DuplicateKey => todo!("Duplicate object literal key"),
            LiteralError::MissingValue => todo!("Missing value?"),
            LiteralError::InvalidValue(e) => Err(e),
            LiteralError::MissingComma => todo!("Missing delimiter"),
        },
        |l| Ok(Expression::Literal(l)),
    )
}

pub fn parse_object_literal(lexer: &mut Lexer) -> Result<Expression, Error> {
    literal::parse_object_literal(lexer).map_or_else(
        |e| match e {
            LiteralError::NoTokens => Err(Error::NoTokens),
            LiteralError::NonLiteral => Err(Error::InvalidLiteral),
            LiteralError::InvalidStartToken => todo!("Invalid start token: {:?}", lexer.peek()),
            LiteralError::UnclosedObject => todo!("Unclosed object literal"),
            LiteralError::UnclosedArray => unreachable!(),
            LiteralError::InvalidKey => todo!("Invalid token for object literal key"),
            LiteralError::DuplicateKey => todo!("Duplicate object literal key"),
            LiteralError::MissingValue => todo!("Missing value for object key"),
            LiteralError::InvalidValue(e) => Err(e),
            LiteralError::MissingComma => todo!("Missing delimiter"),
        },
        |l| Ok(Expression::Literal(l)),
    )
}
