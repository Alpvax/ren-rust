use std::collections::HashMap;

use ren_json_derive::RenJson;

#[derive(Debug, Clone, PartialEq, Eq, RenJson)]
pub enum Type {
    /// any type, e.g. "*"
    Any,
    /// type application, e.g. "Array a"
    App(Box<Type>, Vec<Type>),
    /// concrete type constructor, e.g. "Number"
    Con(String),
    /// function type, e.g. "Number -> Number"
    Fun(Box<Type>, Box<Type>),
    /// unknown (to the user) type, e.g. "?"
    Hole,
    /// record type, e.g. "{x: Number, y: Number}"
    Rec(Row),
    /// sum type, e.g. "#ok a | #err e"
    Sum(Row),
    /// type variable, e.g. "a"
    Var(String),
}
impl Default for Type {
    fn default() -> Self {
        Self::Hole
    }
}

type Row = HashMap<String, Vec<Type>>;

impl Type {
    // CONSTRUCTORS ----------------------------------------------------------------
    pub fn access(k: &str) -> Self {
        Self::fun([Self::rec([(k, Self::var("a"))])], Self::var("a"))
    }
    pub fn arr(typ: Type) -> Self {
        Self::App(Box::new(Self::Con("Array".to_string())), vec![typ])
    }
    pub fn boolean() -> Self {
        Self::sum([("true", []), ("false", [])])
    }
    pub fn fun<T>(args: T, ret: Type) -> Self
    where
        T: IntoIterator<Item = Type>,
        T::IntoIter: DoubleEndedIterator,
    {
        args.into_iter()
            .rfold(ret, |ret, arg| Self::Fun(Box::new(arg), Box::new(ret)))
    }
    pub fn num() -> Self {
        Self::Con("Number".to_string())
    }
    pub fn rec<I, S>(rows: I) -> Self
    where
        I: IntoIterator<Item = (S, Type)>,
        S: ToString,
    {
        Self::Rec(
            rows.into_iter()
                .map(|(k, t)| (k.to_string(), vec![t]))
                .collect(),
        )
    }
    pub fn string() -> Self {
        Self::Con("String".to_string())
    }
    pub fn sum<I, S, T>(rows: I) -> Self
    where
        I: IntoIterator<Item = (S, T)>,
        S: ToString,
        T: IntoIterator<Item = Type>,
    {
        Self::Sum(
            rows.into_iter()
                .map(|(k, types)| (k.to_string(), types.into_iter().collect::<Vec<_>>()))
                .collect(),
        )
    }
    pub fn undefined() -> Self {
        Self::sum([("undefined", [])])
    }
    pub fn var<S>(v: S) -> Self
    where
        S: ToString,
    {
        Self::Var(v.to_string())
    }

    // fn fresh(n: u8) -> String {
    //     if n >= 26 {
    //         format!("{}{}", Self::fresh((n / 26) - 1), Self::fresh(n % 26))
    //     } else {
    //         char::from(n).to_string()
    //     }
    // }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn parens(t: &Type) -> String {
            match t {
                Type::App(..) | Type::Fun(..) => format!("({})", t),
                _ => t.to_string(),
            }
        }
        match self {
            Type::Any => write!(f, "*"),
            Type::App(t1, tn) => write!(
                f,
                "{} {}",
                parens(&*t1),
                tn.iter().map(parens).collect::<Vec<_>>().join(" ")
            ),
            Type::Con(c) => write!(f, "{}", c),
            Type::Fun(t1, t2) => write!(f, "{} → {}", parens(&*t1), *t2),
            Type::Hole => write!(f, "?"),
            Type::Rec(r) => {
                write!(f, "{{")?;
                let mut it = r.iter();
                let mut kv = it.next();
                while let Some((k, v)) = kv {
                    write!(f, "{} :", k)?;
                    for t in v {
                        write!(f, " {}", t)?;
                    }
                    kv = it.next();
                    if kv.is_some() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "}}")
            }
            Type::Sum(r) => {
                write!(f, "[")?;
                let mut it = r.iter();
                let mut kv = it.next();
                while let Some((k, v)) = kv {
                    write!(f, "#{} :", k)?;
                    for t in v {
                        write!(f, " {}", parens(t))?;
                    }
                    kv = it.next();
                    if kv.is_some() {
                        write!(f, "|")?;
                    }
                }
                write!(f, "]")
            }
            Type::Var(v) => write!(f, "{}", v),
        }
    }
}
