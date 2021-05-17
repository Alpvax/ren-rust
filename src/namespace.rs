use std::collections::HashMap;
use std::path::Path;

use crate::names::*;
use crate::value::ValueType;

pub struct Module {
    path: Box<dyn AsRef<Path>>,
    name: Option<Namespace>,
    namespace: NamespaceType,
}

pub struct NamespaceType {
    public: HashMap<Identifier, ValueType>,
    internal: HashMap<Identifier, ValueType>,
}
