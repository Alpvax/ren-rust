use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

trait SimplifiedEnum {}

#[proc_macro_derive(SimplifiedEnum)]
pub fn simple_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_wrapped_macro(&ast)
}

fn make_arm(name: &syn::Ident, s_name: &syn::Ident, f_name: &syn::Ident, field: &syn::Fields) -> syn:: Arm {
    syn::parse_str(&format!("{0}::{2}{3} => {1}::{2},", name, s_name, f_name, match field {
        syn::Fields::Named(_) => "{..}",
        syn::Fields::Unnamed(_) => "(..)",
        syn::Fields::Unit => "",
    })).unwrap()
}

fn impl_wrapped_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let s_name = format_ident!("_Simple_{}", &name);
    let mut gen = TokenStream::new();
    if let syn::Data::Enum(data) = &ast.data {
        let v = data.variants.iter().map(|v| {
            (
                &v.ident,
                make_arm(name, &s_name, &v.ident, &v.fields),
            )
        }).collect::<HashMap<_, _>>();
        let variants = v.keys();
        let arms = v.values();
        gen.extend(TokenStream::from(quote!{
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
        }));
    }
    gen
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
