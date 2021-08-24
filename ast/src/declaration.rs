use crate::{
    expression::{Expression, Pattern},
    VarName,
};

///TODO: tagged name?
type TaggedName = String;

#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        name: VarName,
        parameters: Vec<Pattern>,
        body: Expression,
    },
    Variable {
        name: Pattern,
        body: Expression,
    },
    Enum {
        name: TaggedName,
        variants: Vec<Variant>,
    },
}

//#[derive(Debug, Clone, PartialEq)]
pub type Variant = (TaggedName, u8);

impl Declaration {
    pub fn name_as_pattern(&self) -> Pattern {
        match self {
            Declaration::Function { name, .. } => Pattern::Name(name.clone()),
            Declaration::Variable { name, .. } => name.clone(),
            Declaration::Enum { variants, .. } => Pattern::ArrayDestructure(
                variants
                    .iter()
                    .map(|(tag, _)| Pattern::Name(tag.clone()))
                    .collect(),
            ),
        }
    }
    // pub fn body(&self) -> Option<&Expression> {
    //     match self {
    //         Declaration::Function { body, .. } => Some(body),
    //         Declaration::Variable { body, .. } => Some(body),
    //         Declaration::Enum { .. } => None,
    //     }
    // }
    pub fn references(&self, namespace: Option<&Vec<String>>, name: Option<&str>) -> bool {
        if namespace.is_none() && name.is_none() {
            return false;
        }
        match self {
            Declaration::Function { body, .. } | Declaration::Variable { body, .. } => {
                body.references(namespace, name)
            }
            Declaration::Enum { .. } => false,
        }
    }
}
