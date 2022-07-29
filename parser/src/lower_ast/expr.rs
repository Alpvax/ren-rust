use higher_ast::expr::Operator;
use smol_str::SmolStr;

use crate::syntax::{Context, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    literal,
    macro_impl::create_ast_enum,
    pattern::Pattern,
    FromSyntaxElement, SyntaxNode, SyntaxToken, ToHIR,
};

type HigherExpr = higher_ast::expr::Expr;

create_ast_enum! {
    Expr = Context::Expr => <HigherExpr, ()> {
        // Literal contexts
        Context::Array => LArray(literal::LArray<Self>),
        Context::Constructor => LConstructor(literal::LConstructor<Self>),
        Context::Record => LRecord(literal::LRecord<Self>),
        Context::String => LString(literal::LString<Self>),

        // Var/ref context
        Context::Scoped => VScoped(struct ScopedExpr),

        // Other contexts
        Context::Access => EAccess(struct AccessExpr),
        Context::Declaration => EBinding(struct BindingExpr),
        Context::BinOp => EBinOp(struct BinOpExpr),
        Context::Application => ECall(struct CallExpr),
        Context::Conditional => EConditional(struct ConditionalExpr),
        Context::Lambda => ELambda(struct LambdaExpr),
        Context::PrefixOp => EPrefixOp(struct PrefixOpExpr),
        Context::Where => EWhere(struct WhereExpr),

        // Literal tokens
        Token::Number => LNum(literal::LNumber<Self>),

        // Variable tokens
        Token::VarName => VName(struct VarExpr),
        Token::Placeholder => VPlaceholder(struct PlaceholderExpr),
    }
}
impl super::HigherASTWithVar for HigherExpr {
    fn var_value(var: String) -> Self {
        Self::Var(var)
    }
}

// Implementations =============================================================

impl ScopedExpr {
    pub fn namespace(&self) -> Vec<SmolStr> {
        self.0
            .child_tokens()
            .filter(|t| t.kind() == Token::Namespace.into())
            .map(|t| SmolStr::new(t.text()))
            .collect()
    }
    pub fn varname(&self) -> Option<SmolStr> {
        self.0
            .find_token(Token::VarName)
            .map(|t| SmolStr::new(t.text()))
    }
}
impl ToHIR for ScopedExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::Scoped(
            self.namespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            self.varname().unwrap().to_string(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl VarExpr {
    pub fn name(&self) -> SmolStr {
        SmolStr::new(self.0.text())
    }
}
impl ToHIR for VarExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::Var(self.name().to_string())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

// impl PlaceholderExpr {}
impl ToHIR for PlaceholderExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::Placeholder
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl AccessExpr {
    pub fn obj(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .next()
            .and_then(Expr::from_element)
    }
    pub fn key(&self) -> Option<SmolStr> {
        self.0.child_tokens().last().map(|t| SmolStr::new(t.text()))
    }
}
impl ToHIR for AccessExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::access(
            self.obj().to_higher_ast().unwrap(),
            self.key().unwrap().to_string(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl BindingExpr {
    pub fn pattern(&self) -> Option<Pattern> {
        self.0
            .find_node(Context::Pattern)
            .and_then(Pattern::from_root_node)
    }
    pub fn binding_expr(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Expr)
            .and_then(Expr::from_root_node)
    }
    pub fn result_expr(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .last()
            .and_then(Expr::from_element)
    }
}
impl ToHIR for BindingExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::binding(
            self.pattern().to_higher_ast().unwrap(),
            self.binding_expr().to_higher_ast().unwrap(),
            self.result_expr().to_higher_ast().unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl BinOpExpr {
    pub fn op(&self) -> Option<Operator> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .nth(1)
            .and_then(|e| Operator::from_symbol(e.into_token().unwrap().text()))
    }
    pub fn lhs(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .next()
            .and_then(Expr::from_element)
    }
    pub fn rhs(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Expr::from_element)
    }
}
impl ToHIR for BinOpExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::binop(
            self.lhs().to_higher_ast().unwrap(),
            self.op().unwrap(),
            self.rhs().to_higher_ast().unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl CallExpr {
    pub fn func(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .next()
            .and_then(Expr::from_element)
    }
    pub fn arg(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Expr::from_element)
    }
}
impl ToHIR for CallExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        let mut func = self.func().unwrap();
        let mut r_args = vec![self.arg()];
        while let Expr::ECall(call) = func {
            r_args.push(call.arg());
            func = call.func().unwrap();
        }
        r_args.reverse();
        HigherExpr::apply_many(
            func.to_higher_ast(),
            r_args.into_iter().filter_map(|arg| arg.to_higher_ast()),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl ConditionalExpr {
    pub fn condition(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Condition)
            .and_then(Expr::from_root_node)
    }
    pub fn true_expr(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Then)
            .and_then(Expr::from_root_node)
    }
    pub fn false_expr(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Else)
            .and_then(Expr::from_root_node)
    }
}
impl ToHIR for ConditionalExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::conditional(
            self.condition().to_higher_ast().unwrap(),
            self.true_expr().to_higher_ast().unwrap(),
            self.false_expr().to_higher_ast().unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl LambdaExpr {
    pub fn params(&self) -> Vec<Pattern> {
        self.0
            .find_node(Context::Params)
            .map_or(Vec::new(), |node| {
                node.children()
                    .filter_map(|p_node| {
                        if p_node.kind() == Context::Pattern.into() {
                            Pattern::from_root_node(p_node)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
    }
    pub fn body(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Expr::from_element)
    }
}
impl ToHIR for LambdaExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::lambda(
            self.params()
                .into_iter()
                .map(|p| p.to_higher_ast())
                .collect(),
            self.body().to_higher_ast().unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl PrefixOpExpr {
    pub fn op(&self) -> Option<Operator> {
        self.0
            .child_tokens()
            .skip_trivia()
            .next()
            .and_then(|t| Operator::from_symbol(t.text()))
    }
    pub fn operand(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .skip_trivia()
            .last()
            .and_then(Expr::from_element)
    }
}
impl ToHIR for PrefixOpExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::binop(
            HigherExpr::literal(0),
            self.op().unwrap(),
            self.operand().to_higher_ast().unwrap(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl WhereExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0
            .find_node(Context::Expr)
            .and_then(Expr::from_root_node)
    }
    pub fn branches(&self) -> Vec<(Pattern, Option<Expr>, Expr)> {
        use crate::syntax::SyntaxPart;
        self.0
            .children()
            .filter_map(|node| {
                if node.kind() == Context::Branch.into() {
                    let (pat, guard, expr) =
                        node.children()
                            .fold((None, None, None), |(p, g, e), child| match child.kind() {
                                SyntaxPart::Context(Context::Pattern) => {
                                    (Pattern::from_root_node(child), g, e)
                                }
                                SyntaxPart::Context(Context::Guard) => (
                                    p,
                                    child
                                        .children_with_tokens()
                                        .skip_trivia()
                                        .last()
                                        .and_then(Expr::from_element),
                                    e,
                                ),
                                SyntaxPart::Context(Context::Expr) => {
                                    (p, g, Expr::from_root_node(child))
                                }
                                _ => panic!("Unexpected node in branch"),
                            });
                    if pat.is_some() && expr.is_some() {
                        Some((pat.unwrap(), guard, expr.unwrap()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
impl ToHIR for WhereExpr {
    type HIRType = HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherExpr::Switch(
            Box::new(self.expr().to_higher_ast().unwrap()),
            self.branches()
                .into_iter()
                .map(|(p, g, e)| {
                    (
                        p.to_higher_ast(),
                        g.map(|ge| ge.to_higher_ast()),
                        e.to_higher_ast(),
                    )
                })
                .collect(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}
