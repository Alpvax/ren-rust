macro_rules! create_enum {
    ($enum_name:ident = Context::$ctx_root:ident => <$hir_typ:ty, $val_typ:ty> $(: $span_mapper:expr;)? {
        $(match $ctx_pat:pat => $ctx_fun:expr,)*
        $(Context::$ctx:ident => $ctx_variant:ident($(struct $ctx_struct_name:ident)?$($ctx_typ_name:ty)?),)*
        $(Token::$tok:ident => $tok_variant:ident($(struct $tok_struct_name:ident)?$($tok_typ_name:ty)?),)*
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $enum_name {
            $($ctx_variant($($ctx_struct_name)?$($ctx_typ_name)?),)*
            $($tok_variant($($tok_struct_name)?$($tok_typ_name)?),)*
        }
        $($(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $ctx_struct_name(SyntaxNode);
            impl From<$ctx_struct_name> for $enum_name {
                fn from(v: $ctx_struct_name) -> Self {
                    Self::$ctx_variant(v)
                }
            }

        )?)*
        $($(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $tok_struct_name(SyntaxToken);
            impl From<$tok_struct_name> for $enum_name {
                fn from(v: $tok_struct_name) -> Self {
                    Self::$tok_variant(v)
                }
            }
        )?)*
        impl FromSyntaxElement for $enum_name {
            fn from_token(token_type: Token, token: SyntaxToken) -> Option<Self> {
                match token_type {
                    Token::Whitespace | Token::Comment | Token::ParenOpen | Token::Error => None,
                    $(
                        $(Token::$tok => Some(Self::$tok_variant($tok_struct_name(token))),)?
                        $(Token::$tok => Some(Self::$tok_variant(<$tok_typ_name>::new(token))),)?
                    )*
                    _ => None,
                }
            }
            fn from_node(context: Context, node: SyntaxNode) -> Option<Self> {
                match context {
                    $(
                        $ctx_pat => $ctx_fun(node),
                    )*
                    $(
                        $(Context::$ctx => Some(Self::$ctx_variant($ctx_struct_name(node))),)?
                        $(Context::$ctx => Some(Self::$ctx_variant(<$ctx_typ_name>::new(node))),)?
                    )*
                    Context::$ctx_root | Context::Parenthesised => node.children_with_tokens().skip_trivia().next().and_then(Self::from_element),
                    _ => None
                }
            }
            fn from_root_node(node: SyntaxNode) -> Option<Self> {
                Self::from_node(Context::$ctx_root, node)
            }
            fn get_range(&self) -> ::rowan::TextRange {
                match self {
                    $(
                        $(Self::$ctx_variant($ctx_struct_name(el, ..)) => el.text_range(),)?
                        $(Self::$ctx_variant(typ) => <$ctx_typ_name>::text_range(typ),)?
                    )*
                    $(
                        $(Self::$tok_variant($tok_struct_name(el, ..)) => el.text_range(),)?
                        $(Self::$tok_variant(typ) => <$tok_typ_name>::text_range(typ),)?
                    )*
                }
            }
        }
        impl ToHIR for $enum_name {
            type HIRType = $hir_typ;
            type ValidationError = $val_typ;
            fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
                let res = match self {
                    $(Self::$ctx_variant(val) => val.to_higher_ast(line_lookup).into(),)*
                    $(Self::$tok_variant(val) => val.to_higher_ast(line_lookup).into(),)*
                };
                $(
                    return $span_mapper(res, self.get_range(), line_lookup);
                    #[allow(unreachable_code)]
                )?
                res
            }
            fn validate(&self) -> Option<Self::ValidationError>{
                todo!("{}::validate", stringify!($enum_name))
            }
        }
    };
}
pub(super) use create_enum as create_ast_enum;
