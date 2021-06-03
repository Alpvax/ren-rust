use super::expression::{Expression, Pattern};
use crate::ast::VarName;

#[derive(Debug)]
pub enum Visibility {
    Public,
    Private,
}
impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Debug)]
pub enum Definition {
    Function { name: VarName, args: Vec<Pattern> },
    Variable { name: Pattern },
}

#[derive(Debug)]
pub struct Declaration {
    comment: Vec<String>,
    visibility: Visibility,
    definition: Definition,
    bindings: Vec<Declaration>,
    body: Expression,
}

impl Declaration {
    pub fn new(
        comment: Vec<String>,
        visibility: Visibility,
        definition: Definition,
        bindings: Vec<Declaration>,
        body: Expression,
    ) -> Self {
        Self {
            comment,
            visibility,
            definition,
            bindings,
            body,
        }
    }
    pub fn set_public(mut self) -> Self {
        self.visibility = Visibility::Public;
        self
    }
}

/*
#[derive(Debug)]
pub enum DeclarationType {
    Function(usize),
    Variable,
}

#[derive(Debug)]
pub enum DeclarationBody {
    Simple(Expression),
    Block(Vec<Declaration>, Expression),
}

#[derive(Debug, Clone)]
pub enum BindingPattern {
    Name(VarName),
    ArrDestructure(Vec<BindingPattern>),
    ObjDestructure(HashMap<VarName, BindingPattern>),
    Wildcard(Option<VarName>),
}

#[derive(Debug)]
pub struct Declaration {
    name: BindingPattern,
    declaration_type: DeclarationType,
    body: DeclarationBody,
}
*/
