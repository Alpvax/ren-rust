use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

mod attribute;
pub(crate) use attribute::RenJsonAttribute;
use attribute::RenJsonItems;

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
    fn from_fields(variant_name: &syn::Ident, fields: &VariantFields) -> syn::Result<Self> {
        if let FieldsKind::None = fields.kind {
            Ok(Self {
                variant_name: variant_name.clone(),
                tag: None,
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
                variant_name: variant_name.clone(),
                tag: None,
                meta,
                items: items.into(),
                pattern,
            })
        }
    }
    fn with_data(mut self, tag: Option<&String>, meta: Option<&syn::Member>) -> VariantArmSer {
        self.tag = tag.map(|s| s.to_string());
        self.meta = meta.map(as_ident);
        self
    }
}
impl core::fmt::Debug for VariantArmSer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantArm")
            .field("variant_name", &self.variant_name)
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
            if let Some(meta) = &self.meta {
                tokens.extend(quote! {
                    let mut map = if let ::serde_json::Value::Object(m) = ::serde_json::json!(#meta) {
                        m
                    } else {
                        ::serde_json::Map::new()
                    };
                });
            } else {
                tokens.extend(quote! { let mut map = ::serde_json::Map::new(); });
            }
            let tag = match &self.tag {
                Some(tag) => tag.to_token_stream(),
                None => self.variant_name.to_string().to_token_stream(),
            };
            let count = if self.has_items() { 2usize } else { 1 };
            tokens.extend(quote!{
                map.insert("$".to_string(), #tag.into());
                let mut seq = serializer.serialize_seq(Some(#count))?;
                seq.serialize_element(&map)?;
            });
            if self.has_items() {
                let items = &self.items;
                tokens.extend(quote!{
                    seq.serialize_element(&#items)?;
                });
            }
            tokens.extend(quote!{ seq.end() });
        });
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct VariantArmDe {
    variant_name: syn::Ident,
    generics: syn::Generics,
    tag: Option<String>,
    meta: Option<syn::Type>,
    items: RenJsonItems<syn::Type>,
}
#[allow(dead_code, unused_variables)]
impl<'a> VariantArmDe {
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
        todo!()
    }

    fn with_data(mut self, tag: Option<String>, meta: Option<syn::Type>) -> VariantArmDe {
        self.tag = tag;
        self.meta = meta;
        self
    }
}
impl<'a> core::fmt::Display for VariantArmDe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}
#[allow(unused_variables)]
impl<'a> ToTokens for VariantArmDe {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
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
    arm: Option<VariantArmSer>,
    def: Option<VariantArmDe>,
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
            self.arm =
                Some(pat.with_variant_data(self.name.clone(), self.tag.as_ref(), &self.fields));
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
    pub fn split_arms(self) -> syn::Result<(VariantArmSer, VariantArmDe)> {
        Ok((
            match self.arm {
                Some(arm) => arm,
                None => VariantArmSer::from_fields(
                    &self.name,
                    self.filtered_fields.as_ref().unwrap_or(&self.fields),
                )?
                .with_data(self.tag.as_ref(), self.meta.as_ref().map(|(i, _)| i)),
            },
            match self.def {
                Some(def) => def,
                None => VariantArmDe::from_fields(
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
