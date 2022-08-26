use smol_str::SmolStr;

use crate::syntax::{Context, SyntaxNode, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    literal,
    macro_impl::create_ast_enum,
    FromSyntaxElement, SyntaxToken, ToHIR,
};

type HigherPattern = higher_ast::Pattern;

create_ast_enum! {
    Pattern = Context::Pattern => <HigherPattern, ()> {
        Context::String => PStr(literal::LString<Self>),
        Context::Record => PRec(literal::LRecord<Self>),
        Context::Array => PArr(literal::LArray<Self>),
        Context::Constructor => PCon(literal::LConstructor<Self>),
        Context::TypeMatch => PTyp(struct PType),
        Token::Number => PNum(literal::LNumber<Self>),
        Token::VarName => PVar(struct PVar),
        Token::Placeholder => PAny(struct PAny),
    }
}
impl super::HigherASTWithVar for HigherPattern {
    fn var_value(var: String) -> Self {
        Self::Var(var)
    }
}

impl ToHIR for PAny {
    type HIRType = HigherPattern;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherPattern::Any
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl PType {
    pub fn type_name(&self) -> Option<SmolStr> {
        self.0.child_tokens().skip_trivia().find_map(|token| {
            if token.kind() == Token::Namespace.into() {
                Some(SmolStr::new(token.text()))
            } else {
                None
            }
        })
    }
    pub fn binding(&self) -> Option<Pattern> {
        self.0.children().last().and_then(Pattern::from_root_node)
    }
}
impl ToHIR for PType {
    type HIRType = HigherPattern;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherPattern::Type(
            self.type_name().unwrap().to_string(),
            Box::new(self.binding().to_higher_ast().unwrap()),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl PVar {
    pub fn name(&self) -> SmolStr {
        SmolStr::new(self.0.text())
    }
}

impl ToHIR for PVar {
    type HIRType = HigherPattern;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherPattern::Var(self.name().to_string())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}
