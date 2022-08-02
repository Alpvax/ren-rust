use either::Either;
use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::serde_utils::{serialise_tagged, serialise_tagged_seq};

pub type StringPart<T> = Either<String, T>;
pub trait StringParts<T> {
    fn is_simple(&self) -> bool;
    fn as_simple_str(&self) -> Option<&String>;
    fn as_simple(&self) -> Option<String> {
        self.as_simple_str().cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<T> {
    Array(Vec<T>),
    Enum(String, Vec<T>),
    Number(f64),
    Record(Vec<(String, T)>),
    LStr(Vec<StringPart<T>>),
    // LUnit,
}
impl<T> StringParts<T> for Vec<StringPart<T>> {
    fn is_simple(&self) -> bool {
        self.len() == 1 && self[0].is_left()
    }
    fn as_simple_str(&self) -> Option<&String> {
        if self.len() == 1 {
            (&self[0]).as_ref().left()
        } else {
            None
        }
    }
}

impl<T> From<f64> for Literal<T> {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}
// Utility to help with not requiring i.0 suffix when creating
impl<T> From<i32> for Literal<T> {
    fn from(n: i32) -> Self {
        Self::Number(n.into())
    }
}
impl<T> From<String> for Literal<T> {
    fn from(s: String) -> Self {
        Self::LStr(vec![StringPart::Left(s)])
    }
}
impl<T> From<&str> for Literal<T> {
    fn from(s: &str) -> Self {
        Self::LStr(vec![StringPart::Left(s.to_owned())])
    }
}
impl<T> From<()> for Literal<T> {
    fn from(_: ()) -> Self {
        Self::Enum("undefined".to_owned(), Vec::new())
    }
}

impl<T> Serialize for Literal<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Literal::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                seq.serialize_element(&serde_json::json!({
                    "$": "Array",
                }))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Literal::Enum(name, args) => serialise_tagged!(serializer, "Enum", [], [name, args]),
            Literal::Number(num) => serialise_tagged!(serializer, "Number", [], [num]),
            Literal::Record(fields) => {
                let mut seq = serialise_tagged_seq(serializer, "Record", None, Some(fields.len()))?;
                for (k, v) in fields {
                    seq.serialize_element(&serde_json::json!([ { "$": "Field" }, k, v ]))?;
                }
                seq.end()
            }
            Literal::LStr(parts) => {
                let mut seq = serialise_tagged_seq(serializer, "String", None, Some(parts.len()))?;
                for p in parts {
                    match p {
                        StringPart::Left(s) => {
                            seq.serialize_element(&serde_json::json!([ { "$": "Text" }, s]))?
                        }
                        StringPart::Right(t) => seq.serialize_element(t)?,
                    }
                }
                seq.end()
            }
        }
    }
}
impl<'de, T> Deserialize<'de> for Literal<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
