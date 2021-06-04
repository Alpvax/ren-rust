use std::convert::{TryFrom, TryInto};

use crate::ast::declaration::{Declaration, Definition, Visibility};
use crate::ast::expression::Expression;

use super::expression::{parse_expression, parse_object_literal, parse_pattern, PatternParseError};
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NoTokens,
    MissingPubWhitespace,
    InvalidStart,
    NoFunLet,
    MissingRet,
    InvalidBlockStatement,
    UnclosedBlockDeclaration,
    InvalidBlockStart,
    MissingDefinition,
    MissingBody,
    MissingFunName,
    EmptyFunParams,
    MissingAssignOp,
    PatternError(PatternParseError),
    ExpressionError(expression::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTokens => write!(f, "No tokens to parse"),
            Error::MissingPubWhitespace => write!(f, "\"pub\" keyword must be followed by whitespace"),
            Error::InvalidStart => write!(f, "Declaration must start with \"let\" or \"fun\" (or \"pub\" if it is a toplevel declaration)"),
            Error::NoFunLet => write!(f, "Declaration must start with \"let\" or \"fun\""),
            Error::UnclosedBlockDeclaration => write!(f, "Block declaration must be terminated with '}}'"),
            Error::MissingRet => write!(f, "The final statement in a declaration block must start with the \"ret\" keyword"),
            Error::InvalidBlockStatement => write!(f, "Declaration blocks must be 0 or more declarations, followed by a single ret statement"),
            Error::InvalidBlockStart => write!(f, "Attempted to start a declaration block with a non '{{' token"),
            //Error::RetMustBeLast => write!(f, "The final statement in a declaration block must be a ret statement"),
            Error::MissingDefinition => write!(f, "Conversion error DeclarationBuilder -> Declaration: Missing definition"),
            Error::MissingBody => write!(f, "Conversion error DeclarationBuilder -> Declaration: Missing body"),
            Error::EmptyFunParams => write!(f, "fun declaration needs a list of parameters (patterns) between '=' and '=>'"),
            Error::MissingAssignOp => write!(f, "The name (fun declaration) or pattern (let declaration) needs to be followed by the assignment operator (=)"),
            Error::MissingFunName => write!(f, "the fun keyword must be followed by whitespace, then a VarName (lowercase, followed by 0+ alpahanumeric chars)"),
            Error::PatternError(e) => write!(f, "Pattern error:{}", e),
            Error::ExpressionError(e) => write!(f, "Expression error:{}", e),
        }
    }
}
impl From<expression::Error> for Error {
    fn from(e: expression::Error) -> Self {
        Self::ExpressionError(e)
    }
}
impl From<PatternParseError> for Error {
    fn from(e: PatternParseError) -> Self {
        Self::PatternError(e)
    }
}

struct DeclarationBuilder {
    comment: Vec<String>,
    visibility: Visibility,
    definition: Option<Definition>,
    bindings: Vec<Declaration>,
    body: Option<Expression>,
}
impl DeclarationBuilder {
    pub fn new() -> Self {
        Self {
            comment: Vec::new(),
            visibility: Visibility::default(),
            definition: None,
            bindings: Vec::new(),
            body: None,
        }
    }
}
impl TryFrom<DeclarationBuilder> for Declaration {
    type Error = Error;

    fn try_from(b: DeclarationBuilder) -> Result<Self, Self::Error> {
        Ok(Self::new(
            b.comment,
            b.visibility,
            b.definition.ok_or(Error::MissingDefinition)?,
            b.bindings,
            b.body.ok_or(Error::MissingBody)?,
        ))
    }
}

// Parse `(pub )?(fun|let) ... declarations
pub fn parse_toplevel_declaration(lexer: &mut Lexer) -> Result<Declaration, Error> {
    if let Some(l) = lexer.peek_n_exact::<2>() {
        match l.as_token_array_unchecked() {
            [Token::KWPub, Token::Whitespace] => {
                lexer.next(); //consume pub
                lexer.next(); //consume whitespace
                parse_declaration(lexer).map(Declaration::set_public)
            }
            [Token::KWPub, _] => Err(Error::MissingPubWhitespace),
            [Token::KWLet, _] | [Token::KWFun, _] => parse_declaration(lexer),
            _ => Err(Error::InvalidStart),
        }
    } else {
        Err(Error::NoTokens)
    }
}
/// Parse let/fun declarations. Consumes first 2 tokens regardeless!!
pub fn parse_declaration(lexer: &mut Lexer) -> Result<Declaration, Error> {
    if let [Some(tok), Some(Token::Whitespace)] = [lexer.next_token(), lexer.next_token()] {
        //TODO: Not consume?
        let mut builder = DeclarationBuilder::new();
        builder.definition = Some(match tok {
            Token::KWLet => parse_let_def(lexer),
            Token::KWFun => parse_fun_def(lexer),
            _ => Err(Error::NoFunLet),
        }?);
        consume_whitespace(lexer);
        if let Some(Token::CurlyOpen) = lexer.peek_token() {
            let block = parse_bindings_or_obj(lexer)?;
            builder.bindings = block.0;
            builder.body = Some(block.1);
        } else {
            builder.body = Some(parse_expression(lexer)?);
        }
        builder.try_into()
    } else {
        Err(Error::NoFunLet)
    }
}

/// Parse `name = args =>` section of a fun declaration
fn parse_fun_def(lexer: &mut Lexer) -> Result<Definition, Error> {
    use Token::{OpAssign, OpFun, VarName, Whitespace};
    match lexer.peek_n::<3>().as_token_array() {
        [Some(VarName(name)), Some(Whitespace), Some(OpAssign)]
        | [Some(VarName(name)), Some(OpAssign), _] => {
            lexer.next(); //Consume name
            consume_whitespace(lexer);
            lexer.next(); //Consume '='
            consume_whitespace(lexer);
            let mut args = Vec::new();
            loop {
                if let Some(tok) = lexer.peek_token() {
                    if tok == &OpFun {
                        lexer.next(); //Consume "=>"
                        break;
                    }
                    args.push(parse_pattern(lexer)?);
                    consume_whitespace(lexer);
                }
            }
            if args.len() < 1 {
                Err(Error::EmptyFunParams)
            } else {
                Ok(Definition::Function {
                    name: name.to_owned(),
                    args,
                })
            }
        }
        [Some(VarName(_)), ..] => Err(Error::MissingAssignOp),
        _ => Err(Error::MissingFunName),
    }
}

/// Parse `pattern =` section of a fun declaration
fn parse_let_def(lexer: &mut Lexer) -> Result<Definition, Error> {
    let name = parse_pattern(lexer)?;
    consume_whitespace(lexer);
    if let Some(Token::OpAssign) = lexer.peek_token() {
        lexer.next();
        Ok(Definition::Variable { name })
    } else {
        Err(Error::MissingAssignOp)
    }
}

/// Parse body starting with '{'
fn parse_bindings_or_obj(lexer: &mut Lexer) -> Result<(Vec<Declaration>, Expression), Error> {
    use Token::{CurlyClose, CurlyOpen, KWFun, KWLet, KWRet, Whitespace};
    match lexer.peek_n::<3>().as_token_array() {
        [Some(CurlyOpen), Some(Whitespace), Some(t)] | [Some(CurlyOpen), Some(t), _] => {
            match t {
                KWFun | KWLet | KWRet => {
                    lexer.next(); //Consume CurlyOpen
                    consume_whitespace(lexer);
                    let mut b = Vec::new();
                    loop {
                        match lexer.peek_token() {
                            Some(KWFun) | Some(KWLet) => b.push(parse_declaration(lexer)?),
                            Some(KWRet) => {
                                let body = parse_expression(lexer)?;
                                if let Some(CurlyClose) = lexer.peek_token() {
                                    lexer.next(); //Consume CurlyClose
                                    return Ok((b, body));
                                } else {
                                    return Err(Error::UnclosedBlockDeclaration);
                                }
                            }
                            Some(CurlyClose) => return Err(Error::MissingRet),
                            _ => return Err(Error::InvalidBlockStatement),
                        }
                    }
                }
                _ => Ok((Vec::new(), parse_object_literal(lexer)?)),
            }
        }
        _ => Err(Error::InvalidBlockStart),
    }
}
