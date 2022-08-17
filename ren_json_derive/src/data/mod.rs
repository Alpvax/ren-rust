use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

mod attribute;
pub(crate) use attribute::RenJsonAttribute;
use attribute::RenJsonItems;

use super::as_ident;

fn struct_name(enum_name: &syn::Ident, variant_name: &syn::Ident) -> syn::Ident {
    format_ident!("RenJsonSerialised{}{}", enum_name, variant_name)
}

fn debug_member(member: &syn::Member) -> String {
    match member {
        syn::Member::Named(name) => name.to_string(),
        syn::Member::Unnamed(idx) => idx.index.to_string(),
    }
}

fn filter_generics<'a, T>(enum_generics: &syn::Generics, types: T) -> syn::Generics
where
    T: IntoIterator<Item = &'a syn::Type>,
{
    fn references(param: &syn::GenericParam, typ: &syn::Type) -> bool {
        let recurse = |ty: &syn::Type| references(&param, &ty);
        match typ {
            syn::Type::Array(syn::TypeArray { elem, .. })
            | syn::Type::Group(syn::TypeGroup { elem, .. })
            | syn::Type::Paren(syn::TypeParen { elem, .. })
            | syn::Type::Ptr(syn::TypePtr { elem, .. })
            | syn::Type::Slice(syn::TypeSlice { elem, .. }) => recurse(&*elem),
            syn::Type::BareFn(syn::TypeBareFn {
                inputs,
                lifetimes,
                output,
                ..
            }) => {
                inputs.iter().any(|arg| recurse(&arg.ty))
                    || if let syn::GenericParam::Lifetime(ld) = &param {
                        lifetimes
                            .iter()
                            .flat_map(|l| l.lifetimes.iter())
                            .any(|l| l.lifetime == ld.lifetime)
                    } else if let syn::ReturnType::Type(_, ty) = output {
                        recurse(&*ty)
                    } else {
                        false
                    }
            }
            // syn::Type::ImplTrait(syn::TypeImplTrait{bounds, ..}) => {
            //     bounds.iter().any(|bound| match &bound {
            //         syn::TypeParamBound::Trait(_) => todo!(),
            //         syn::TypeParamBound::Lifetime(l) => ref_lifetimes.contains(l),
            //     })
            // },
            syn::Type::Infer(_) | syn::Type::Never(_) => false,
            // syn::Type::Macro(_) => todo!(),
            syn::Type::Path(syn::TypePath { path, .. }) => {
                if path.leading_colon.is_none() && path.segments.len() == 1 {
                    if let (
                        Some(syn::PathSegment { ident: s_ident, .. }),
                        syn::GenericParam::Type(syn::TypeParam { ident: p_ident, .. }),
                    ) = (&path.segments.last(), &param)
                    {
                        if p_ident == s_ident {
                            return true;
                        }
                    }
                }
                path.segments.iter().any(|seg| {
                    match &seg.arguments {
                        syn::PathArguments::None => false,
                        syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments { args, .. },
                        ) => args.iter().any(|arg| match (&arg, &param) {
                            (
                                syn::GenericArgument::Lifetime(a_l),
                                syn::GenericParam::Lifetime(p_l),
                            ) => a_l == &p_l.lifetime,
                            (syn::GenericArgument::Type(t), _) => recurse(&t),
                            // (syn::GenericArgument::Binding(_), syn::GenericParam::Type(_)) => todo!(),
                            // (syn::GenericArgument::Binding(_), syn::GenericParam::Lifetime(_)) => todo!(),
                            // (syn::GenericArgument::Binding(_), syn::GenericParam::Const(_)) => todo!(),
                            // (syn::GenericArgument::Const(_), syn::GenericParam::Const(syn::ConstParam{ident, ..})) => todo!(),
                            _ => false,
                        }),
                        syn::PathArguments::Parenthesized(syn::ParenthesizedGenericArguments {
                            inputs,
                            output,
                            ..
                        }) => {
                            inputs.iter().any(|ty| recurse(&ty))
                                || if let syn::ReturnType::Type(_, ty) = output {
                                    recurse(&*ty)
                                } else {
                                    false
                                }
                        }
                    }
                })
            }
            syn::Type::Reference(syn::TypeReference { elem, lifetime, .. }) => {
                if let syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime: p_l, .. }) = &param
                {
                    lifetime.as_ref().filter(|l| l == &p_l).is_some()
                } else {
                    recurse(&*elem)
                }
            }
            // syn::Type::TraitObject(_) => todo!(),
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => elems.iter().any(recurse),
            // syn::Type::Verbatim(_) => todo!(),
            _ => false,
        }
    }
    let types = types.into_iter().collect::<Vec<_>>();
    let mut generics = enum_generics.clone();
    generics.params = generics
        .params
        .into_iter()
        .filter(|param| types.iter().any(|typ| references(param, typ)))
        .collect();
    // TODO: if let Some(ref mut where_clause) = generics.where_clause {
    //     where_clause.predicates = where_clause.predicates.into_iter().filter(|pre| {
    //         match pre {
    //             syn::WherePredicate::Type(syn::PredicateType{bounded_ty, lifetimes, bounds, ..}) => todo!(),
    //             syn::WherePredicate::Lifetime(syn::PredicateLifetime{lifetime, bounds, ..}) => todo!(),
    //             syn::WherePredicate::Eq(_) => unimplemented!(),
    //         }
    //     }).collect();
    // }
    generics
}

// struct VariantGenerics<'a> {
//     params: Vec<syn::GenericParam>,
//     where_clause: &'a Option<syn::WhereClause>,
// }
// impl<'a> VariantGenerics<'a> {
//     fn new<'f>(enum_generics: &'a syn::Generics, variant_fields: &'f syn::Fields) -> Self {
//         fn references(param: &syn::GenericParam, typ: &syn::Type) -> bool {
//             let recurse = |ty: &syn::Type| references(&param, &ty);
//             match typ {
//                 syn::Type::Array(syn::TypeArray { elem, .. })
//                 | syn::Type::Group(syn::TypeGroup { elem, .. })
//                 | syn::Type::Paren(syn::TypeParen { elem, .. })
//                 | syn::Type::Ptr(syn::TypePtr { elem, .. })
//                 | syn::Type::Slice(syn::TypeSlice { elem, .. }) => recurse(&*elem),
//                 syn::Type::BareFn(syn::TypeBareFn {
//                     inputs,
//                     lifetimes,
//                     output,
//                     ..
//                 }) => {
//                     inputs.iter().any(|arg| recurse(&arg.ty))
//                         || if let syn::GenericParam::Lifetime(ld) = &param {
//                             lifetimes
//                                 .iter()
//                                 .flat_map(|l| l.lifetimes.iter())
//                                 .any(|l| l.lifetime == ld.lifetime)
//                         } else if let syn::ReturnType::Type(_, ty) = output {
//                             recurse(&*ty)
//                         } else {
//                             false
//                         }
//                 }
//                 // syn::Type::ImplTrait(syn::TypeImplTrait{bounds, ..}) => {
//                 //     bounds.iter().any(|bound| match &bound {
//                 //         syn::TypeParamBound::Trait(_) => todo!(),
//                 //         syn::TypeParamBound::Lifetime(l) => ref_lifetimes.contains(l),
//                 //     })
//                 // },
//                 syn::Type::Infer(_) | syn::Type::Never(_) => false,
//                 // syn::Type::Macro(_) => todo!(),
//                 syn::Type::Path(syn::TypePath { path, .. }) => {
//                     if path.leading_colon.is_none() && path.segments.len() == 1 {
//                         if let (
//                             Some(syn::PathSegment { ident: s_ident, .. }),
//                             syn::GenericParam::Type(syn::TypeParam { ident: p_ident, .. }),
//                         ) = (&path.segments.last(), &param)
//                         {
//                             if p_ident == s_ident {
//                                 return true;
//                             }
//                         }
//                     }
//                     path.segments.iter().any(|seg| {
//                         match &seg.arguments {
//                             syn::PathArguments::None => false,
//                             syn::PathArguments::AngleBracketed(
//                                 syn::AngleBracketedGenericArguments { args, .. },
//                             ) => args.iter().any(|arg| match (&arg, &param) {
//                                 (
//                                     syn::GenericArgument::Lifetime(a_l),
//                                     syn::GenericParam::Lifetime(p_l),
//                                 ) => a_l == &p_l.lifetime,
//                                 (syn::GenericArgument::Type(t), _) => recurse(&t),
//                                 // (syn::GenericArgument::Binding(_), syn::GenericParam::Type(_)) => todo!(),
//                                 // (syn::GenericArgument::Binding(_), syn::GenericParam::Lifetime(_)) => todo!(),
//                                 // (syn::GenericArgument::Binding(_), syn::GenericParam::Const(_)) => todo!(),
//                                 // (syn::GenericArgument::Const(_), syn::GenericParam::Const(syn::ConstParam{ident, ..})) => todo!(),
//                                 _ => false,
//                             }),
//                             syn::PathArguments::Parenthesized(
//                                 syn::ParenthesizedGenericArguments { inputs, output, .. },
//                             ) => {
//                                 inputs.iter().any(|ty| recurse(&ty))
//                                     || if let syn::ReturnType::Type(_, ty) = output {
//                                         recurse(&*ty)
//                                     } else {
//                                         false
//                                     }
//                             }
//                         }
//                     })
//                 }
//                 syn::Type::Reference(syn::TypeReference { elem, lifetime, .. }) => {
//                     if let syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime: p_l, .. }) =
//                         &param
//                     {
//                         lifetime.as_ref().filter(|l| l == &p_l).is_some()
//                     } else {
//                         recurse(&*elem)
//                     }
//                 }
//                 // syn::Type::TraitObject(_) => todo!(),
//                 syn::Type::Tuple(syn::TypeTuple { elems, .. }) => elems.iter().any(recurse),
//                 // syn::Type::Verbatim(_) => todo!(),
//                 _ => false,
//             }
//         }
//         let params = enum_generics
//             .params
//             .iter()
//             .filter(|param| variant_fields.iter().any(|f| references(&param, &f.ty)))
//             .cloned()
//             .collect();
//         Self {
//             params,
//             where_clause: &enum_generics.where_clause,
//         }
//     }
//     fn generics(&self, insert_lifetime: bool) -> TokenStream {
//         let param = &self.params;
//         match (insert_lifetime, param.len()) {
//             (false, 0) => TokenStream::new(),
//             (true, 0) => quote! { <'ren_serde> },
//             (false, _) => quote! { <#(#param),*> },
//             (true, _) => quote! { <'ren_serde, #(#param),*> },
//         }
//     }
// }

// struct VariantMeta {
//     fields: Vec<syn::Ident>,
//     selected: Option<syn::Ident>,
// }
// impl VariantMeta {
//     fn new(variant_fields: &syn::Fields) -> Self {
//         Self {
//             fields: variant_fields
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(i, f)| match &f.ty {
//                     syn::Type::Path(syn::TypePath { path, .. })
//                         if path
//                             .segments
//                             .last()
//                             .filter(|seg| seg.ident.to_string() == Self::meta_ident())
//                             .is_some() =>
//                     {
//                         Some(match &f.ident {
//                             Some(i) => i.clone(),
//                             _ => format_ident!("field_{}", i),
//                         })
//                     }
//                     _ => None,
//                 })
//                 .collect(),
//             selected: None,
//         }
//     }
//     fn meta_ident() -> &'static str {
//         "Meta"
//     }
//     fn set_meta_member(&mut self, member: syn::Member) {
//         self.set_meta(as_ident(member));
//     }
//     fn set_meta(&mut self, ident: syn::Ident) {
//         self.selected = Some(ident);
//     }
//     fn exists(&self) -> bool {
//         self.selected.is_some() || self.fields.len() == 0
//     }
//     fn typ(&self) -> TokenStream {
//         let ty = Self::meta_ident();
//         if self.exists() {
//             quote! { &'ren_serde #ty }
//         } else {
//             TokenStream::new()
//         }
//     }
//     fn ident(&self) -> Option<&syn::Ident> {
//         if self.selected.is_some() {
//             self.selected.as_ref()
//         } else if self.fields.len() == 1 {
//             self.fields.last()
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldsKind {
    None,
    Named,
    Unnamed,
}

#[derive(Clone)]
pub(crate) struct VariantArm<'a> {
    enum_name: &'a syn::Ident,
    variant_name: syn::Ident,
    meta: Option<syn::Ident>,
    items: RenJsonItems<syn::Ident>,
    pattern: syn::Pat,
}
impl<'a> VariantArm<'a> {
    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    fn has_items(&self) -> bool {
        match self.items {
            RenJsonItems::None => false,
            _ => true,
        }
    }
    fn has_fields(&self) -> bool {
        self.has_meta() || self.has_items()
    }
    fn from_fields(
        enum_name: &'a syn::Ident,
        variant_name: &syn::Ident,
        fields: &VariantFields,
    ) -> syn::Result<Self> {
        if let FieldsKind::None = fields.kind {
            Ok(Self {
                enum_name,
                variant_name: variant_name.clone(),
                meta: None,
                items: RenJsonItems::None,
                pattern: syn::parse_quote! { Self::#variant_name },
            })
        } else {
            let (meta, items, fields_vec) = fields.iter().fold(
                (None, Vec::new(), Vec::new()),
                |(mut meta, mut items, mut all), (f, t)| {
                    all.push(f);
                    if let syn::Type::Path(syn::TypePath { path, .. }) = t {
                        if path
                            .segments
                            .last()
                            .filter(|syn::PathSegment { ident, .. }| ident.to_string() == "Meta")
                            .is_some()
                            && meta.is_none()
                        {
                            meta = Some(f.clone());
                            return (meta, items, all);
                        }
                    }
                    items.push(f);
                    (meta, items, all)
                },
            );
            let pattern: syn::Pat = if let FieldsKind::Named = fields.kind {
                syn::parse_quote! {
                    Self::#variant_name{ #(#fields_vec),* }
                }
            } else {
                syn::parse_quote! {
                    Self::#variant_name(#(#fields_vec),*)
                }
            };
            Ok(Self {
                enum_name,
                variant_name: variant_name.clone(),
                meta,
                items: items.into(),
                pattern,
            })
        }
    }
    fn with_meta(mut self, meta: Option<&syn::Member>) -> VariantArm<'a> {
        self.meta = meta.map(as_ident);
        self
    }
}
impl<'a> core::fmt::Debug for VariantArm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantArm")
            .field("enum_name", &self.enum_name)
            .field("variant_name", &self.variant_name)
            .field("meta", &self.meta)
            .field("items", &self.items)
            .field("pattern", &format!("{}", self.pattern.to_token_stream()))
            .finish()
    }
}
impl<'a> core::fmt::Display for VariantArm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}
impl<'a> ToTokens for VariantArm<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pattern.to_tokens(tokens);
        <syn::Token![=>]>::default().to_tokens(tokens);
        struct_name(self.enum_name, &self.variant_name).to_tokens(tokens);
        if self.has_fields() {
            syn::token::Paren::default().surround(tokens, |tokens| {
                if let Some(meta) = &self.meta {
                    meta.to_tokens(tokens);
                    if self.has_items() {
                        <syn::Token![,]>::default().to_tokens(tokens);
                    }
                }
                match &self.items {
                    RenJsonItems::None => (),
                    RenJsonItems::SingleValue(v) | RenJsonItems::SingleList(v) => {
                        v.to_tokens(tokens)
                    }
                    RenJsonItems::Multiple(item) => tokens.extend(quote! { (#(#item),*) }),
                }
            });
        }
        tokens.extend(quote! { .serialize(serializer), });
        // <syn::Token![,]>::default().to_tokens(tokens);
    }
}

#[derive(Clone)]
pub(crate) struct VariantStruct<'a> {
    enum_name: &'a syn::Ident,
    variant_name: syn::Ident,
    generics: syn::Generics,
    tag: Option<String>,
    meta: Option<syn::Type>,
    items: RenJsonItems<syn::Type>,
    #[allow(dead_code)]
    ref_types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]>,
}
impl<'a> VariantStruct<'a> {
    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    fn has_items(&self) -> bool {
        match self.items {
            RenJsonItems::None => false,
            _ => true,
        }
    }
    fn has_fields(&self) -> bool {
        self.has_meta() || self.has_items()
    }
    fn from_fields(
        enum_name: &'a syn::Ident,
        variant_name: &syn::Ident,
        enum_generics: &syn::Generics,
        fields: &VariantFields,
    ) -> syn::Result<Self> {
        if let FieldsKind::None = fields.kind {
            Ok(Self {
                enum_name,
                variant_name: variant_name.clone(),
                tag: None,
                meta: None,
                generics: syn::Generics::default(),
                items: RenJsonItems::None,
                ref_types: Default::default(),
            })
        } else {
            let (meta, items, ref_types) = fields.iter().fold(
                (None, Vec::new(), syn::punctuated::Punctuated::new()),
                |(mut meta, mut items, mut all), (_, t)| {
                    let ref_type: syn::Type = syn::parse_quote!(&'ren_serde #t);
                    all.push(ref_type);
                    if let syn::Type::Path(syn::TypePath { path, .. }) = t {
                        if path
                            .segments
                            .last()
                            .filter(|syn::PathSegment { ident, .. }| ident.to_string() == "Meta")
                            .is_some()
                            && meta.is_none()
                        {
                            meta = Some(t.clone());
                            return (meta, items, all);
                        }
                    }
                    items.push(t);
                    (meta, items, all)
                },
            );
            let mut generics = filter_generics(enum_generics, ref_types.iter());
            if ref_types.len() > 0 {
                generics.params.push(syn::parse_quote! { 'ren_serde });
            }
            Ok(Self {
                enum_name,
                variant_name: variant_name.clone(),
                generics,
                tag: None,
                meta,
                items: items.into(),
                ref_types,
            })
        }
    }

    fn with_data(mut self, tag: Option<String>, meta: Option<syn::Type>) -> VariantStruct<'a> {
        self.tag = tag;
        self.meta = meta;
        self
    }
}
impl<'a> core::fmt::Display for VariantStruct<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}
impl<'a> ToTokens for VariantStruct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = struct_name(self.enum_name, &self.variant_name);
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        syn::token::Struct::default().to_tokens(tokens);
        struct_name.to_tokens(tokens);
        if self.has_fields() {
            impl_generics.to_tokens(tokens);
            syn::token::Paren::default().surround(tokens, |tokens| {
                if let Some(meta) = &self.meta {
                    tokens.extend(quote! { &'ren_serde #meta });
                    if self.has_items() {
                        <syn::Token![,]>::default().to_tokens(tokens);
                    }
                }
                match &self.items {
                    RenJsonItems::None => (),
                    RenJsonItems::SingleValue(v) | RenJsonItems::SingleList(v) => {
                        tokens.extend(quote! { &'ren_serde #v })
                    }
                    RenJsonItems::Multiple(item) => {
                        tokens.extend(quote! { (#(&'ren_serde #item),*) })
                    }
                }
            });
            where_clause.to_tokens(tokens);
        }
        <syn::Token![;]>::default().to_tokens(tokens);
        // Serde
        let (map_expr, index_stmnt): (syn::Expr, syn::Expr) = if self.meta.is_some() {
            (
                syn::parse_quote! {
                    if let ::serde_json::Value::Object(m) = ::serde_json::json!(self.0) {
                        m
                    } else {
                        ::serde_json::Map::new()
                    }
                },
                syn::parse_quote! { self.1 },
            )
        } else {
            (
                syn::parse_quote! { ::serde_json::Map::new() },
                syn::parse_quote! { self.0 },
            )
        };
        let tag = match &self.tag {
            Some(tag) => tag.to_token_stream(),
            None => self.variant_name.to_string().to_token_stream(),
        };
        let mut ser_impl: syn::ItemImpl = syn::parse_quote! {
            #[automatically_derived]
            impl #impl_generics ::serde::Serialize for #struct_name #ty_generics #where_clause {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    use ::serde::ser::SerializeSeq;
                    let mut map = #map_expr;
                    map.insert("$".to_string(), #tag.into());
                    let mut seq = serializer.serialize_seq(Some(2))?;
                    seq.serialize_element(&map)?;
                }
            }
        };
        ser_impl.items = ser_impl
            .items
            .into_iter()
            .map(|item| {
                if let syn::ImplItem::Method(mut method) = item {
                    match self.items {
                        RenJsonItems::None => (),
                        RenJsonItems::SingleList(_) => method
                            .block
                            .stmts
                            .push(syn::parse_quote! { seq.serialize_element([#index_stmnt])?; }),
                        _ => method
                            .block
                            .stmts
                            .push(syn::parse_quote! { seq.serialize_element(#index_stmnt)?; }),
                    }
                    method
                        .block
                        .stmts
                        .push(syn::Stmt::Expr(syn::parse_quote! { seq.end() }));
                    syn::ImplItem::Method(method)
                } else {
                    unreachable!()
                }
            })
            .collect();
        ser_impl.to_tokens(tokens);
    }
}

struct VariantFields {
    kind: FieldsKind,
    fields: IndexMap<syn::Ident, syn::Type>,
}
impl VariantFields {
    fn new(fields: &syn::Fields) -> Self {
        let (fields, kind) = match fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => (
                named
                    .into_iter()
                    // .map(|f| (f.ident.as_ref().unwrap().clone(), unbox_type(&f.ty)))
                    .map(|f| {
                        (
                            f.ident
                                .as_ref()
                                .expect("unwrapping named field ident")
                                .clone(),
                            f.ty.clone(),
                        )
                    })
                    .collect(),
                FieldsKind::Named,
            ),
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => (
                unnamed
                    .into_iter()
                    .enumerate()
                    // .map(|(i, f)| (format_ident!("field_{}", i), unbox_type(&f.ty)))
                    .map(|(i, f)| (format_ident!("field_{}", i), f.ty.clone()))
                    .collect(),
                FieldsKind::Unnamed,
            ),
            syn::Fields::Unit => (IndexMap::new(), FieldsKind::None),
        };
        Self { fields, kind }
    }
    fn len(&self) -> usize {
        self.fields.len()
    }
    fn iter(&self) -> impl Iterator<Item = (&syn::Ident, &syn::Type)> {
        self.fields.iter()
    }
    // fn iter_types(&self) -> impl Iterator<Item = &syn::Type> {
    //     self.fields.values()
    // }
    // fn iter_names(&self) -> impl Iterator<Item = &syn::Ident> {
    //     self.fields.keys()
    // }
    fn contains(&self, name: &syn::Ident) -> bool {
        self.fields.contains_key(name)
    }
    // fn filtered<F>(&self, mut f: F) -> Self
    // where
    //     F: FnMut((&syn::Ident, &syn::Type)) -> bool,
    // {
    //     Self {
    //         kind: self.kind,
    //         fields: self
    //             .iter()
    //             .filter_map(|(i, t)| {
    //                 if f((i, t)) {
    //                     Some((i.clone(), t.clone()))
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect(),
    //     }
    // }
    fn filtered_items(&self, items: &RenJsonItems<syn::Ident>) -> Self {
        let fields = match items {
            RenJsonItems::None => IndexMap::new(),
            RenJsonItems::SingleValue(v) | RenJsonItems::SingleList(v) => {
                let mut map = IndexMap::new();
                if let Some(val) = self.fields.get(v) {
                    map.insert(v.clone(), val.clone());
                }
                map
            }
            RenJsonItems::Multiple(vec) => self
                .fields
                .iter()
                .filter_map(|(i, t)| {
                    if vec.iter().any(|id| i == id) {
                        Some((i.clone(), t.clone()))
                    } else {
                        None
                    }
                })
                .collect(),
        };
        Self {
            kind: self.kind,
            fields,
        }
    }
    // fn contains_member(&self, member: &syn::Member) -> bool {
    //     match member {
    //         syn::Member::Named(name) => self.contains(name),
    //         syn::Member::Unnamed(idx) => self.contains(&format_ident!("field_{}", idx)),
    //     }
    // }
}

pub(crate) struct VariantData<'a> {
    enum_name: &'a syn::Ident,
    name: syn::Ident,
    generics: &'a syn::Generics,
    fields: VariantFields,
    filtered_fields: Option<VariantFields>,
    tag: Option<String>,
    meta: Option<(syn::Member, syn::Type)>,
    items: RenJsonItems<syn::Ident>,
    arm: Option<VariantArm<'a>>,
    def: Option<VariantStruct<'a>>,
}
impl<'a> VariantData<'a> {
    pub fn new(
        enum_name: &'a syn::Ident,
        generics: &'a syn::Generics,
        name: &syn::Ident,
        fields: syn::Fields,
    ) -> Self {
        Self {
            enum_name,
            name: name.clone(),
            generics,
            fields: VariantFields::new(&fields),
            filtered_fields: None,
            tag: None,
            meta: None,
            items: RenJsonItems::None,
            arm: None,
            def: None,
        }
    }
    pub fn apply_attribute(&mut self, attr: RenJsonAttribute) {
        if attr.tag.is_some() {
            self.tag = attr.tag;
        }
        if let Some(meta) = attr.meta {
            self.meta = match &meta {
                syn::Member::Named(i) => self.fields.fields.get(i),
                syn::Member::Unnamed(idx) => self
                    .fields
                    .fields
                    .get_index(idx.index.try_into().expect("field index > usize"))
                    .map(|(_, ty)| ty),
            }
            .map(|typ| (meta, typ.clone()));
        }
        if let Some(pat) = attr.pattern {
            self.filtered_fields = Some(self.fields.filtered_items(&pat.items()));
            self.arm = Some(pat.with_variant_data(self.enum_name, self.name.clone(), &self.fields));
        }
        if let Some(items) = attr.items {
            self.items = items
                .into_iter()
                .map(as_ident)
                // .filter(|ident| self.fields.contains_key(ident))
                .collect();
            self.filtered_fields = Some(self.fields.filtered_items(&self.items));
        }
    }
    pub fn split_arm_def(self) -> syn::Result<(VariantArm<'a>, VariantStruct<'a>)> {
        Ok((
            match self.arm {
                Some(arm) => arm,
                None => VariantArm::from_fields(
                    self.enum_name,
                    &self.name,
                    self.filtered_fields.as_ref().unwrap_or(&self.fields),
                )?
                .with_meta(self.meta.as_ref().map(|(i, _)| i)),
            },
            match self.def {
                Some(def) => def,
                None => VariantStruct::from_fields(
                    self.enum_name,
                    &self.name,
                    self.generics,
                    self.filtered_fields.as_ref().unwrap_or(&self.fields),
                )?
                .with_data(self.tag, self.meta.map(|(_, t)| t)),
            },
        ))
    }
}
impl core::fmt::Debug for VariantData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantData")
            .field("enum_name", &self.enum_name)
            .field("name", &self.name)
            // .field("generics", &self.generics)
            // .field("fields", &self.fields)
            .field("tag", &self.tag)
            .field(
                "meta",
                &self.meta.as_ref().map(|(mem, _typ)| debug_member(mem)),
            )
            .field("items", &self.items)
            .field("arm", &self.arm)
            // .field("def", &self.def)
            .finish()
    }
}
