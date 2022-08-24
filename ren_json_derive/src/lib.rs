use proc_macro_error::{emit_call_site_error, proc_macro_error};
use quote::{format_ident, quote};

mod data;
use data::{RenJsonAttribute, RenJsonEnumAttribute, VariantData};

#[proc_macro_error]
#[proc_macro_derive(RenJson, attributes(ren_json))]
pub fn ren_enum_serialise(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::ItemEnum {
        ident: enum_ident,
        generics,
        mut variants,
        attrs,
        ..
    } = syn::parse(tokens).expect("Error parsing stream as variant");
    let mut type_params = Vec::new();
    if let Some(syn::Attribute { tokens, .. }) =
        attrs.into_iter().find(|syn::Attribute { path, .. }| {
            path.segments
                .last()
                .map_or(false, |syn::PathSegment { ident, .. }| {
                    ident.to_string() == "ren_json"
                })
        })
    {
        match syn::parse2::<RenJsonEnumAttribute>(tokens) {
            Ok(att) => type_params = att.types,
            Err(e) => emit_call_site_error!(
                "Error parsing #[ren_json(Type parameters...)] attribute for enum {}:\n\t{}",
                enum_ident,
                e
            ),
        }
    }
    let (v_ser_arm, v_de_arm) = std::mem::take(&mut variants).into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut ser_arms, mut de_arms),
         syn::Variant {
             ident: var_ident,
             attrs,
             fields,
             ..
         }| {
            let mut data = VariantData::new(&enum_ident, &generics, &var_ident, fields);
            if let Some(syn::Attribute { tokens, .. }) =
                attrs.into_iter().find(|syn::Attribute { path, .. }| {
                    path.segments
                        .last()
                        .map_or(false, |syn::PathSegment { ident, .. }| {
                            ident.to_string() == "ren_json"
                        })
                })
            {
                match syn::parse2::<RenJsonAttribute>(tokens) {
                    Ok(att) => data.apply_attribute(att),
                    Err(e) => emit_call_site_error!(
                        "Error parsing #[ren_json(..)] attribute for variant {}:\n\t{}",
                        var_ident,
                        e
                    ),
                }
            }
            match data.split_arms() {
                Ok(res) => {
                    ser_arms.push(res.0);
                    de_arms.push(res.1);
                }
                Err(e) => emit_call_site_error!(
                    "Error generating RenJson code for variant {}:\n\t{}",
                    var_ident,
                    e
                ),
            }
            (ser_arms, de_arms)
        },
    );
    let mut ser_generics = generics.clone();
    if type_params.len() > 0 {
        ser_generics.where_clause = Some(match ser_generics.where_clause {
            Some(mut w_c) => {
                w_c.predicates
                    .extend(type_params.iter().map::<::syn::WherePredicate, _>(
                        |t_p| ::syn::parse_quote! { #t_p: ::serde::Serialize,},
                    ));
                w_c
            }
            None => ::syn::parse_quote! { where #(#type_params: ::serde::Serialize,)* },
        });
    }
    let (impl_generics, ty_generics, ser_where_clause) = ser_generics.split_for_impl();
    let mut tokens = quote! {
        impl #impl_generics ::serde::Serialize for #enum_ident #ty_generics #ser_where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use ::serde::ser::SerializeSeq;
                match self {
                    #(#v_ser_arm)*
                }
            }
        }
    };
    let mut de_generics = generics.clone();
    de_generics.params.push(syn::parse_quote! { 'de });
    if type_params.len() > 0 {
        de_generics.where_clause = Some(match de_generics.where_clause {
            Some(mut w_c) => {
                w_c.predicates
                    .extend(type_params.iter().map::<::syn::WherePredicate, _>(
                        |t_p| ::syn::parse_quote! { #t_p: ::serde::Deserialize<'de>, },
                    ));
                w_c
            }
            None => ::syn::parse_quote! { where #(#type_params: ::serde::Deserialize<'de>,)* },
        });
    }
    let (de_impl_generics, visitor_generics, de_where_clause) = de_generics.split_for_impl();
    tokens.extend(quote! {
        impl #de_impl_generics ::serde::Deserialize<'de> for #enum_ident #ty_generics #de_where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use core::marker::PhantomData;
                struct Visitor #visitor_generics (PhantomData<fn(#(#type_params),*) -> &'de ()>);
                impl #de_impl_generics ::serde::de::Visitor<'de> for Visitor #visitor_generics #de_where_clause {
                // impl ::serde::de::Visitor for Visitor {
                    type Value = #enum_ident #ty_generics;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "a list, with the first item being an object with a property of \"$\" = string <<< and any other meta properties>>>")
                    }
                    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error> where S: ::serde::de::SeqAccess<'de> {
                        use ::serde::de::{Error, Unexpected};
                        let meta_el = seq.next_element::<::serde_json::Map<String, ::serde_json::Value>>()?;
                        if let Some(mut meta) = meta_el {
                            let tag_val = meta.remove("$").ok_or(S::Error::missing_field("$"))?;
                            if let ::serde_json::Value::String(tag) = tag_val {
                                match tag.as_str() {
                                    #(#v_de_arm,)*
                                    _ => Err(S::Error::custom(format!("Unable to deserialize {} variant from unsupported tag: \"{}\"", stringify!(#enum_ident), tag))),
                                }
                            } else {
                                Err(S::Error::custom("tag (\"$\" value) must be a string"))
                            }
                        } else {
                            Err(S::Error::invalid_type(Unexpected::Option, &self))
                        }
                    }
                }
                deserializer.deserialize_seq(Visitor(PhantomData))
            }
        }
    });
    tokens.into()
}

fn as_ident<M>(member: M) -> syn::Ident
where
    M: core::borrow::Borrow<syn::Member>,
{
    match member.borrow() {
        syn::Member::Named(i) => i.clone(),
        syn::Member::Unnamed(i) => format_ident!("field_{}", i),
    }
}
