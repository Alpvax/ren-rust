use crate::ast;

use super::{ParentParser, ParentParserTokenResult, Parser};

#[derive(Debug)]
pub enum ModuleParseErr {
    ImportBelowDeclarations,
    UnexpectedEOF,
    UnexpectedToken,
}

pub enum SubValues {
    //TODO
    Import,
    Declaration,
}

pub enum SubParser {
    Import,
    Declaration,
}
impl Parser for SubParser {
    type Output = SubValues;

    type Error = ();

    fn try_parse_token(&mut self, token: super::SpannedToken) -> super::TokenParseResult {
        //todo!("parse subtype")
        super::TokenParseResult::Done
    }

    fn unwrap(self) -> Result<Self::Output, Vec<Self::Error>> {
        Ok(match self {
            SubParser::Import => SubValues::Import,
            SubParser::Declaration => SubValues::Declaration,
        })
    }

    fn push_error(&mut self, error: Self::Error) {
        todo!()
    }
}

pub struct ModuleParser {
    subparser: Option<SubParser>,
    imports: Vec<ast::import::Import>,
    imports_done: bool,
    declarations: Vec<ast::declaration::Declaration>,
    errors: Vec<(ModuleParseErr, logos::Span)>,
}
impl ModuleParser {
    pub fn new() -> Self {
        Self {
            subparser: None,
            imports: Vec::new(),
            imports_done: false,
            declarations: Vec::new(),
            errors: Vec::new(),
        }
    }
}
impl ParentParser for ModuleParser {
    type SubParser = SubParser;
    fn get_subparser(&mut self) -> Option<&mut Self::SubParser> {
        (&mut self.subparser).as_mut()
    }

    fn start_subparser(&mut self, sub: Self::SubParser) {
        self.subparser = Some(sub);
    }

    fn parse_token(
        &mut self,
        token: super::SpannedToken,
    ) -> (
        super::ParentParserTokenResult<Self::Error, Self::SubParser>,
        bool,
    ) {
        match token.0 {
            crate::Token::KWImport => (ParentParserTokenResult::SubParser(SubParser::Import), true),
            crate::Token::KWLet | crate::Token::KWFun | crate::Token::KWPub => (
                ParentParserTokenResult::SubParser(SubParser::Declaration),
                false,
            ),
            crate::Token::EOF => (ParentParserTokenResult::Done, true),
            _ => (
                ParentParserTokenResult::Error((ModuleParseErr::UnexpectedToken, token.1)),
                true,
            ),
        }
    }
}
parent_parser! {ModuleParser, ast::Module, (ModuleParseErr, logos::Span), {
        fn unwrap(self) -> Result<Self::Output, Vec<Self::Error>> {
            if self.errors.len() < 1 {
                Ok(ast::Module::new())
            } else {
                Err(self.errors)
            }
        }

        fn push_error(&mut self, error: Self::Error) {
            self.errors.push(error);
        }
    }
}
