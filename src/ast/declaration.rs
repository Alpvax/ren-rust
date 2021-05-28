use std::collections::HashMap;

use super::expression::Expression;
use crate::ast::VarName;

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
