pub type Namespace = String;
pub type VarName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    Namespace(Namespace),
    VarName(VarName),
}
