use crate::ast::{Namespace, VarName};

#[derive(Debug)]
pub struct Import {
    pub path: String,
    pub namespace: Option<Vec<Namespace>>,
    pub exposing: Option<Vec<VarName>>,
}

impl Import {
    pub fn new(path: &str, namespace: Option<Vec<&str>>, exposing: Option<Vec<&str>>) -> Import {
        Import {
            path: path.to_owned(),
            namespace: namespace.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
            exposing: exposing.map(|v| v.iter().map(|&s| s.to_owned()).collect()),
        }
    }
    pub fn new_from_owned(
        path: String,
        namespace: Option<Vec<String>>,
        exposing: Option<Vec<String>>,
    ) -> Import {
        Import {
            path: path.to_owned(),
            namespace: namespace,
            exposing: exposing,
        }
    }
}
