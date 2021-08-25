pub use declaration::Declaration;

pub mod declaration;
pub mod expression;

#[derive(Debug, Default)]
pub struct Module {
    imports: Vec<Import>,
    declarations: Vec<(Visibility, Declaration)>,
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
    pub fn remove_import_by_path(&mut self, path: &str) {
        self.imports.retain(|i| i.path != path)
    }
    pub fn add_declaration(&mut self, visibility: Visibility, declaration: Declaration) {
        self.declarations.push((visibility, declaration));
    }
    // pub fn remove_declaration(&mut self, declaration: Declaration) {
    //     self.declarations.push((visibility, declaration));
    // }
    pub fn has_declarations(&self) -> bool {
        self.declarations.len() > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

type Namespace = String;
type VarName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub name: Vec<Namespace>,
    pub bindings: Vec<VarName>,
}
