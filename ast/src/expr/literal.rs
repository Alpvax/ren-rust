use either::Either;

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
    LUnit,
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
