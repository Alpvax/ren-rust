use super::expression::Operator;

pub type Namespace = String;
pub type VarName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    Scoped(Vec<Namespace>, VarName),
    Local(VarName),
    Operator(Operator),
    Field(VarName),
}
