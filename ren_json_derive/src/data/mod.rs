use std::collections::HashMap;

use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

mod attribute;
use attribute::RenJsonItems;
pub(crate) use attribute::{RenJsonAttribute, RenJsonEnumAttribute};

use super::as_ident;

fn debug_member(member: &syn::Member) -> String {
    match member {
        syn::Member::Named(name) => name.to_string(),
        syn::Member::Unnamed(idx) => idx.index.to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldsKind {
    None,
    Named,
    Unnamed,
}

#[derive(Clone)]
pub(crate) struct VariantArmSer {
    variant_name: syn::Ident,
    tag: Option<String>,
    meta: Option<syn::Ident>,
    items: RenJsonItems<syn::Ident>,
    pattern: syn::Pat,
}
impl VariantArmSer {
    // fn has_meta(&self) -> bool {
    //     self.meta.is_some()
    // }
    fn has_items(&self) -> bool {
        match self.items {
            RenJsonItems::None => false,
            _ => true,
        }
    }
    // fn has_fields(&self) -> bool {
    //     self.has_meta() || self.has_items()
    // }
    fn from_field_defaults(variant_name: &syn::Ident, fields: &FieldDefaults) -> syn::Result<Self> {
        if let FieldsKind::None = fields.kind {
            Ok(Self {
                variant_name: variant_name.clone(),
                tag: None,
                meta: None,
                items: RenJsonItems::None,
                pattern: syn::parse_quote! { Self::#variant_name },
            })
        } else {
            // let (meta, items, fields_vec) = fields.iter().fold(
            //     (None, Vec::new(), Vec::new()),
            //     |(mut meta, mut items, mut all), (f, t)| {
            //         all.push(f);
            //         if let syn::Type::Path(syn::TypePath { path, .. }) = t {
            //             if path
            //                 .segments
            //                 .last()
            //                 .filter(|syn::PathSegment { ident, .. }| ident.to_string() == "Meta")
            //                 .is_some()
            //                 && meta.is_none()
            //             {
            //                 meta = Some(f.clone());
            //                 return (meta, items, all);
            //             }
            //         }
            //         items.push(f);
            //         (meta, items, all)
            //     },
            // );
            // let pattern: syn::Pat = if let FieldsKind::Named = fields.kind {
            //     syn::parse_quote! {
            //         Self::#variant_name{ #(#fields_vec),* }
            //     }
            // } else {
            //     syn::parse_quote! {
            //         Self::#variant_name(#(#fields_vec),*)
            //     }
            // };
            Ok(Self {
                variant_name: variant_name.clone(),
                tag: None,
                meta: fields.meta_ident(),
                items: fields.item_idents(),
                pattern: fields.self_pattern(variant_name),
            })
        }
    }
    fn with_data(mut self, tag: Option<&String>, meta: Option<&syn::Member>) -> VariantArmSer {
        self.tag = tag.map(|s| s.to_string());
        self.meta = meta.map(as_ident).or(self.meta);
        self
    }
}
impl core::fmt::Debug for VariantArmSer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantArm")
            .field("variant_name", &self.variant_name)
            .field("tag", &self.tag)
            .field("meta", &self.meta)
            .field("items", &self.items)
            .field("pattern", &format!("{}", self.pattern.to_token_stream()))
            .finish()
    }
}
impl core::fmt::Display for VariantArmSer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}
impl ToTokens for VariantArmSer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pattern.to_tokens(tokens);
        <syn::Token![=>]>::default().to_tokens(tokens);
        syn::token::Brace::default().surround(tokens, |tokens| {
            let tag = match &self.tag {
                Some(tag) => tag.to_token_stream(),
                None => self.variant_name.to_string().to_token_stream(),
            };
            tokens.extend(quote! {
                let mut map = ::serde_json::Map::new();
                map.insert("$".to_string(), #tag.into());
            });
            if let Some(meta) = &self.meta {
                tokens.extend(quote! {
                    if let ::serde_json::Value::Object(m) = ::serde_json::json!(#meta) {
                        map.extend(m);
                    }
                });
            }
            let count = if self.has_items() { 2usize } else { 1 };
            tokens.extend(quote! {
                let mut seq = serializer.serialize_seq(Some(#count))?;
                seq.serialize_element(&map)?;
            });
            if self.has_items() {
                let items = &self.items;
                tokens.extend(quote! {
                    seq.serialize_element(&#items)?;
                });
            }
            tokens.extend(quote! { seq.end() });
        });
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct VariantArmDe<'a> {
    enum_ident: &'a syn::Ident,
    variant_name: syn::Ident,
    tag: Option<String>,
    pattern: Option<syn::Pat>,
    meta: bool,
    body: VariantConstructor,
}
#[allow(dead_code, unused_variables)]
impl<'a> VariantArmDe<'a> {
    fn new(
        enum_ident: &'a syn::Ident,
        tag: Option<String>,
        variant_name: &syn::Ident,
        ctor: VariantConstructor,
        fields: &FieldDefaults,
    ) -> syn::Result<Self> {
        // if let VariantConstructor::None = &fields {
        Ok(Self {
            enum_ident,
            variant_name: variant_name.clone(),
            tag,
            pattern: fields.items().de_pattern(),
            meta: fields.meta.is_some(),
            body: ctor,
        })
        // } else {
        //     let (meta, items, fields_vec) = fields.iter().fold(
        //         (None, Vec::new(), Vec::new()),
        //         |(mut meta, mut items, mut all), (f, t)| {
        //             all.push(f);
        //             if let syn::Type::Path(syn::TypePath { path, .. }) = t {
        //                 if path
        //                     .segments
        //                     .last()
        //                     .filter(|syn::PathSegment { ident, .. }| ident.to_string() == "Meta")
        //                     .is_some()
        //                     && meta.is_none()
        //                 {
        //                     meta = Some(syn::Member::Named(f.clone()));
        //                     return (meta, items, all);
        //                 }
        //             }
        //             items.push(f);
        //             (meta, items, all)
        //         },
        //     );
        //     let pattern: syn::Pat = if let FieldsKind::Named = fields.kind {
        //         syn::parse_quote! {
        //             Self::#variant_name{ #(#fields_vec),* }
        //         }
        //     } else {
        //         syn::parse_quote! {
        //             Self::#variant_name(#(#fields_vec),*)
        //         }
        //     };
        //     Ok(Self {
        //         variant_name: variant_name.clone(),
        //         kind: fields.kind,
        //         tag: None,
        //         meta,
        //         items: items.into_iter().cloned().map(syn::Member::Named).collect(),
        //         pattern: Some(pattern),
        //     })
        // }
    }
    // TODO: fn with_data(mut self, tag: Option<String>, meta: Option<syn::Type>) -> VariantArmDe {
    //     self.tag = tag;
    //     self.meta = meta;
    //     self
    // }
}
impl core::fmt::Display for VariantArmDe<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}
#[allow(unused_variables)]
impl ToTokens for VariantArmDe<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.tag {
            Some(tag) => tag.to_tokens(tokens),
            None => self.variant_name.to_string().to_tokens(tokens),
        };
        <syn::Token![=>]>::default().to_tokens(tokens);
        let name: syn::Path = {
            let e_name = self.enum_ident;
            let name = &self.variant_name;
            syn::parse_quote! { #e_name::#name }
        };
        let name_str = format!("{}::{}", self.enum_ident, self.variant_name);
        if let VariantConstructor::None = &self.body {
            tokens.extend(quote! { Ok(#name) });
        } else {
            syn::token::Brace::default().surround(tokens, |tokens| {
                if let Some(pat) = self.pattern.as_ref() {
                    tokens.extend(quote! {
                        let #pat = seq.next_element()?.ok_or(
                            S::Error::custom(format!("Missing value(s) when deserialising {}", #name_str)),
                        )?;
                    });
                }
                if self.meta {
                    tokens.extend(quote! {
                        let meta = ::serde_json::from_value(::serde_json::Value::Object(meta))
                            .map_err(|e| S::Error::custom(format!("Error parsing meta as object when deserialising {}: {}", #name_str, e)))?;
                        });
                }
                let body = &self.body;
                tokens.extend(quote! { Ok(#name #body) });
                // let (pat, body) = match self.kind {
                //     FieldsKind::None => unreachable!("Already handled"),
                //     FieldsKind::Named => todo!("Named fields"),//syn::token::Brace::default().surround(tokens, args_func),
                //     FieldsKind::Unnamed => {
                //          else {
                //             self.body.pattern()
                //         }
                //         fn to_index(member: &syn::Member) -> u32 {
                //             match member {
                //                 syn::Member::Named(i) => (&i.to_string()[6..]).parse().expect("parsing field index from ident"),
                //                 syn::Member::Unnamed(i) => i.index,
                //             }
                //         }
                //         let mut expr: syn::ExprCall = syn::parse_quote!{ Self::#name() };
                //         let mut args = if self.has_items() {
                //             self.items.iter().map(to_index).collect()
                //         } else {
                //             Vec::new()
                //         };
                //         if let Some(meta) = &self.meta {
                //             let i = to_index(meta);

                //         }
                //         expr
                //     },
                // };
                // tokens.extend(quote!{
                //     if let Some(#pat) = seq.next_element()? {
                //         #body
                //     } else {
                //         Err(S::Error::custom("Missing / malformed value(s)"))
                //     }
                // });
            });
        }
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
    fn defaults(&self) -> FieldDefaults {
        let (meta, fields) = self
            .iter()
            .fold((None, Vec::new()), |(mut meta, mut all), (f, t)| {
                all.push(f.clone().into());
                if let syn::Type::Path(syn::TypePath { path, .. }) = t {
                    if path
                        .segments
                        .last()
                        .filter(|syn::PathSegment { ident, .. }| ident.to_string() == "Meta")
                        .is_some()
                        && meta.is_none()
                    {
                        meta = Some(f.clone());
                        return (meta, all);
                    }
                }
                (meta, all)
            });
        FieldDefaults {
            kind: self.kind,
            meta: meta.map(|i| i.into()),
            fields,
        }
    }
}

struct FieldDefaults {
    kind: FieldsKind,
    meta: Option<syn::Member>,
    fields: Vec<syn::Member>,
}
impl FieldDefaults {
    fn self_pattern(&self, variant_name: &syn::Ident) -> syn::Pat {
        let field = &self.all_idents();
        match self.kind {
            FieldsKind::None => syn::parse_quote! { Self::#variant_name},
            FieldsKind::Named => syn::parse_quote! { Self::#variant_name {#(#field),*} },
            FieldsKind::Unnamed => syn::parse_quote! { Self::#variant_name (#(#field),*) },
        }
    }
    fn all_idents(&self) -> Vec<syn::Ident> {
        self.fields
            .iter()
            .map(|mem| match mem {
                syn::Member::Named(i) => i.clone(),
                syn::Member::Unnamed(i) => format_ident!("field_{}", i),
            })
            .collect()
    }
    fn meta_ident(&self) -> Option<syn::Ident> {
        self.meta.as_ref().map(|mem| match mem {
            syn::Member::Named(i) => i.clone(),
            syn::Member::Unnamed(i) => format_ident!("field_{}", i),
        })
    }
    fn items(&self) -> RenJsonItems<syn::Member> {
        if let Some(meta) = &self.meta {
            self.fields.iter().filter(|i| i != &meta).cloned().collect()
        } else {
            self.fields.iter().cloned().collect()
        }
    }
    fn item_idents(&self) -> RenJsonItems<syn::Ident> {
        if let Some(meta) = &self.meta {
            self.fields
                .iter()
                .filter(|i| i != &meta)
                .map(as_ident)
                .collect()
        } else {
            self.fields.iter().map(as_ident).collect()
        }
    }
}

impl From<&FieldDefaults> for VariantConstructor {
    fn from(fields: &FieldDefaults) -> Self {
        let mut ctor = match fields.kind {
            FieldsKind::None => Self::None,
            FieldsKind::Named => Self::Named(
                fields
                    .all_idents()
                    .iter()
                    .map(|name| (name.clone(), None))
                    .collect(),
            ),
            FieldsKind::Unnamed => Self::Unnamed(
                fields.fields.len(),
                fields
                    .fields
                    .iter()
                    .map(|mem| match mem {
                        syn::Member::Named(name) => (
                            (&name.to_string()[6..])
                                .parse()
                                .expect("Error converting field_{i} name into u32"),
                            name.clone(),
                        ),
                        syn::Member::Unnamed(i) => (i.index, format_ident!("field_{}", i)),
                    })
                    .collect(),
            ),
        };
        if let Some(meta) = &fields.meta {
            ctor.set_meta(meta);
        }
        ctor
    }
}

#[derive(Debug, Clone)]
enum VariantConstructor {
    None,
    Named(HashMap<syn::Ident, Option<syn::Ident>>),
    Unnamed(usize, HashMap<u32, syn::Ident>),
}
impl VariantConstructor {
    fn new(fields: &syn::Fields) -> Self {
        match fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => Self::Named(
                named
                    .iter()
                    .map(|syn::Field { ident, .. }| (ident.as_ref().unwrap().clone(), None))
                    .collect(),
            ),
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                Self::Unnamed(unnamed.len(), HashMap::new())
            }
            syn::Fields::Unit => Self::None,
        }
    }
    fn set_meta_name(&mut self, meta: syn::Ident) {
        let ident = format_ident!("meta");
        match self {
            Self::None => (),
            Self::Named(ref mut map) => {
                map.insert(meta, Some(ident));
            }
            Self::Unnamed(_, ref mut map) => {
                map.insert(
                    (&meta.to_string()[6..])
                        .parse()
                        .expect("Error converting field_{i} name into u32"),
                    ident,
                );
            }
        }
    }
    fn set_meta_index(&mut self, meta: &syn::Index) {
        match self {
            Self::None => (),
            Self::Named(_) => {
                panic!("cannot set meta index of named fields"); //map.insert(format_ident!("field_{}", meta.index))
            }
            Self::Unnamed(_, ref mut map) => {
                map.insert(meta.index, format_ident!("meta"));
            }
        }
    }
    fn set_meta(&mut self, meta: &syn::Member) {
        match meta {
            syn::Member::Named(m) => self.set_meta_name(m.clone()),
            syn::Member::Unnamed(m) => self.set_meta_index(m),
        }
    }
    // fn items_pattern(&self) -> syn::Pat {
    //     match self {
    //         VariantConstructor::None => panic!("Should not call 'items_pattern' on VariantConstructor::None"),
    //         VariantConstructor::Named(names) => {
    //             let fields: Vec<_> = names.values().filter_map(|opt| opt.as_ref()).collect();
    //             syn::parse_quote!{ {#(#names),*} }
    //         },
    //         VariantConstructor::Unnamed(len, names) => todo!(),
    //     }
    // }
    fn with_items(&self, items: &RenJsonItems<syn::Member>) -> Self {
        match (self, items) {
            (Self::None, _) => Self::None,
            (_, RenJsonItems::None) => self.clone(),
            // (Self::Named(map), RenJsonItems::SingleValue(v)) | (Self::Named(map), RenJsonItems::SingleList(v)) => {
            //     let mut map = map.clone();
            //     map.insert(v, Some(v))
            // },
            (Self::Named(_), _ /*map), RenJsonItems::Multiple(v)*/) => todo!(),
            (Self::Unnamed(len, map), RenJsonItems::SingleValue(v))
            | (Self::Unnamed(len, map), RenJsonItems::SingleList(v)) => {
                let mut map = map.clone();
                match v {
                    syn::Member::Named(i) => map.insert(
                        i.to_string()[6..]
                            .parse()
                            .expect("parsing field index from ident"),
                        i.clone(),
                    ),
                    syn::Member::Unnamed(i) => {
                        map.insert(i.index, format_ident!("field_{}", i.index))
                    }
                };
                Self::Unnamed(*len, map)
            }
            (Self::Unnamed(len, map), RenJsonItems::Multiple(items)) => {
                let mut map = map.clone();
                for member in items {
                    match member {
                        syn::Member::Named(i) => map.insert(
                            i.to_string()[6..]
                                .parse()
                                .expect("parsing field index from ident"),
                            i.clone(),
                        ),
                        syn::Member::Unnamed(i) => {
                            map.insert(i.index, format_ident!("field_{}", i.index))
                        }
                    };
                }
                Self::Unnamed(*len, map)
            }
        }
    }
    // fn push_meta(&mut self, meta: &syn::Member) {
    //     *self = match (self, meta) {
    //         (Self::None, syn::Member::Named(m)) => Self::named_meta(m.clone()),
    //         (Self::None, syn::Member::Unnamed(m)) => Self::unnamed_meta(m),
    //         (Self::Named(map), syn::Member::Named(m)) => todo!(),
    //         (Self::Named(map), syn::Member::Unnamed(m)) => todo!(),
    //         (Self::Unnamed(map), syn::Member::Named(m)) => todo!(),
    //         (Self::Unnamed(map), syn::Member::Unnamed(m)) => todo!(),
    //     }
    // }
}
impl ToTokens for VariantConstructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_val = quote! { Default::default() };
        match self {
            Self::None => (),
            Self::Named(names) => syn::token::Brace::default().surround(tokens, |tokens| {
                let field = names
                    .iter()
                    .map(|(k, v)| quote! { #k: #v })
                    .collect::<Vec<_>>();
                tokens.extend(quote! { #(#field),* });
            }),
            Self::Unnamed(len, values) => syn::token::Paren::default().surround(tokens, |tokens| {
                for i in 0..u32::try_from(*len).unwrap() {
                    if let Some(name) = values.get(&i) {
                        name.to_tokens(tokens);
                    } else {
                        tokens.extend(default_val.clone());
                    }
                    <syn::Token![,]>::default().to_tokens(tokens);
                }
            }),
        }
    }
}

pub(crate) struct VariantData<'a> {
    enum_name: &'a syn::Ident,
    name: syn::Ident,
    fields: VariantFields,
    filtered_fields: Option<VariantFields>,
    constructor: VariantConstructor,
    tag: Option<String>,
    meta: Option<(syn::Member, syn::Type)>,
    items: RenJsonItems<syn::Ident>,
    arm_ser: Option<VariantArmSer>,
    // arm_de: Option<VariantArmDe>,
}
impl<'a> VariantData<'a> {
    pub fn new(enum_name: &'a syn::Ident, name: &syn::Ident, fields: syn::Fields) -> Self {
        Self {
            enum_name,
            name: name.clone(),
            fields: VariantFields::new(&fields),
            filtered_fields: None,
            constructor: VariantConstructor::new(&fields),
            tag: None,
            meta: None,
            items: RenJsonItems::None,
            arm_ser: None,
            // arm_de: None,
        }
    }
    pub fn apply_attribute(&mut self, attr: RenJsonAttribute) {
        if attr.tag.is_some() {
            self.tag = attr.tag;
        }
        if let Some(meta) = attr.meta {
            self.constructor.set_meta(&meta);
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
            self.arm_ser =
                Some(pat.with_variant_data(self.name.clone(), self.tag.as_ref(), &self.fields));
        }
        if let Some(items) = attr.items {
            self.constructor = self.constructor.with_items(&items);
            self.items = items
                .into_iter()
                .map(as_ident)
                // .filter(|ident| self.fields.contains_key(ident))
                .collect();
            self.filtered_fields = Some(self.fields.filtered_items(&self.items));
        }
    }
    pub fn split_arms(self) -> syn::Result<(VariantArmSer, VariantArmDe<'a>)> {
        let fields = self.fields.defaults();
        Ok((
            match self.arm_ser {
                Some(arm) => arm,
                None => VariantArmSer::from_field_defaults(
                    &self.name,
                    &fields,
                    // self.filtered_fields.as_ref().unwrap_or(&self.fields),
                )?
                .with_data(self.tag.as_ref(), self.meta.as_ref().map(|(i, _)| i)),
            },
            VariantArmDe::new(
                self.enum_name,
                self.tag,
                &self.name,
                VariantConstructor::from(&fields),
                &fields,
            )?,
            // match self.arm_de {
            //     Some(def) => def,
            //     None => VariantArmDe::from_fields(
            //         &self.name,
            //         &self.fields,
            //     )?, // .with_data(self.tag, self.meta.map(|(_, t)| t)),
            // },
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
            .field("arm", &self.arm_ser)
            // .field("def", &self.def)
            .finish()
    }
}
