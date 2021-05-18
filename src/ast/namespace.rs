use std::collections::HashMap;
//use std::path::Path;

use super::names::*;
use crate::value::ValueType;

#[derive(Debug)]
pub struct Module {
    path: String, //Box<dyn AsRef<Path>>,
    name: Option<Namespace>,
    namespace: NamespaceType,
}

#[derive(Debug)]
pub struct NamespaceType {
    public: HashMap<Identifier, ValueType>,
    internal: HashMap<Identifier, ValueType>,
}
