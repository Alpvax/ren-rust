use crate::ast::import::Import;

use super::{Builder, ModuleParser};

pub struct ImportParser {
    parent: ModuleParser,
}
impl Builder<Import, ()> for ImportParser {
    fn accept_token(self, token: &super::NextToken) -> super::BuilderResult<Import, Self, ()>
    where
        Self: Sized,
    {
        todo!()
    }

    fn accept_value<V>(&self, value: V) -> Result<Self, ()>
    where
        Self: Sized,
    {
        todo!()
    }

    fn _map_unfinished_to_error(self) -> () {
        todo!()
    }
}
