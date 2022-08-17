use proc_macro_error::{emit_call_site_error, proc_macro_error};
use quote::{format_ident, quote};

mod data;
use data::{RenJsonAttribute, VariantData};

#[proc_macro_error]
#[proc_macro_derive(RenJson, attributes(ren_json, meta))]
pub fn ren_enum_serialise(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::ItemEnum {
        ident: enum_ident,
        generics,
        mut variants,
        ..
    } = syn::parse(tokens).expect("Error parsing stream as variant");
    let (v_ser_arm, _v_de_arm) = std::mem::take(&mut variants).into_iter().fold(
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics ::serde::Serialize for #enum_ident #ty_generics #where_clause {
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
        // #(
        //     #v_def
        // )*
    }
    .into()
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

// struct RenJsonData {
//     // name: syn::Ident,
//     struct_name: syn::Ident,
//     tag: String,
//     pat: syn::Pat,
//     meta: Option<syn::Member>,
//     items: Vec<syn::Member>,
//     item_types: Vec<syn::Type>,
// }
// impl RenJsonData {
//     fn new(
//         enum_ident: &syn::Ident,
//         variant_ident: syn::Ident,
//         tag: Option<String>,
//         meta: Option<syn::Member>,
//         fields: HashMap<syn::Member, syn::Type>,
//         fields_type: FieldsType,
//     ) -> Self {
//         let pat = syn::parse_str(&format!(
//             "value @ Self::{}{}",
//             variant_ident.to_string(),
//             fields_type.all_args(),
//         ))
//         .expect(&format!(
//             "Error creating path from ident: {}",
//             variant_ident
//         ));
//         let (items, item_types) = if let Some(meta_mem) = &meta {
//             fields
//                 .into_iter()
//                 .filter(|field| &field.0 != meta_mem)
//                 .unzip()
//         } else {
//             fields.into_iter().unzip()
//         };
//         Self {
//             struct_name: format_ident!("RenJsonSerialised{}{}", enum_ident, variant_ident),
//             tag: tag.unwrap_or_else(|| variant_ident.to_string()),
//             pat,
//             meta,
//             items,
//             item_types,
//         }
//     }
//     fn constructor(&self) -> TokenStream {
//         let RenJsonData {
//             struct_name,
//             tag,
//             items,
//             ..
//         } = self;
//         let item_access =
//             array_or_tuple(&items.iter().map(|item| quote! { value.#item }).collect());
//         match &self.meta {
//             Some(meta) => quote! {
//                 #struct_name(#tag, value.#meta, #item_access)
//             },
//             None => quote! {
//                 #struct_name(#tag, #item_access)
//             },
//         }
//     }
//     fn struct_def(&self) -> TokenStream {
//         let RenJsonData {
//             struct_name,
//             item_types,
//             ..
//         } = self;
//         let (lifetime, item_types) = if item_types.len() > 0 {
//             (
//                 quote! {<'a>},
//                 array_or_tuple(
//                     &item_types
//                         .into_iter()
//                         .map(|typ| quote! { &'a #typ })
//                         .collect(),
//                 ),
//             )
//         } else {
//             (quote! {}, TokenStream::new())
//         };
//         match self.meta {
//             Some(_) => quote! {
//                 struct #struct_name<'a>(&'static str, &'a Meta, #item_types);
//                 impl #struct_name<'_> {
//                     pub fn tagged(&self) -> serde_json::Map<String, serde_json::Value> {
//                         let mut map = if let serde_json::Value::Object(m) = serde_json::json!(*self.1) {
//                             m
//                         } else {
//                             serde_json::Map::new()
//                         };
//                         map.insert("$".to_string(), self.0.into());
//                         map
//                     }
//                 }
//             },
//             None => quote! {
//                 struct #struct_name #lifetime (&'static str, #item_types);
//                 impl #lifetime #struct_name #lifetime {
//                     pub fn tagged(&self) -> serde_json::Map<String, serde_json::Value> {
//                         let mut map = serde_json::Map::new();
//                         map.insert("$".to_string(), self.0.into());
//                         map
//                     }
//                 }
//             },
//         }
//     }
// }
