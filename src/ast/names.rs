pub type Namespace = String;
pub type VarName = String;

#[derive(Debug, Clone)]
pub enum Identifier {
    Namespace,
    VarName,
}
