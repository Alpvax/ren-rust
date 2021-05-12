use crate::names::{Namespace, VarName};

#[derive(Debug)]
pub struct Import {
    path: String,
    namespace: Option<Vec<Namespace>>,
    exposing: Option<Vec<VarName>>,
}

impl Import {
    pub fn new(path: &str, namespace: Option<Vec<&str>>, exposing: Option<Vec<&str>>) -> Import {
        Import {
            path: path.to_owned(),
            namespace: namespace.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
            exposing: exposing.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
        }
    }
}
