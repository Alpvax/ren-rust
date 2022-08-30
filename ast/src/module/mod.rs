use serde::{Deserialize, Serialize};

use crate::decl::Decl;

pub mod import;
use import::Import;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    name: String,
    path: String,
    #[serde(rename = "pkgPath")]
    pkg_path: String,
    #[serde(rename = "usesFFI")]
    uses_ffi: bool,
}
impl Default for Meta {
    fn default() -> Self {
        Self {
            name: Default::default(),
            path: Default::default(),
            pkg_path: ".pkg/".to_string(),
            uses_ffi: false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Module(Meta, Vec<Import>, Vec<Decl>);
//TODO: Module serialise

impl Module {
    pub fn new<I, D>(meta: Meta, imports: I, declarations: D) -> Self
    where
        I: Iterator<Item = Import>,
        D: Iterator<Item = Decl>,
    {
        Self(meta, imports.collect(), declarations.collect())
    }
    // QUERIES ---------------------------------------------------------------------
    pub fn meta(&self) -> &Meta {
        &self.0
    }
    pub fn meta_mut(&mut self) -> &mut Meta {
        &mut self.0
    }

    pub fn imports(&self, name: &str) -> bool {
        self.1.iter().any(|imp| imp.path == name)
    }
    pub fn imports_local(&self, name: &str) -> bool {
        self.1.iter().any(|imp| imp.is_local() && imp.path == name)
    }
    pub fn imports_package(&self, name: &str) -> bool {
        self.1
            .iter()
            .any(|imp| imp.is_package() && imp.path == name)
    }
    pub fn imports_external(&self, name: &str) -> bool {
        self.1
            .iter()
            .any(|imp| imp.is_external() && imp.path == name)
    }

    pub fn declares(&self, name: &str) -> bool {
        self.2.iter().any(|decl| decl.name() == name)
    }
    pub fn declares_local(&self, name: &str) -> bool {
        self.2
            .iter()
            .any(|decl| decl.is_local() && decl.name() == name)
    }
    pub fn declares_external(&self, name: &str) -> bool {
        self.2
            .iter()
            .any(|decl| decl.is_external() && decl.name() == name)
    }

    pub fn exports(&self, name: &str) -> bool {
        self.2
            .iter()
            .any(|decl| decl.is_public() && decl.name() == name)
    }
    pub fn exports_local(&self, name: &str) -> bool {
        self.2
            .iter()
            .any(|decl| decl.is_public() && decl.is_local() && decl.name() == name)
    }
    pub fn exports_external(&self, name: &str) -> bool {
        self.2
            .iter()
            .any(|decl| decl.is_public() && decl.is_external() && decl.name() == name)
    }

    // MANIPULATIONS ---------------------------------------------------------------
    // fn push_import(&mut self, import: Import) {
    //     self.1.push(import);
    // }
    // fn push_imports<I>(&mut self, imports: I) where I: IntoIterator<Item = Import> {
    //     self.1.extend(imports);
    // }
    // fn push_decl(&mut self, decl: Decl) {
    //     self.2.push(import);
    // }
    // fn push_decl<I>(&mut self, decls: I) where I: IntoIterator<Item = Decl> {
    //     self.2.extend(imports);
    // }
}
