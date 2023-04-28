#![allow(dead_code)] //XXX
use ren_json_derive::RenJson;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub source: Source,
    pub path: String,
    pub alias: Vec<String>,
    // pub unqualified: Vec<String>,
}
//TODO: Import tagged serialise

#[derive(Debug, Clone, Copy, PartialEq, Eq, RenJson)]
pub enum Source {
    External,
    Package,
    Project,
}

impl Import {
    // CONSTRUCTORS ----------------------------------------------------------------
    pub fn project(path: String, alias: Vec<String> /* , unqualified: Vec<String>*/) -> Self {
        Self {
            source: Source::Project,
            path,
            alias,
            // unqualified,
        }
    }

    pub fn package(path: String, alias: Vec<String> /* , unqualified: Vec<String>*/) -> Self {
        Self {
            source: Source::Package,
            path,
            alias,
            // unqualified,
        }
    }

    pub fn external(path: String, alias: Vec<String> /* , unqualified: Vec<String>*/) -> Self {
        Self {
            source: Source::External,
            path,
            alias,
            // unqualified,
        }
    }

    // QUERIES ---------------------------------------------------------------------

    pub fn is_project(&self) -> bool {
        self.source == Source::Project
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
    //         { a | unqualified = List.uniques <| a.unqualified <> b.unqualified }

    //     else
    //         a

    // CONVERSION ------------------------------------------------------------------
    pub fn to_json_str(&self, _pretty: bool) -> ::serde_json::Result<String> {
        Ok("TODO: serialise Import to json".to_string())
        /*
        if pretty {
            ::serde_json::to_string_pretty(self)
        } else {
            ::serde_json::to_string(self)
        }*/
    }
    pub fn to_json_bytes(&self, _pretty: bool) -> ::serde_json::Result<Vec<u8>> {
        Ok("TODO: serialise Import to json".bytes().collect())
        /*
        if pretty {
            ::serde_json::to_string_pretty(self)
        } else {
            ::serde_json::to_string(self)
        }*/
    }
    pub fn to_json_writer<W: std::io::Write>(
        &self,
        mut _w: W,
        _pretty: bool,
    ) -> ::serde_json::Result<()> {
        Ok(())
        /*TODO: serialise Import to json
        if pretty {
            ::serde_json::to_writer_pretty(&mut w, self)
        } else {
            ::serde_json::to_writer(&mut w, self)
        }*/
    }
}
