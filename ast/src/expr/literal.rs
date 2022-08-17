use either::Either;
use ren_json_derive::RenJson;
use serde::Deserialize;

// use crate::serde_utils::{serialise_tagged, serialise_tagged_seq};

#[derive(Debug, Clone, PartialEq, RenJson)]
pub enum StringPart<T>
where
    T: crate::ASTType,
{
    Text(String),
    Value(T),
}

impl<T> StringPart<T>
where
    T: crate::ASTType,
{
    pub fn is_text(&self) -> bool {
        match self {
            Self::Text(_) => true,
            Self::Value(_) => false,
        }
    }
    pub fn text(&self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text.clone()),
            Self::Value(_) => None,
        }
    }
    pub fn map<U, F>(self, f: F) -> StringPart<U>
    where
        F: FnOnce(T) -> U,
        U: crate::ASTType,
    {
        match self {
            Self::Text(s) => StringPart::Text(s),
            Self::Value(v) => StringPart::Value(f(v)),
        }
    }
}
impl<T> std::str::FromStr for StringPart<T>
where
    T: crate::ASTType,
{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Text(s.to_string()))
    }
}
impl<T> From<String> for StringPart<T>
where
    T: crate::ASTType,
{
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}
impl<T> From<&str> for StringPart<T>
where
    T: crate::ASTType,
{
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}
impl<T> From<T> for StringPart<T>
where
    T: crate::ASTType,
{
    fn from(v: T) -> Self {
        Self::Value(v)
    }
}
impl<S, T> From<Either<S, T>> for StringPart<T>
where
    S: Into<String>,
    T: crate::ASTType,
{
    fn from(e: Either<S, T>) -> Self {
        match e {
            Either::Left(s) => Self::Text(s.into()),
            Either::Right(v) => Self::Value(v),
        }
    }
}
impl<T, S> From<StringPart<T>> for Either<S, T>
where
    S: From<String>,
    T: crate::ASTType,
{
    fn from(sp: StringPart<T>) -> Self {
        match sp {
            StringPart::Text(s) => Either::Left(s.into()),
            StringPart::Value(v) => Either::Right(v),
        }
    }
}
pub trait StringParts<T> {
    fn is_simple(&self) -> bool;
    fn as_simple(&self) -> Option<String>;
}
impl<T> StringParts<T> for Vec<StringPart<T>>
where
    T: crate::ASTType,
{
    fn is_simple(&self) -> bool {
        self.len() == 1 && self[0].is_text()
    }
    fn as_simple(&self) -> Option<String> {
        if self.len() == 1 {
            (&self[0]).text()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, RenJson)]
pub enum Literal<T>
where
    T: crate::ASTType,
{
    Array(Vec<T>),
    Enum(String, Vec<T>),
    Number(f64),
    Record(Vec<(String, T)>),
    #[ren_json(tag = "String")]
    LStr(Vec<StringPart<T>>),
    // LUnit,
}

impl<T> From<f64> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}
// Utility to help with not requiring i.0 suffix when creating
impl<T> From<i32> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(n: i32) -> Self {
        Self::Number(n.into())
    }
}
impl<T> From<String> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(s: String) -> Self {
        Self::LStr(vec![StringPart::Text(s)])
    }
}
impl<T> From<&str> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(s: &str) -> Self {
        Self::LStr(vec![StringPart::Text(s.to_owned())])
    }
}
impl<T> From<()> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(_: ()) -> Self {
        Self::Enum("undefined".to_owned(), Vec::new())
    }
}

// impl<T> Serialize for Literal<T>
// where
//     T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeSeq;
//         match self {
//             Literal::Array(items) => {
//                 let mut seq = serializer.serialize_seq(Some(items.len()))?;
//                 seq.serialize_element(&serde_json::json!({
//                     "$": "Array",
//                 }))?;
//                 for item in items {
//                     seq.serialize_element(item)?;
//                 }
//                 seq.end()
//             }
//             Literal::Enum(name, args) => serialise_tagged!(serializer, "Enum", [], [name, args]),
//             Literal::Number(num) => serialise_tagged!(serializer, "Number", [], [num]),
//             Literal::Record(fields) => {
//                 let mut seq = serialise_tagged_seq(serializer, "Record", None, Some(fields.len()))?;
//                 for (k, v) in fields {
//                     seq.serialize_element(&serde_json::json!([ { "$": "Field" }, k, v ]))?;
//                 }
//                 seq.end()
//             }
//             Literal::LStr(parts) => {
//                 let mut seq = serialise_tagged_seq(serializer, "String", None, Some(parts.len()))?;
//                 for p in parts {
//                     match p {
//                         StringPart::Text(s) => {
//                             seq.serialize_element(&serde_json::json!([ { "$": "Text" }, s]))?
//                         }
//                         StringPart::Value(t) => seq.serialize_element(t)?,
//                     }
//                 }
//                 seq.end()
//             }
//         }
//     }
// }
impl<T> From<Vec<T>> for Literal<T>
where
    T: crate::ASTType,
{
    fn from(items: Vec<T>) -> Self {
        Self::Array(items)
    }
}
impl<T> FromIterator<T> for Literal<T>
where
    T: crate::ASTType,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Array(iter.into_iter().collect())
    }
}
impl<'de, T> Deserialize<'de> for Literal<T>
where
    T: Deserialize<'de> + crate::ASTType,
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
impl<T> FromIterator<(String, T)> for Literal<T>
where
    T: crate::ASTType,
{
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self::Record(iter.into_iter().collect())
    }
}
impl<T> FromIterator<StringPart<T>> for Literal<T>
where
    T: crate::ASTType,
{
    fn from_iter<I: IntoIterator<Item = StringPart<T>>>(iter: I) -> Self {
        Self::LStr(iter.into_iter().collect())
    }
}
