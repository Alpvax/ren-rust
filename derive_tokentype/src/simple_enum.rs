use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

fn make_arm(name: &syn::Ident, s_name: &syn::Ident, f_name: &syn::Ident, field: &syn::Fields) -> syn:: Arm {
    syn::parse_str(&format!("{0}::{2}{3} => {1}::{2},", name, s_name, f_name, match field {
        syn::Fields::Named(_) => "{..}",
        syn::Fields::Unnamed(_) => "(..)",
        syn::Fields::Unit => "",
    })).unwrap()
}

pub fn impl_wrapped_macro(item: &syn::ItemEnum) -> TokenStream {
    let name = &item.ident;
    let s_name = format_ident!("_Simple_{}", &name);
    let v = item.variants.iter().map(|v| {
        (
            &v.ident,
            make_arm(name, &s_name, &v.ident, &v.fields),
        )
    }).collect::<HashMap<_, _>>();
    let variants = v.keys();
    let arms = v.values();
    TokenStream::from(quote!{
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        enum #s_name {
            #(#variants),*
        }
        impl From<&#name> for #s_name {
            fn from(e: &#name) -> Self {
                match e {
                    #(#arms)*
                }
            }
        }
        impl From<#name> for #s_name {
            fn from(e: #name) -> Self {
                e.simple_type()
            }
        }
        impl SimplifiedEnum for #name {
            type Simple = #s_name;
            fn simple_type(&self) -> Self::Simple {
                #s_name::from(self)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::*;
    trait SimplifiedEnum {
        type Simple;
        fn simple_type(&self) -> Self::Simple;
    }

    #[derive(Debug, SimplifiedEnum)]
    enum T {
        A { x: u8 },
        B(char),
        C,
    }
    #[test]
    fn from_non_simple() {
        assert_eq!(_Simple_T::A, <T as SimplifiedEnum>::Simple::from(T::A { x: 3 }));
        assert_eq!(_Simple_T::B,
            <T as SimplifiedEnum>::Simple::from(T::B('c')));
        assert_eq!(_Simple_T::C,
            <T as SimplifiedEnum>::Simple::from(T::C)
        );
    }
}
