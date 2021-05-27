use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, parse::Parser};

pub fn impl_wrapped_macro(item: &syn::ItemEnum) -> TokenStream {
    let name = &item.ident;
    let s_name = format_ident!("Spanned{}", &name);
    let (variants, arms): (Vec<_>, Vec<_>) = item.variants.iter().map(|v| {
        (
            make_variant(&v),
            make_arm(name, &s_name, &v.ident, &v.fields),
        )
    }).unzip();
    TokenStream::from(quote!{
        enum #s_name {
            #(#variants),*
        }
        impl From<&(#name, logos::Span)> for #s_name {
            fn from(e: &(#name, logos::Span)) -> Self {
                match e {
                    #(#arms)*
                }
            }
        }
    })
}

fn make_variant(variant: &syn::Variant) -> syn::Variant {
    let mut v = variant.clone();
    v.fields = match &variant.fields {
        syn::Fields::Named(f) => {
            let mut p = f.named.clone();
            p.extend(vec![syn::Field::parse_named.parse_str("token_span: logos::Span").unwrap()]);
            syn::Fields::Named(syn::FieldsNamed { brace_token: f.brace_token, named: p })
        }
        syn::Fields::Unnamed(f) => {
            let mut p = f.unnamed.clone();
            p.extend(vec![syn::Field::parse_unnamed.parse_str("logos::Span").unwrap()]);
            syn::Fields::Unnamed(syn::FieldsUnnamed { paren_token: f.paren_token, unnamed: p })}
        syn::Fields::Unit => syn::Fields::Unnamed(syn::parse_str::<syn::FieldsUnnamed>("(logos::Span)").unwrap())
    };
    v
}

fn make_arm(name: &syn::Ident, s_name: &syn::Ident, f_name: &syn::Ident, field: &syn::Fields) -> syn:: Arm {
    syn::parse_str(&format!("{0}::{2}{3} => {1}::{2},", name, s_name, f_name, match field {
        syn::Fields::Named(_) => "{..}",
        syn::Fields::Unnamed(_) => "(..)",
        syn::Fields::Unit => "",
    })).unwrap()
}
