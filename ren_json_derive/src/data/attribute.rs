use std::fmt::Debug;

use quote::TokenStreamExt;

use super::{debug_member, FieldsKind};

macro_rules! parse_unordered_fields {
    ($($name:ident = $($tok:expr),+ => $typ:ty),+ $(,)?) => {
        |content: syn::parse::ParseStream| -> syn::Result<($(Option<$typ>),+)> {
            $(let mut $name = None;)+
            let mut errors: Vec<syn::Error> = Vec::new();
            loop {
                let lookahead = content.lookahead1();
                if content.is_empty() {
                    break;
                }$(
                    else if $(lookahead.peek($tok))||+ {
                        if $name.is_some() {
                            errors.push(content.error(format!("Duplicate {} field", stringify!($name))));
                        } else {
                            $name = Some(content.parse::<$typ>()?);
                        }
                    }
                )+ else {
                    errors.push(lookahead.error());
                    break;
                }
                if content.peek(syn::Token![,]) {
                    content.parse::<syn::Token![,]>()?;
                    continue;
                }
                break;
            }
            match errors.into_iter().reduce(|mut acc, e| {
                acc.combine(e);
                acc
            }) {
                Some(err) => Err(err),
                None => Ok(($($name),+))
            }
        }
    };
}

mod kw {
    syn::custom_keyword!(tag);
    syn::custom_keyword!(meta);
    syn::custom_keyword!(items);
}

struct RenJsonTagField(syn::LitStr);
impl syn::parse::Parse for RenJsonTagField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<kw::tag>()?;
        input.parse::<syn::Token![=]>()?;
        Ok(Self(input.parse()?))
    }
}
impl From<RenJsonTagField> for String {
    fn from(field: RenJsonTagField) -> Self {
        field.0.value()
    }
}

struct RenJsonMetaField<T>(T);
impl<T> syn::parse::Parse for RenJsonMetaField<T>
where
    T: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<kw::meta>()?;
        input.parse::<syn::Token![=]>()?;
        Ok(Self(input.parse()?))
    }
}
// impl From<RenJsonMetaField<syn::Member>> for syn::Member {
//     fn from(field: RenJsonMetaField<syn::Member>) -> Self {
//         field.0
//     }
// }
// impl From<RenJsonMetaField<syn::Ident>> for syn::Member {
//     fn from(field: RenJsonMetaField<syn::I>) -> Self {
//         field.0.into()
//     }
// }
// impl From<RenJsonMetaField<syn::Ident>> for syn::Ident {
//     fn from(field: RenJsonMetaField<syn::Ident>) -> Self {
//         field.0
//     }
// }
// impl<T, U> From<RenJsonMetaField<T>> for Option<U> where T: syn::parse::Parse + Into<U> {
//     fn from(field: RenJsonMetaField<T>) -> Self {
//         Some(field.0.into())
//     }
// }
impl<T> From<RenJsonMetaField<T>> for Option<syn::Member>
where
    T: Into<syn::Member>,
{
    fn from(field: RenJsonMetaField<T>) -> Self {
        Some(field.0.into())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RenJsonItems<T> {
    /// [{$: tag, ...meta}]
    None,
    /// [{$: tag, ...meta}, value]
    SingleValue(T),
    /// [{$: tag, ...meta}, [value]]
    SingleList(T),
    /// [{$: tag, ...meta}, [...values]]
    Multiple(Vec<T>),
}
impl<T> syn::parse::Parse for RenJsonItems<T>
where
    T: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let content;
        if lookahead.peek(syn::token::Paren) {
            syn::parenthesized!(content in input);
        } else if lookahead.peek(syn::token::Bracket) {
            syn::bracketed!(content in input);
        } else {
            return input.parse().map(Self::SingleValue);
        }
        Ok(if content.is_empty() {
            Self::None
        } else {
            let items = input.parse_terminated::<_, syn::Token![,]>(T::parse)?;
            if items.len() > 1 {
                Self::Multiple(items.into_iter().collect())
            } else {
                Self::SingleList(items.into_iter().next().expect(&format!("Error unwrapping first value from SingleList with no items! should be unreachable: content = {}", content)))
            }
        })
    }
}
impl<T> RenJsonItems<T>
where
    T: quote::ToTokens,
{
    pub fn map<U, F>(self, mut f: F) -> RenJsonItems<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Self::None => RenJsonItems::None,
            Self::SingleValue(v) => RenJsonItems::SingleValue(f(v)),
            Self::SingleList(v) => RenJsonItems::SingleList(f(v)),
            Self::Multiple(v) => RenJsonItems::Multiple(v.into_iter().map(f).collect()),
        }
    }
    pub fn iter(&self) -> ItemsIntoIterator<&T, impl Iterator<Item = &T>> {
        match self {
            Self::None => ItemsIntoIterator::Empty,
            Self::SingleValue(v) | Self::SingleList(v) => ItemsIntoIterator::Single(v),
            Self::Multiple(vec) => ItemsIntoIterator::Vec(vec.iter()),
        }
    }
    /// Create a new iterator with the filtered elements, leaving the current items unaffected
    pub fn filtered<F>(&self, mut f: F) -> RenJsonItems<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        match self {
            Self::None => RenJsonItems::None,
            Self::SingleValue(v) => {
                if f(v) {
                    RenJsonItems::SingleValue(v.clone())
                } else {
                    RenJsonItems::None
                }
            }
            Self::SingleList(v) => {
                if f(v) {
                    RenJsonItems::SingleList(v.clone())
                } else {
                    RenJsonItems::None
                }
            }
            Self::Multiple(vec) => vec.iter().cloned().filter(f).collect(),
        }
    }
    /// Create a new iterator with the first n elements, leaving the current items unaffected
    pub fn first_n(&self, n: usize) -> RenJsonItems<T>
    where
        T: Clone,
    {
        if n < 1 {
            RenJsonItems::None
        } else {
            match self {
                Self::None => RenJsonItems::None,
                Self::SingleValue(v) => RenJsonItems::SingleValue(v.clone()),
                Self::SingleList(v) => RenJsonItems::SingleList(v.clone()),
                Self::Multiple(vec) => vec.iter().cloned().take(n).collect(),
            }
        }
    }
}
impl<T> Default for RenJsonItems<T> {
    fn default() -> Self {
        Self::None
    }
}
impl<T> Default for &RenJsonItems<T> {
    fn default() -> Self {
        &RenJsonItems::None
    }
}
impl From<&RenJsonItems<syn::Ident>> for RenJsonItems<syn::Member> {
    fn from(items: &RenJsonItems<syn::Ident>) -> Self {
        items.clone().map(syn::Member::from)
    }
}
impl From<Vec<&syn::Ident>> for RenJsonItems<syn::Ident> {
    fn from(items: Vec<&syn::Ident>) -> Self {
        items.into_iter().cloned().collect()
    }
}
impl From<Vec<syn::Ident>> for RenJsonItems<syn::Ident> {
    fn from(items: Vec<syn::Ident>) -> Self {
        items.into_iter().collect()
    }
}
impl From<Vec<&syn::Type>> for RenJsonItems<syn::Type> {
    fn from(items: Vec<&syn::Type>) -> Self {
        items.into_iter().cloned().collect()
    }
}
pub(crate) enum ItemsIntoIterator<T, V: Iterator<Item = T>> {
    Empty,
    Single(T),
    Vec(V),
}
impl<T, V: Iterator<Item = T>> Iterator for ItemsIntoIterator<T, V> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Self::Vec(iter) = self {
            iter.next()
        } else if let Self::Empty = self {
            None
        } else {
            if let Self::Single(v) = std::mem::replace(self, Self::Empty) {
                Some(v)
            } else {
                unreachable!()
            }
        }
    }
}
impl<T> IntoIterator for RenJsonItems<T> {
    type Item = T;

    type IntoIter = ItemsIntoIterator<T, <Vec<T> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            RenJsonItems::None => ItemsIntoIterator::Empty,
            RenJsonItems::SingleValue(v) | RenJsonItems::SingleList(v) => {
                ItemsIntoIterator::Single(v)
            }
            RenJsonItems::Multiple(vec) => ItemsIntoIterator::Vec(vec.into_iter()),
        }
    }
}
impl<T> FromIterator<T> for RenJsonItems<T>
where
    T: quote::ToTokens,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut items = iter.into_iter().collect::<Vec<_>>();
        match items.len() {
            0 => Self::None,
            1 => Self::SingleValue(
                items
                    .pop()
                    .expect("collecting SingleList from iterator: unwrapping"),
            ),
            _ => Self::Multiple(items),
        }
    }
}
impl<T> ::quote::ToTokens for RenJsonItems<T>
where
    T: ::quote::ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RenJsonItems::None => {}
            RenJsonItems::SingleValue(v) => v.to_tokens(tokens),
            RenJsonItems::SingleList(v) => {
                syn::token::Bracket::default().surround(tokens, |tokens| v.to_tokens(tokens))
            }
            RenJsonItems::Multiple(vec) => syn::token::Paren::default()
                .surround(tokens, |tokens| {
                    tokens.append_separated(vec.iter(), syn::token::Comma::default())
                }),
        }
    }
}

struct RenJsonItemsField<T>(RenJsonItems<T>)
where
    T: syn::parse::Parse;
impl<T> syn::parse::Parse for RenJsonItemsField<T>
where
    T: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<kw::items>()?;
        input.parse::<syn::Token![=]>()?;
        Ok(Self(input.parse()?))
    }
}
// impl From<RenJsonPatField<syn::Member>> for Vec<syn::Member> {
//     fn from(items: RenJsonPatField<syn::Member>) -> Self {
//         items.0
//     }
// }

pub(super) struct RenJsonPatField(
    syn::Pat,
    Option<syn::Ident>,
    Option<RenJsonItems<syn::Ident>>,
);
impl syn::parse::Parse for RenJsonPatField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pat = input.parse()?;
        let lookahead = input.lookahead1();
        let (meta, items) = if input.is_empty() || lookahead.peek(syn::token::Comma) {
            match &pat {
                syn::Pat::Struct(syn::PatStruct { fields, .. }) => {
                    let mut meta = None;
                    let mut items = Vec::new();
                    for syn::FieldPat { pat: f_pat, .. } in fields {
                        if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &**f_pat {
                            let name = ident.to_string();
                            if name == "meta" {
                                if meta.is_none() {
                                    meta = Some(ident.clone());
                                } else {
                                    return Err(input.error("duplicate pattern field \"meta\""));
                                }
                            } else if name.starts_with("item") {
                                if name.len() > 4 {
                                    items.push((ident.clone(), (&name[4..]).parse::<usize>().ok()));
                                } else {
                                    items.push((ident.clone(), None));
                                }
                            }
                        } else {
                            return Err(input.error("Unsupported pattern match"));
                        }
                    }
                    if items.iter().all(|(_, i)| i.is_some()) {
                        items.sort_by_cached_key(|i| i.1);
                    }
                    (meta, Some(items.into_iter().map(|(n, _)| n).collect()))
                }
                syn::Pat::Tuple(syn::PatTuple { elems, .. })
                | syn::Pat::TupleStruct(syn::PatTupleStruct {
                    pat: syn::PatTuple { elems, .. },
                    ..
                }) => {
                    let mut meta = None;
                    let mut items = Vec::new();
                    for elem in elems {
                        if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = elem {
                            let name = ident.to_string();
                            if name == "meta" {
                                if meta.is_none() {
                                    meta = Some(ident.clone());
                                } else {
                                    return Err(input.error("duplicate pattern field \"meta\""));
                                }
                            } else if name.starts_with("item") {
                                if name.len() > 4 {
                                    items.push((ident.clone(), (&name[4..]).parse::<usize>().ok()));
                                } else {
                                    items.push((ident.clone(), None));
                                }
                            }
                        } else {
                            return Err(input.error("Unsupported pattern match"));
                        }
                    }
                    if items.iter().all(|(_, i)| i.is_some()) {
                        items.sort_by_cached_key(|i| i.1);
                    }
                    (meta, Some(items.into_iter().map(|(n, _)| n).collect()))
                }
                _ => todo!(),
            }
        } else if lookahead.peek(syn::Token![=>]) {
            input.parse::<syn::Token![=>]>()?;
            if input.peek(syn::token::Brace) {
                let content;
                syn::braced!(content in input);
                if content.is_empty() {
                    return Err(content.error("braced pattern result must match the format `{meta, items}` or `{items}` where meta is an ident, and items is RenJsonItems"));
                }
                (
                    if content.peek2(syn::Token![,]) {
                        let meta = content.parse::<syn::Ident>()?;
                        content.parse::<syn::Token![,]>()?;
                        Some(meta)
                    } else {
                        None
                    },
                    Some(content.parse::<RenJsonItems<syn::Ident>>()?),
                )
            } else {
                let (meta, items) = parse_unordered_fields!(
                    meta = kw::meta => RenJsonMetaField<syn::Ident>,
                    items = kw::items => RenJsonItemsField<syn::Ident>,
                )(&input)?;
                (meta.map(|m| m.0), items.map(|i| i.0))
            }
        } else {
            return Err(lookahead.error());
        };
        Ok(Self(pat, meta, items))
    }
}
impl RenJsonPatField {
    pub fn with_variant_data<'a>(
        &self,
        variant_name: syn::Ident,
        tag: Option<&String>,
        fields: &super::VariantFields,
    ) -> super::VariantArmSer {
        let items = self.2.as_ref().unwrap_or_default();
        let pat = &self.0;
        let pattern = syn::parse_quote! { Self::#variant_name #pat };
        super::VariantArmSer {
            variant_name,
            tag: tag.map(|s| s.to_string()),
            meta: self.1.as_ref().cloned(),
            items: if fields.kind == FieldsKind::Named {
                items.filtered(|item| fields.contains(item))
            } else {
                items.first_n(fields.len())
            },
            pattern,
        }
    }
    pub fn items(&self) -> RenJsonItems<syn::Ident> {
        self.2.as_ref().cloned().unwrap_or_default()
    }
}

// enum RenJsonAttributeField {
//     Tag(String),
//     Meta(syn::Member),
//     Items(syn::punctuated::Punctuated<syn::Member, syn::token::Comma>),
//     Pattern(syn::Pat, Option<syn::Ident>, Option<Vec<syn::Ident>>),
// }
// impl syn::parse::Parse for RenJsonAttributeField {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
//             match input.parse::<syn::Ident>()?.to_string().as_str() {
//                 "tag" | "name" => {
//                     input.parse::<syn::Token![=]>()?;
//                     Ok(Self::Tag(input.parse::<syn::LitStr>()?.value()))
//                 }
//                 "meta" => {
//                     input.parse::<syn::Token![=]>()?;
//                     Ok(Self::Meta(input.parse::<syn::Member>()?))
//                 }
//                 "items" => {
//                     input.parse::<syn::Token![=]>()?;
//                     let content;
//                     if input.peek(syn::token::Paren) {
//                         syn::parenthesized!(content in input);
//                     } else if input.peek(syn::token::Bracket) {
//                         syn::bracketed!(content in input);
//                     } else {
//                         // return Err(input.error("items = not (..) or [..]"));
//                         return Ok(Self::Items(syn::punctuated::Punctuated::new()));
//                     }
//                     Ok(Self::Items(syn::punctuated::Punctuated::parse_terminated(
//                         &content,
//                     )?))
//                 }
//                 key => Err(input.error(format!(
//                     "Unexpected key: {}, should be one of [tag, meta, items]",
//                     key
//                 ))),
//             }
//         } else {
//             input.parse::<syn::Pat>().and_then( |pat| {
//                 if input.is_empty() || input.peek(syn::token::Comma) {
//                     todo!("parse meta, items from pat")
//                 } else if input.peek(syn::Token![=>]) {
//                     input.parse::<syn::Token![=>]>()?;
//                     todo!("parse meta, items from pat expr")
//                 } else {
//                     Err(input.error("pattern followed by invalid symbol. (should be \",\" or \"=> ...\")"))
//                 }
//             }).map_err(|e| {
//                 let mut err = input.error(format!("Not a (key = value) attribute field or pattern: {:?}", input));
//                 err.combine(e);
//                 err
//             })
//         }
//     }
// }
// impl std::fmt::Debug for RenJsonAttributeField {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Tag(tag) => f.debug_tuple("Tag").field(tag).finish(),
//             Self::Meta(mem) => f.debug_tuple("Meta").field(&format_member(mem)).finish(),
//             Self::Items(items) => f
//                 .debug_tuple("Items")
//                 .field(
//                     &items
//                         .iter()
//                         .map(format_member)
//                         .collect::<Vec<_>>()
//                         .join(", "),
//                 )
//                 .finish(),
//             Self::Pattern(_pat, _meta, _items) => todo!(),
//         }
//     }
// }

pub(crate) struct RenJsonAttribute {
    pub tag: Option<String>,
    pub meta: Option<syn::Member>,
    pub items: Option<RenJsonItems<syn::Member>>,
    pub(super) pattern: Option<RenJsonPatField>,
}
impl core::fmt::Debug for RenJsonAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenJsonAttribute")
            .field("tag", &self.tag)
            .field("meta", &self.meta.as_ref().map(debug_member))
            .field(
                "items",
                &self
                    .items
                    .as_ref()
                    .map(|items| items.iter().map(debug_member).collect::<Vec<_>>()),
            )
            .field("pattern", &"...") //&self.pattern)
            .finish()
    }
}
impl syn::parse::Parse for RenJsonAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        if input.peek(syn::token::Paren) {
            syn::parenthesized!(content in input);
        } else if input.peek(syn::token::Bracket) {
            syn::bracketed!(content in input);
        } else if input.peek(syn::token::Brace) {
            syn::braced!(content in input);
        } else {
            return Ok(Self {
                tag: None,
                meta: None,
                items: None,
                pattern: None,
            });
        }
        let (tag, meta, items, pattern) = parse_unordered_fields!(
            tag = kw::tag => RenJsonTagField,
            meta = kw::meta => RenJsonMetaField<syn::Member>,
            items = kw::items => RenJsonItemsField<syn::Member>,
            pattern = syn::token::Paren, syn::token::Brace => RenJsonPatField,
        )(&content)?;
        Ok(if let Some(pat) = pattern {
            Self {
                tag: tag.map(|t| t.0.value()),
                meta: meta
                    .map(|m| m.0)
                    .or(pat.1.as_ref().map(|m| m.clone().into())),
                items: items
                    .map(|i| i.0)
                    .or(pat.2.as_ref().map(|m| RenJsonItems::from(m))),
                pattern: Some(pat),
            }
        } else {
            Self {
                tag: tag.map(|t| t.0.value()),
                meta: meta.map(|m| m.0),
                items: items.map(|i| i.0),
                pattern: None,
            }
        })
    }
}
