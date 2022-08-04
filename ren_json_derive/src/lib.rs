use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{emit_call_site_error, proc_macro_error};
use quote::{format_ident, quote};

#[proc_macro_error]
#[proc_macro_derive(RenJson, attributes(ren_json))]
pub fn ren_enum_serialise(tokens: TokenStream) -> TokenStream {
    let syn::ItemEnum {
        ident: enum_ident,
        generics,
        mut variants,
        ..
    } = syn::parse(tokens).expect("Error parsing stream as variant");
    let (v_struct_name, v_tag, v_pat, v_meta, v_expr) = std::mem::take(&mut variants)
        .into_iter()
        .filter_map(
            |syn::Variant {
                 ident: var_ident,
                 attrs,
                 ..
             }| {
                if let Some(syn::Attribute { tokens, .. }) =
                    attrs.into_iter().find(|syn::Attribute { path, .. }| {
                        path.segments
                            .last()
                            .map_or(false, |syn::PathSegment { ident, .. }| {
                                ident.to_string() == "ren_json"
                            })
                    })
                {
                    syn::parse2::<RenJsonAttribute>(tokens)
                        .expect("Error parsing ren_json attribute")
                        .data(&enum_ident, var_ident)
                } else {
                    emit_call_site_error!(
                        "Variant {} is missing the #[ren_json(...)] attribute",
                        var_ident
                    );
                    None
                }
            },
        )
        .fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, data| {
                acc.0.push(data.struct_name);
                acc.1.push(data.tag);
                acc.2.push(data.pat);
                acc.3.push(data.meta);
                acc.4.push(data.items);
                acc
            },
        );
    quote!{
        impl #generics serde::Serialize for #enum_ident #generics {
            fn serialize<S>(&self, serialiser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#v_pat => {
                        println!("Making struct {} with items: {:?}", stringify!(#v_struct_name), #v_expr);
                        let serialisable = #v_struct_name(#v_tag, #v_meta, ());//TODO: #v_expr);
                        serialisable.serialize(serialiser)
                    })*
                }
            }
        }
        #(
            struct #v_struct_name<'a>(&'static str, &'a Meta, (/*TODO: item types*/));
            impl<'a> serde::Serialize for #v_struct_name<'a> {
                fn serialize<S>(&self, serialiser: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use ::serde::ser::SerializeSeq;
                    use ::serde_json::{json, Value, Map};
                    let mut map = if let Value::Object(m) = json!(*self.1) {
                        m
                    } else {
                        Map::new()
                    };
                    map.insert("$".to_string(), self.0.into());
                    let mut seq = serialiser.serialize_seq(Some(2))?;
                    seq.serialize_element(&map)?;
                    seq.serialize_element(&self.2)?;
                    seq.end()
                }
            }
        )*
    }.into()
}

struct RenJsonData {
    // name: syn::Ident,
    struct_name: syn::Ident,
    tag: String,
    pat: syn::Pat,
    meta: syn::Expr,
    items: syn::ExprTuple,
}
struct RenJsonAttribute {
    tag: Option<String>,
    matcher: syn::Arm,
}
impl RenJsonAttribute {
    fn data(self, enum_ident: &syn::Ident, variant_ident: syn::Ident) -> Option<RenJsonData> {
        let tag = self.tag.unwrap_or(variant_ident.to_string());
        let struct_name = format_ident!("RenJsonSerialised{}{}", enum_ident, variant_ident);
        let pat = match self.matcher.pat {
            syn::Pat::Tuple(syn::PatTuple {
                attrs,
                elems,
                paren_token,
            }) => syn::Pat::TupleStruct(syn::PatTupleStruct {
                attrs,
                path: syn::parse_str(&format!("Self::{}", variant_ident.to_string())).expect(
                    &format!("Error creating path from ident: {}", variant_ident),
                ),
                pat: syn::PatTuple {
                    attrs: Vec::new(),
                    elems,
                    paren_token,
                },
            }),
            pat @ syn::Pat::TupleStruct(_) | pat @ syn::Pat::Struct(_) => pat,
            _ => {
                emit_call_site_error!("Invalid pattern type");
                return None;
            }
        };
        match *self.matcher.body {
            syn::Expr::Tuple(syn::ExprTuple { elems, .. }) => {
                let mut iter = elems.into_iter();
                let meta = match iter.next().unwrap() {
                    syn::Expr::Tuple(syn::ExprTuple { elems, .. }) if elems.len() < 1 => {
                        syn::parse_str("None").unwrap()
                    }
                    expr => expr,
                };
                let items = match iter.last().unwrap() {
                    syn::Expr::Tuple(items) => items,
                    syn::Expr::Array(syn::ExprArray {
                        attrs,
                        elems,
                        bracket_token,
                    }) => syn::ExprTuple {
                        attrs,
                        elems,
                        paren_token: syn::token::Paren(bracket_token.span),
                    },
                    expr @ syn::Expr::Path(..) => {
                        let mut elems = syn::punctuated::Punctuated::new();
                        elems.push(expr);
                        syn::ExprTuple {
                            attrs: Vec::new(),
                            paren_token: syn::token::Paren(Span::call_site()),
                            elems,
                        }
                    }
                    _ => {
                        emit_call_site_error!(
                            "items of ren_json for variant {} is not a tuple or single value:",
                            variant_ident
                        );
                        return None;
                    }
                };
                Some(RenJsonData {
                    // name: variant_ident,
                    struct_name,
                    tag,
                    pat,
                    meta,
                    items,
                })
            }
            _ => {
                emit_call_site_error!(
                    "body of ren_json for variant {} is not a tuple",
                    variant_ident
                );
                None
            }
        }
    }
}
impl syn::parse::Parse for RenJsonAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let tag = if content.peek(syn::Ident) && content.peek2(syn::Token![=]) {
            match content.parse::<syn::Ident>()?.to_string().as_str() {
                "tag" | "name" => {
                    content.parse::<syn::Token![=]>()?;
                    let tag = content.parse::<syn::LitStr>()?;
                    content.parse::<syn::Token![,]>()?;
                    Some(tag.value())
                }
                _ => None,
            }
        } else {
            None
        };
        let matcher = content.parse::<syn::Arm>()?;
        Ok(Self { tag, matcher })
    }
}
