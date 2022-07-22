use crate::syntax::{Context, Token};

use super::{
    extensions::SyntaxNodeExtension, FromSyntaxElement, SyntaxNode, SyntaxToken, skip_trivia,
};

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
            fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
                match context {
                    $(
                        $pattern_n => Some(Self::$name_n($struct_name_n(node))),
                    )*
                    $(
                        $node_pat => $node_res,
                    )+
                }
            }
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
    Access(AccessExpr = SyntaxNode) = Context::Access,
    Binding(BindingExpr = SyntaxNode) = Context::Declaration,
    BinOp(BinOpExpr = SyntaxNode) = Context::BinOp,
    Call(CallExpr = SyntaxNode) = Context::Application,
    Conditional(ConditionalExpr = SyntaxNode) = Context::Conditional,
    Lambda(LambdaExpr = SyntaxNode) = Context::Lambda,
    PrefixOp(PrefixOpExpr = SyntaxNode) = Context::PrefixOp,
    Where(WhereExpr = SyntaxNode) = Context::Where,

    {
        match token_type {
            _ => None,
        }
        match context {
            _ => None,
        }
    }
}

// Implementations =============================================================

impl ArrayExpr {
    fn items(&self) -> Vec<Expr> {
        self.0.map_children().collect()
    }
}

// impl BoolExpr {
//     fn value(&self) -> bool {
//         self.0.text().to_string().parse().expect("Error parsing boolean from token")
//     }
// }

impl ConsExpr {

}

impl NumberExpr {
    fn value(&self) -> bool {
        self.0.text().to_string().parse().expect("Error parsing boolean from token")
    }
}
impl RecordExpr {
    fn fields(&self) -> Vec<(String, Option<Expr>)> {
        self.0.children().filter_map(|field_node| {
            let mut iter = field_node.children_with_tokens().filter(skip_trivia);
            iter.find(|n| n.kind() == Token::VarName.into()).map(|n| n.into_token().unwrap().text().to_string())
                .and_then(|name| {
                    Some((name.clone(), iter.last().and_then(Expr::from_element)))
                })
        }).collect()
    }
}
impl StringExpr {

}

impl ScopedExpr {

}
impl VarExpr {

}
impl PlaceholderExpr {

}

impl AccessExpr {

}
impl BindingExpr {

}
impl BinOpExpr {

}
impl CallExpr {

}
impl ConditionalExpr {

}
impl LambdaExpr {

}
impl PrefixOpExpr {

}
impl WhereExpr {

}
