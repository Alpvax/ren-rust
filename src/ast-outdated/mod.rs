#![allow(dead_code)]

pub mod declaration;
pub mod expression;
pub mod import;
pub mod names;
pub use names::{Identifier, Namespace, VarName};

pub use self::declaration::Declaration;
pub use self::import::Import;
pub mod namespace;
//pub use namespace::Module;

#[derive(Debug)]
pub struct Module {
    imports: Vec<Import>,
    declarations: Vec<Declaration>,
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
    pub fn add_declaration(&mut self, declaration: Declaration) {
        self.declarations.push(declaration);
    }
    pub fn has_declarations(&self) -> bool {
        self.declarations.len() > 0
    }
}
