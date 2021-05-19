use crate::ast::import::Import;

use super::{ModuleParser, ParseTokenResult, TokenParser};

pub struct ImportParser {
    parent: ModuleParser,
}
impl TokenParser<Import, ()> for ImportParser {
    type SubParsers = ();

    fn accept_token(&mut self, token: &super::NextToken) -> ParseTokenResult<Import, (), Self::SubParsers> {
        todo!()
    }

    fn accept_value(&mut self, value: super::ParserValueType<Self, Import, ()>) -> ParseTokenResult<Import, (), Self::SubParsers> {
        todo!()
    }
}
