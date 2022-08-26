#![allow(dead_code)] //XXX
use ren_json_derive::RenJson;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub path: String,
    pub source: Source,
    pub name: Vec<String>,
    pub unqualified: Vec<String>,
}
//TODO: Import tagged serialise

#[derive(Debug, Clone, Copy, PartialEq, Eq, RenJson)]
pub enum Source {
    Local,
    Package,
    External,
}

impl Import {
    // CONSTRUCTORS ----------------------------------------------------------------
    pub fn local(path: String, name: Vec<String>, unqualified: Vec<String>) -> Self {
        Self {
            path,
            source: Source::Local,
            name,
            unqualified,
        }
    }

    pub fn package(path: String, name: Vec<String>, unqualified: Vec<String>) -> Self {
        Self {
            path,
            source: Source::Package,
            name,
            unqualified,
        }
    }

    pub fn external(path: String, name: Vec<String>, unqualified: Vec<String>) -> Self {
        Self {
            path,
            source: Source::External,
            name,
            unqualified,
        }
    }

    // QUERIES ---------------------------------------------------------------------

    pub fn is_local(&self) -> bool {
        self.source == Source::Local
    }
    pub fn is_package(&self) -> bool {
        self.source == Source::Package
    }
    pub fn is_external(&self) -> bool {
        self.source == Source::External
    }

    // pub fn alike(&self, other: &Self) -> bool {
    //     self.path == other.path && self.source == other.source && self.name == other.name
    // }

    // MANIPULATIONS ---------------------------------------------------------------

    // merge : Import -> Import -> Import
    // merge a b =
    //     if alike a b then
    //         { a | unqualified = List.uniques <| a.unqualified ++ b.unqualified }

    //     else
    //         a
}
