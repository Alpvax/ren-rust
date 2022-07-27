use higher_ast::expr::Operator;
use smol_str::SmolStr;

use crate::syntax::{Context, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    pattern::Pattern,
    FromSyntaxElement, SyntaxNode, SyntaxToken, ToHIR,
};

type HigherLiteral = higher_ast::core::Literal<higher_ast::expr::Expr>;
type HigherExpr = higher_ast::expr::Expr;

macro_rules! make_expr_enum {
    ($($name:ident($struct_name:ident = $typ:ident) = $p:pat,)+ { match token_type {$($tok_pat:pat => $tok_res:expr,)+} match context {$($node_pat:pat => $node_res:expr,)+}}) => {
        make_expr_enum!{@acc [], [], [] $($name($struct_name = $typ) = $p,)+ {$($tok_pat => $tok_res,)+} {$($node_pat => $node_res,)+}}
    };
    (@acc [$($name:ident($struct_name:ident = $typ:ident)),*], [$($name_n:ident($struct_name_n:ident) = $pattern_n:pat),*], [$($name_t:ident($struct_name_t:ident) = $pattern_t:pat),*] $v:ident($s:ident = SyntaxNode) = $p:pat, $($v_r:ident($sn_r:ident = $typ_r:ident) = $p_r:pat,)* {$($tok_pat:pat => $tok_res:expr,)+} {$($node_pat:pat => $node_res:expr,)+}) => {
        make_expr_enum!{@acc [$($name($struct_name = $typ),)* $v($s = SyntaxNode)], [$($name_n($struct_name_n) = $pattern_n,)* $v($s) = $p], [$($name_t($struct_name_t) = $pattern_t),*] $($v_r($sn_r = $typ_r) = $p_r,)* {$($tok_pat => $tok_res,)+} {$($node_pat => $node_res,)+}}
    };
    (@acc [$($name:ident($struct_name:ident = $typ:ident)),*], [$($name_n:ident($struct_name_n:ident) = $pattern_n:pat),*], [$($name_t:ident($struct_name_t:ident) = $pattern_t:pat),*] $v:ident($s:ident = SyntaxToken) = $p:pat, $($v_r:ident($sn_r:ident = $typ_r:ident) = $p_r:pat,)* {$($tok_pat:pat => $tok_res:expr,)+} {$($node_pat:pat => $node_res:expr,)+}) => {
        make_expr_enum!{@acc [$($name($struct_name = $typ),)* $v($s = SyntaxToken)], [$($name_n($struct_name_n) = $pattern_n),*], [$($name_t($struct_name_t) = $pattern_t,)* $v($s) = $p] $($v_r($sn_r = $typ_r) = $p_r,)* {$($tok_pat => $tok_res,)+} {$($node_pat => $node_res,)+}}
    };
    (@acc [$($name:ident($struct_name:ident = $typ:ident)),*], [$($name_n:ident($struct_name_n:ident) = $pattern_n:pat),*], [$($name_t:ident($struct_name_t:ident) = $pattern_t:pat),*] {$($tok_pat:pat => $tok_res:expr,)+} {$($node_pat:pat => $node_res:expr,)+}) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Expr {$(
            $name ($struct_name),
        )+}
        impl FromSyntaxElement for Expr {
            fn from_token(token_type: Token, token: SyntaxToken) -> Option<Self> {
                match token_type {
                    Token::Whitespace | Token::Comment | Token::ParenOpen | Token::Error => None,
                    $(
                        $pattern_t => Some(Self::$name_t($struct_name_t(token))),
                    )*
                    $(
                        $tok_pat => $tok_res,
                    )+
                }
            }
            fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
                match context {
                    $(
                        $pattern_n => Some(Self::$name_n($struct_name_n(node))),
                    )*
                    $(
                        $node_pat => $node_res(node),
                    )+
                }
            }
            fn from_root_node(node: SyntaxNode) -> Option<Self> {
                Self::from_node(Context::Expr, node)
            }
        }
        impl ToHIR for Expr {
            type HIRType = higher_ast::expr::Expr;
            type ValidationError = ();
            fn to_higher_ast(&self) -> Self::HIRType {
                match self {
                    $(
                        Self::$name(val) => val.to_higher_ast().into(),
                    )*
                }
            }
            fn validate(&self) -> Option<Self::ValidationError>{
                todo!("Expr::validate")
            }
        }
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $struct_name($typ);
            impl From<$struct_name> for Expr {
                fn from(v: $struct_name) -> Self {
                    Self::$name(v)
                }
            }
        )+
    };
}

make_expr_enum! {
    // Literals
    LArray(ArrayExpr = SyntaxNode) = Context::Array,
    // LBool(BoolExpr = SyntaxToken) = Token::Bool,
    LConstructor(ConsExpr = SyntaxNode) = Context::Constructor,
    LNum(NumberExpr = SyntaxToken) = Token::Number,
    LRecord(RecordExpr = SyntaxNode) = Context::Record,
    LString(StringExpr = SyntaxNode) = Context::String,
    // LUnit(UnitExpr = SyntaxToken) = Token::Undefined,

    // Variable
    VScoped(ScopedExpr = SyntaxNode) = Context::Scoped,
    VName(VarExpr = SyntaxToken) = Token::VarName,
    VPlaceholder(PlaceholderExpr = SyntaxToken) = Token::Placeholder,

    // Compound / nested
    EAccess(AccessExpr = SyntaxNode) = Context::Access,
    EBinding(BindingExpr = SyntaxNode) = Context::Declaration,
    EBinOp(BinOpExpr = SyntaxNode) = Context::BinOp,
    ECall(CallExpr = SyntaxNode) = Context::Application,
    EConditional(ConditionalExpr = SyntaxNode) = Context::Conditional,
    ELambda(LambdaExpr = SyntaxNode) = Context::Lambda,
    EPrefixOp(PrefixOpExpr = SyntaxNode) = Context::PrefixOp,
    EWhere(WhereExpr = SyntaxNode) = Context::Where,

    {
        match token_type {
            _ => None,
        }
        match context {
            Context::Expr => |node: SyntaxNode| node.children_with_tokens().skip_trivia().next().and_then(Expr::from_element),
            _ => |_| None,
        }
    }
}

// Implementations =============================================================

impl ArrayExpr {
    pub fn items(&self) -> Vec<Expr> {
        self.0
            .children()
            .filter_map(|node| {
                if node.kind() == Context::Item.into() {
                    Expr::from_node(Context::Expr, node)
                } else {
                    None
                }
            })
            .collect()
    }
}
impl ToHIR for ArrayExpr {
    type HIRType = HigherLiteral;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherLiteral::LArr(
            self.items()
                .into_iter()
                .map(|item| item.to_higher_ast())
                .collect(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl ConsExpr {
    pub fn tag(&self) -> Option<SmolStr> {
        self.0
            .find_token(Token::VarName)
            .map(|t| SmolStr::new(t.text()))
    }
    pub fn args(&self) -> Vec<Expr> {
        self.0
            .find_node(Context::Args)
            .map_or(Vec::default(), |node| {
                node.children_with_tokens()
                    .filter_map(Expr::from_element)
                    .collect()
            })
    }
}
impl ToHIR for ConsExpr {
    type HIRType = HigherLiteral;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        self.tag()
            .map(|tag| {
                HigherLiteral::LCon(
                    tag.to_string(),
                    self.args()
                        .into_iter()
                        .map(|arg| arg.to_higher_ast())
                        .collect(),
                )
            })
            .unwrap()
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl NumberExpr {
    pub fn value(&self) -> f64 {
        self.0
            .text()
            .to_string()
            .parse()
            .expect("Error parsing number from token")
    }
}
impl ToHIR for NumberExpr {
    type HIRType = HigherLiteral;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        HigherLiteral::LNum(self.value())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl RecordExpr {
    pub fn fields(&self) -> Vec<(String, Option<Expr>)> {
        self.0
            .children()
            .filter_map(|field_node| {
                let mut iter = field_node.children_with_tokens().skip_trivia();
                iter.find(|n| n.kind() == Token::VarName.into())
                    .map(|n| n.into_token().unwrap().text().to_string())
                    .and_then(|name| Some((name.clone(), iter.last().and_then(Expr::from_element))))
            })
            .collect()
    }
}
impl ToHIR for RecordExpr {
    type HIRType = HigherLiteral;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        fn map_field((name, val): (String, Option<Expr>)) -> (String, HigherExpr) {
            let val = val
                .map(|v| v.to_higher_ast())
                .unwrap_or(HigherExpr::Var(name.clone()));
            (name, val)
        }
        HigherLiteral::LRec(self.fields().into_iter().map(map_field).collect())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringSegment {
    Text(SmolStr),
    Expr(Expr),
}
impl StringExpr {
    pub fn parts(&self) -> Vec<StringSegment> {
        let (mut parts, text) = self
            .0
            .children_with_tokens()
            .filter_map(|e| {
                use crate::syntax::{StringToken, SyntaxPart};
                match e.kind() {
                    SyntaxPart::StringToken(StringToken::Text)
                    | SyntaxPart::StringToken(StringToken::Escape) => e
                        .into_token()
                        .map(|t| StringSegment::Text(SmolStr::new(t.text()))),
                    SyntaxPart::Context(Context::Expr) => e
                        .into_node()
                        .and_then(|node| Expr::from_node(Context::Expr, node))
                        .map(|expr| StringSegment::Expr(expr)),
                    _ => None,
                }
            })
            .fold((Vec::new(), Vec::new()), |(mut acc, mut text), part| {
                match part {
                    StringSegment::Text(txt) => {
                        text.push(txt);
                    }
                    expr_segment => {
                        if text.len() > 0 {
                            acc.push(StringSegment::Text(SmolStr::new(text.join(""))));
                            text.clear();
                        }
                        acc.push(expr_segment);
                    }
                }
                (acc, text)
            });
        if text.len() > 0 {
            parts.push(StringSegment::Text(SmolStr::new(text.join(""))));
        }
        parts
    }
}
impl ToHIR for StringExpr {
    type HIRType = HigherLiteral; //TODO: HigherExpr;
    type ValidationError = ();

    fn to_higher_ast(&self) -> Self::HIRType {
        //TODO: proper string ast
        HigherLiteral::LStr(
            self.parts()
                .into_iter()
                .find_map(|part| match part {
                    StringSegment::Text(text) => Some(text.to_string()),
                    StringSegment::Expr(_) => None,
                })
                .unwrap_or("NO_TEXT_PART".to_string()),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

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
