use std::collections::HashMap;

use super::expression::Expression;
use crate::ast::VarName;

pub enum DeclarationType {
    Function(usize),
    Variable,
}

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

pub struct Declaration {
    name: BindingPattern,
    declaration_type: DeclarationType,
    body: DeclarationBody,
}
