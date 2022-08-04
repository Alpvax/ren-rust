use proc_macro::TokenStream;
use quote::{quote, format_ident};

#[proc_macro_derive(RenJson, attributes(ren_json))]
pub fn ren_enum_serialise(tokens: TokenStream) -> TokenStream {
    let syn::ItemEnum{ident: enum_ident, generics, mut variants, ..} = syn::parse(tokens).expect("Error parsing stream as variant");
    let (v_struct_name, v_tag, v_pat, v_meta, v_expr) = std::mem::take(&mut variants).into_iter().map(|syn::Variant{ident: var_ident, attrs, ..}| {
        if let Some(syn::Attribute{tokens, ..}) = attrs.into_iter().find(|syn::Attribute{path, ..}| path.segments.last().map_or(false, |syn::PathSegment{ident, ..}| ident.to_string() == "ren_json")) {
            syn::parse2::<RenJsonAttribute>(tokens).expect("Error parsing ren_json attribute").data(&enum_ident, var_ident)
        } else {
            panic!("Variant {} is missing the #[ren_json(...)] attribute", var_ident)
        }
    }).fold((Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()), |mut acc, data| {
        acc.0.push(data.struct_name);
        acc.1.push(data.tag);
        acc.2.push(data.pat);
        acc.3.push(data.meta);
        acc.4.push(data.items);
        acc
    });
    quote!{
        impl #generics serde::Serialize for #enum_ident #generics {
            fn serialize<S>(&self, serialiser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#v_pat => {
                        let serialisable = #v_struct_name(#v_tag, #v_meta, #v_expr);
                        serialisable.serialize(serialiser)
                    })*
                }
            }
        }
        #(
            struct #v_struct_name(&'static str, Meta, )
        )*
    }.into()
}

struct RenJsonData {
    name: syn::Ident,
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
    fn data(self, enum_ident: &syn::Ident, variant_ident: syn::Ident) -> RenJsonData {
        let tag = self.tag.unwrap_or(variant_ident.to_string());
        let struct_name = format_ident!("RenJsonSerialised{}{}", enum_ident, variant_ident);
        let pat = match self.matcher.pat {
            syn::Pat::Tuple(syn::PatTuple{attrs, elems, paren_token}) => syn::Pat::TupleStruct(syn::PatTupleStruct { attrs, path: syn::parse_str(variant_ident.to_string().as_str()).expect(&format!("Error creating path from ident: {}", variant_ident)), pat: syn::PatTuple{attrs: Vec::new(), elems, paren_token} }),
            pat @ syn::Pat::TupleStruct(_) | pat @ syn::Pat::Struct(_) => pat,
            _ => panic!("Invalid pattern type"),
        };
        if let syn::Expr::Tuple(syn::ExprTuple{elems, ..}) = *self.matcher.body {
            let mut iter = elems.into_iter();
            let meta = iter.next().unwrap();
            if let syn::Expr::Tuple(items) = iter.last().unwrap() {
                RenJsonData {
                    name: variant_ident,
                    struct_name,
                    tag,
                    pat,
                    meta,
                    items,
                }
            } else {
                todo!("items of ren_json for variant {} is not a tuple:", variant_ident)
            }
        } else {
            panic!("body of ren_json for variant {} is not a tuple", variant_ident)
        }
    }
}
impl syn::parse::Parse for RenJsonAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::parenthesized!(content in input);
        panic!("first item is ident: {:?}", content);//XXX
        let tag = if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
            match input.parse::<syn::Ident>()?.to_string().as_str() {
                "tag" | "name" => {
                    input.parse::<syn::Ident>()?;
                    input.parse::<syn::Token![=]>()?;
                    let tag = input.parse::<syn::LitStr>()?;
                    input.parse::<syn::Token![,]>()?;
                    Some(tag.value())
                }
                _ => None,
            }
        } else {
            None
        };
        let matcher = input.parse::<syn::Arm>()?;
        Ok(Self { tag, matcher })
    }
}
