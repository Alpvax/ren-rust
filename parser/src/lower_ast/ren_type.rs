use smol_str::SmolStr;

use crate::syntax::{Context, SyntaxNode, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension, TokenTypeWrapper},
    macro_impl::create_ast_enum,
    FromSyntaxElement, SyntaxToken, ToHIR,
};

type HigherType = higher_ast::Type;

create_ast_enum! {
    Type = Context::Type => <HigherType, ()> {
        Context::Application => TApp(struct TApp),
        Context::FunType => TFun(struct TFun),
        Context::Record => TRec(struct TRec),
        Context::SumType => TSum(struct TSum),
        Context::Variant => TEnum(struct TEnum),

        Token::Namespace => TCon(struct TCon),
        Token::OpMul => TAny(struct TAny),
        Token::TypeQuestion => THole(struct THole),
        Token::VarName => TVar(struct TVar),
    }
}
impl super::HigherASTWithVar for HigherType {
    fn var_value(var: String) -> Self {
        Self::Var(var)
    }
}

impl ToHIR for TAny {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::Any
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl ToHIR for THole {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::Hole
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TRec {
    pub fn fields(&self) -> impl Iterator<Item = (String, Type)> {
        self.0.children().filter_map(|field_node| {
            let mut iter = field_node.children_with_tokens().skip_trivia();
            iter.find(|n| n.kind() == Token::VarName.into()).map(|n| {
                let name = n.into_token().unwrap().text().to_string();
                let err = format!("missing type for field {}", name);
                (name, iter.last().and_then(Type::from_element).expect(&err))
            })
        })
    }
}
impl ToHIR for TRec {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::rec(
            self.fields()
                .map(|(k, t)| (k, t.to_higher_ast(line_lookup))),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TVar {
    pub fn name(&self) -> SmolStr {
        SmolStr::new(self.0.text())
    }
}
impl ToHIR for TVar {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::Var(self.name().to_string())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TFun {
    pub fn arg(&self) -> Option<Type> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .next()
            .and_then(Type::from_element)
    }
    pub fn ret(&self) -> Option<Type> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Type::from_element)
    }
}
impl ToHIR for TFun {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::fun(
            self.arg().map(|t| t.to_higher_ast(line_lookup)),
            self.ret().map(|t| t.to_higher_ast(line_lookup)).unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TCon {
    pub fn name(&self) -> SmolStr {
        SmolStr::new(self.0.text())
    }
}
impl ToHIR for TCon {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::Con(self.name().to_string())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TEnum {
    fn name(&self) -> Option<SmolStr> {
        self.0
            .child_tokens()
            .find(|t| t.kind_matches(Token::VarName))
            .map(|t| SmolStr::new(t.text()))
    }
}
impl ToHIR for TEnum {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        // HigherType::sum([(self.name().unwrap(), Vec::new())])
        unimplemented!("Should never be calling to_higher_ast on Variant context")
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TApp {
    fn typ(&self) -> Option<Type> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .next()
            .and_then(Type::from_element)
    }
    fn arg(&self) -> Option<Type> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Type::from_element)
    }
}
impl ToHIR for TApp {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        let mut typ = self.typ().unwrap();
        let mut r_args = vec![self.arg()];
        while let Type::TApp(app) = typ {
            r_args.push(app.arg());
            typ = app.typ().unwrap();
        }
        r_args.reverse();
        let args = r_args
            .into_iter()
            .filter_map(|arg| arg.to_higher_ast(line_lookup))
            .collect();
        if let Type::TEnum(var) = typ {
            HigherType::sum([(var.name().unwrap(), args)])
        } else {
            HigherType::App(Box::new(typ.to_higher_ast(line_lookup)), args)
        }
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl TSum {
    pub fn parts(&self) -> impl Iterator<Item = (String, Vec<Type>)> {
        //TODO: iterate (variant_name, [args...])
        // self.0.children().filter_map(|node| if node.kind() == Context::Variant.into() {
        //     node.children_with_tokens().skip_trivia()
        // } else {
        //     None
        // })
        None.into_iter() //XXX
    }
}
impl ToHIR for TSum {
    type HIRType = HigherType;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherType::sum(
            self.parts()
                .map(|(s, v)| (s, v.into_iter().map(|t| t.to_higher_ast(line_lookup)))),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}
