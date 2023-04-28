#![allow(dead_code)] //XXX
use std::{borrow::Cow, collections::HashMap};

use ast::{Expr, Type};

pub(super) struct Environment {
    scopes: Vec<Scope<'static>>,
}
impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::global()],
        }
    }
}
impl Environment {
    fn scope_mut(&mut self) -> &mut Scope<'static> {
        self.scopes.last_mut().unwrap()
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }
    /// Returns false if attempting to pop the global scope
    pub fn pop_scope(&mut self) -> bool {
        self.scopes.len() > 1 && self.scopes.pop().is_some()
    }
    pub fn push_declaration(&mut self, decl: ast::Decl) {
        match decl {
            ast::Decl::Let { var, typ, expr, .. } => {
                if !self.has_variable(&var) {
                    self.scope_mut().vars.insert(var.into(), (typ, Some(expr)));
                }
            }
            ast::Decl::Ext { var, typ, .. } => {
                if !self.has_variable(&var) {
                    self.scope_mut().vars.insert(var.into(), (typ, None));
                }
            }
            ast::Decl::Type { name, typ, .. } => {
                if !self.has_type(&name) {
                    self.scope_mut().types.insert(name.into(), typ);
                }
            }
        };
    }
    pub fn has_type(&self, type_name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .find(|Scope { types, .. }| types.contains_key(type_name))
            .is_some()
    }
    pub fn has_variable(&self, var_name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .find(|Scope { vars, .. }| vars.contains_key(var_name))
            .is_some()
    }
    pub fn has_import(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .find(|Scope { imports, .. }| imports.contains_key(name))
            .is_some()
    }
    pub fn type_declarations<'a>(&'a self) -> HashMap<&'a str, &'a Type> {
        self.scopes
            .iter()
            .fold(HashMap::new(), |mut map, Scope { types, .. }| {
                map.extend(types.iter().map(|(n, t)| (n.as_ref(), t)));
                map
            })
    }
    pub fn variables<'a>(&'a self) -> HashMap<&'a str, (&'a Type, Option<&'a Expr>)> {
        self.scopes
            .iter()
            .fold(HashMap::new(), |mut map, Scope { vars, .. }| {
                map.extend(vars.iter().map(|(n, (t, e))| (n.as_ref(), (t, e.as_ref()))));
                map
            })
    }
    pub fn imports<'a>(&'a self) -> HashMap<&'a str, &'a str> {
        self.scopes
            .iter()
            .fold(HashMap::new(), |mut map, Scope { imports, .. }| {
                map.extend(imports.iter().map(|(n, i)| (n.as_ref(), i.as_ref())));
                map
            })
    }
    pub fn verify(&self) -> Vec<String> {
        //TODO: check subtypes
        // let mut errors = Vec::new();
        // let t_names = self.type_declarations().into_iter().flat_map(|(name, typ)| {
        //     typ.
        // })
        // self.variables().into_iter().flat_map(|(name, (typ, expr))| {
        // })
        // errors
        Vec::new()
    }
}

#[derive(Debug, Default)]
struct Scope<'a> {
    types: HashMap<Cow<'a, str>, Type>,
    vars: HashMap<Cow<'a, str>, (Type, Option<Expr>)>,
    imports: HashMap<Cow<'a, str>, Cow<'a, str>>,
}
impl<'a> Scope<'a> {
    fn global() -> Self {
        let mut types = HashMap::new();
        types.insert("String".into(), Type::string());
        types.insert("Number".into(), Type::num());
        Self {
            types,
            vars: HashMap::new(),
            imports: HashMap::new(),
        }
    }
}
