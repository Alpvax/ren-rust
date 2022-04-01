use crate::{expression::Expression, Ident, Type, Visibility};


#[derive(Debug, Clone)]
pub enum BindingContent<T> {
    External,
    Internal(T),
}

#[derive(Debug, Clone)]
pub enum ModuleDeclaration {
    Binding {
        identifier: Ident,
        visibility: Visibility,
        ren_type: Option<Type>,
        content: BindingContent<Expression>,
    },
    Statement(Expression),
    Type {
        identifier: Ident,
        visibility: Visibility,
        content: BindingContent<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum BlockDeclaration {
    Binding {
        identifier: Ident,
        ren_type: Option<Type>,
        body: Expression,
    },
    Statement(Expression),
}

pub trait Declaration {
    fn references(&self, namespace: Option<&Vec<String>>, name: Option<&str>) -> bool {
        if namespace.is_none() && name.is_none() {
            return false;
        }
        self.get_body().map_or(false, |body| body.references(namespace, name))
    }
    fn get_type(&self) -> Option<&Type>;
    fn get_body(&self) -> Option<&Expression>;
}

impl Declaration for ModuleDeclaration {
    fn get_type(&self) -> Option<&Type> {
        match self {
            Self::Binding { ren_type, .. } => ren_type.as_ref(),
            Self::Type { content: BindingContent::Internal(ren_type), .. } => Some(ren_type),
            _ => None,
        }
    }

    fn get_body(&self) -> Option<&Expression> {
        match self {
            Self::Binding { content: BindingContent::Internal(body), .. } | Self::Statement(body) => Some(&body),
            _ => None,
        }
    }
}

impl Declaration for BlockDeclaration {
    fn get_type(&self) -> Option<&Type> {
        match self {
            Self::Binding { ren_type, .. } => ren_type.as_ref(),
            Self::Statement(_) => None,
        }
    }

    fn get_body(&self) -> Option<&Expression> {
        match self {
            Self::Binding { body, .. } => Some(&body),
            Self::Statement(e) => Some(&e),
        }
    }
}

// impl Declaration {
//     pub fn name_as_pattern(&self) -> Pattern {
//         match self {
//             Declaration::Function { name, .. } => Pattern::Name(name.clone()),
//             Declaration::Variable { name, .. } => name.clone(),
//             Declaration::Enum { variants, .. } => Pattern::ArrayDestructure(
//                 variants
//                     .iter()
//                     .map(|(tag, _)| Pattern::Name(tag.clone()))
//                     .collect(),
//             ),
//         }
//     }
//     pub fn body(&self) -> Option<&Expression> {
//         match self {
//             Declaration::Function { body, .. } => Some(body),
//             Declaration::Variable { body, .. } => Some(body),
//             Declaration::Enum { .. } => None,
//         }
//     }
//     pub fn references(&self, namespace: Option<&Vec<String>>, name: Option<&str>) -> bool {
//         if namespace.is_none() && name.is_none() {
//             return false;
//         }
//         match self {
//             Declaration::Function { body, .. } | Declaration::Variable { body, .. } => {
//                 body.references(namespace, name)
//             }
//             Declaration::Enum { .. } => false,
//         }
//     }
// }
