use declaration::ModuleDeclaration;
pub use import::Import;

pub mod core;
pub mod expr;
#[cfg(test)]
mod conversion_tests;

pub mod declaration;
pub mod expression;
pub mod pattern;

//TODO:
pub type Ident = String;

#[derive(Debug, Clone)]
pub enum Type {
    Function(Box<Self>, Box<Self>),
    Union(Box<Self>, Box<Self>),
    Parameters(Box<Self>, Vec<Self>),
    Tagged(Ident, Vec<Self>),
    Record(Vec<(Ident, Self)>),
    Grouped(Box<Self>),
    Named(Ident),
    Undefined,
    Any,
}

#[derive(Debug, Default)]
pub struct Module {
    imports: Vec<Import>,
    declarations: Vec<(Visibility, ModuleDeclaration)>,
}
impl Module {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }
    // pub fn remove_import_by_path(&mut self, path: &str) {
    //     self.imports.retain(|i| i.path != path)
    // }
    pub fn add_declaration(&mut self, visibility: Visibility, declaration: ModuleDeclaration) {
        self.declarations.push((visibility, declaration));
    }
    pub fn has_declarations(&self) -> bool {
        self.declarations.len() > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

pub mod import {
    use crate::Ident;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Qualifier {
        Pkg,
        Ext,
        None,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Import {
        qualifier: Qualifier,
        path: String,
        alias: Vec<String>,
        bindings: Vec<Ident>,
    }
    impl Import {
        pub fn new<S: Into<String>>(
            qualifier: Qualifier,
            path: S,
            alias: Vec<String>,
            bindings: Vec<Ident>,
        ) -> Self {
            Self {
                qualifier,
                path: path.into(),
                alias,
                bindings,
            }
        }
        pub fn get_qualifier(&self) -> Qualifier {
            self.qualifier
        }
        pub fn get_path(&self) -> &str {
            &self.path
        }
        pub fn get_alias(&self) -> Option<&Vec<String>> {
            Some(&self.alias)
        }
        pub fn get_bindings(&self) -> Option<&Vec<Ident>> {
            Some(&self.bindings)
        }
        // /// Get all the exposed namespaces and fields (last part of alias and all exposed fields)
        // pub fn get_all_exposed(&self) -> Vec<&String> {
        //     self.alias.last().into_iter().chain(self.bindings.iter()).collect()
        // }
    }
}
