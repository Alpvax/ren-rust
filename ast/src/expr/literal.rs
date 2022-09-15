use either::Either;
use ren_json_derive::RenJson;

#[derive(Debug, Clone, PartialEq, RenJson)]
#[ren_json(T)]
pub enum StringPart<T> {
    Text(String),
    Value(T),
}

impl<T> StringPart<T> {
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
    {
        match self {
            Self::Text(s) => StringPart::Text(s),
            Self::Value(v) => StringPart::Value(f(v)),
        }
    }
}
impl<T> std::str::FromStr for StringPart<T> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Text(s.to_string()))
    }
}
impl<T> From<String> for StringPart<T> {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}
impl<T> From<&str> for StringPart<T> {
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}
impl<T> From<T> for StringPart<T>
where
    T: crate::ASTLiteralType,
{
    fn from(v: T) -> Self {
        Self::Value(v)
    }
}
impl<S, T> From<Either<S, T>> for StringPart<T>
where
    S: Into<String>,
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
impl<T> StringParts<T> for Vec<StringPart<T>> {
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
#[ren_json(T)]
pub enum Literal<T> {
    Array(Vec<T>),
    Enum(String, Vec<T>),
    Number(f64),
    //TODO: Add Field object wrappers to json,
    Record(Vec<(String, T)>),
    #[ren_json(tag = "String")]
    LStr(Vec<StringPart<T>>),
    // LUnit,
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
        Self::LStr(vec![StringPart::Text(s)])
    }
}
impl<T> From<&str> for Literal<T> {
    fn from(s: &str) -> Self {
        Self::LStr(vec![StringPart::Text(s.to_owned())])
    }
}
impl<T> From<()> for Literal<T> {
    fn from(_: ()) -> Self {
        Self::Enum("undefined".to_owned(), Vec::new())
    }
}
impl<T> From<Vec<T>> for Literal<T> {
    fn from(items: Vec<T>) -> Self {
        Self::Array(items)
    }
}
impl<T> From<Vec<(String, T)>> for Literal<T> {
    fn from(items: Vec<(String, T)>) -> Self {
        items.into_iter().collect()
    }
}
impl<'s, T> From<Vec<(&'s str, T)>> for Literal<T> {
    fn from(items: Vec<(&'s str, T)>) -> Self {
        items.into_iter().collect()
    }
}
impl<T> FromIterator<T> for Literal<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Array(iter.into_iter().collect())
    }
}
impl<T> FromIterator<(String, T)> for Literal<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self::Record(iter.into_iter().collect())
    }
}
impl<'s, T> FromIterator<(&'s str, T)> for Literal<T> {
    fn from_iter<I: IntoIterator<Item = (&'s str, T)>>(iter: I) -> Self {
        Self::Record(iter.into_iter().map(|(s, v)| (s.to_string(), v)).collect())
    }
}
impl<T> FromIterator<StringPart<T>> for Literal<T> {
    fn from_iter<I: IntoIterator<Item = StringPart<T>>>(iter: I) -> Self {
        Self::LStr(iter.into_iter().collect())
    }
}
