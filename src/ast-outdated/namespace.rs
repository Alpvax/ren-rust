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
impl Module {
    #[deprecated]
    pub fn new() -> Self {
        Self {
            path: String::new(),
            name: None,
            namespace: NamespaceType::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct NamespaceType {
    public: HashMap<Identifier, ValueType>,
    internal: HashMap<Identifier, ValueType>,
}
