use crate::ast::import::Import;

use super::{Builder, ModuleParser};

pub struct ImportParser {
    parent: ModuleParser,
}
impl Builder<Import, ()> for ImportParser {
    type SubValues = ();

    fn accept_token(self, _token: &super::NextToken) -> super::BuilderResult<Import, Self, ()>
    where
        Self: Sized,
    {
        todo!()
    }

    fn accept_value<V>(self, _value: Self::SubValues) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Err(())
    }

    fn _map_unfinished_to_error(self) -> () {
        todo!()
    }
}
