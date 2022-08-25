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

// CONSTRUCTORS ----------------------------------------------------------------

// local : String -> List String -> List String -> Import
// local path name unqualified =
//     { path = path
//     , source = Local
//     , name = name
//     , unqualified = unqualified
//     }

// package : String -> List String -> List String -> Import
// package path name unqualified =
//     { path = path
//     , source = Package
//     , name = name
//     , unqualified = unqualified
//     }

// external : String -> List String -> List String -> Import
// external path name unqualified =
//     { path = path
//     , source = External
//     , name = name
//     , unqualified = unqualified
//     }

// QUERIES ---------------------------------------------------------------------

// isLocal : Import -> Bool
// isLocal =
//     .source >> (==) Local

// isPackage : Import -> Bool
// isPackage =
//     .source >> (==) Package

// isExternal : Import -> Bool
// isExternal =
//     .source >> (==) External

// alike : Import -> Import -> Bool
// alike a b =
//     a.path == b.path && a.source == b.source && a.name == b.name

// MANIPULATIONS ---------------------------------------------------------------

// merge : Import -> Import -> Import
// merge a b =
//     if alike a b then
//         { a | unqualified = List.uniques <| a.unqualified ++ b.unqualified }

//     else
//         a
