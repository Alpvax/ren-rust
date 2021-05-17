use std::collections::HashMap;
use std::ops::Deref;

use crate::names::VarName;

type ParamCount = usize;

macro_rules! vtmap {
    ( $([ $( $t:ty $(,)?),+ ]: $vt:expr$(,)?),+ ) => {
        $($(impl ValueTypeMapper for $t {
            fn get_type_for(&self) -> ValueType {
                $vt
            }
        })+)+
    };
}

pub trait ValueTypeMapper {
    fn get_type_for(&self) -> ValueType;
}
impl<T> ValueTypeMapper for Option<T>
where
    T: ValueTypeMapper,
{
    fn get_type_for(&self) -> ValueType {
        match self {
            Some(v) => v.get_type_for(),
            None => ValueType::Undefined,
        }
    }
}

impl<T> ValueTypeMapper for Vec<T>
where
    T: ValueTypeMapper,
{
    fn get_type_for(&self) -> ValueType {
        ValueType::Array(self.iter().map(T::get_type_for).collect())
    }
}
impl ValueTypeMapper for HashMap<VarName, Value> {
    fn get_type_for(&self) -> ValueType {
        ValueType::Object(
            self.iter()
                .map(|(k, v)| (k.to_owned(), v.v_type.clone()))
                .collect(),
        )
    }
}

vtmap!(
    [()]: ValueType::Unknown,
    [String, &str]: ValueType::String,
    [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,]: ValueType::Number,
    [bool]: ValueType::Boolean,
);

#[derive(Debug, PartialEq)]
pub struct Value<T: ValueTypeMapper = ()> {
    v_type: ValueType,
    value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Function(Box<ValueType>, Box<ValueType>),
    Object(HashMap<VarName, ValueType>),
    Array(Vec<ValueType>),
    Number,
    String,
    Boolean,
    Undefined,
    Unknown,
}
impl ValueType {
    pub fn func_param_types(&self) -> Option<Vec<ValueType>> {
        self.function_type().map(|(p, _)| p)
    }
    pub fn function_type(&self) -> Option<(Vec<ValueType>, ValueType)> {
        if let ValueType::Function(a, b) = self {
            let mut res = vec![*a.clone()];
            let mut t = *b.clone();
            while let ValueType::Function(a, b) = t {
                res.push(*a);
                t = *b;
            }
            Some((res, t))
        } else {
            None
        }
    }
    pub fn function(params: Vec<ValueType>, ret_type: ValueType) -> Self {
        if params.len() > 0 {
            let mut t = ret_type;
            for p_t in params.into_iter().rev() {
                t = ValueType::Function(Box::new(p_t), Box::new(t));
            }
            t
        } else {
            Self::Unknown
        }
    }
    pub fn function_n(num_args: ParamCount) -> Self {
        let mut t = ValueType::Function(Box::new(ValueType::Unknown), Box::new(ValueType::Unknown));
        for _ in 1..num_args {
            t = ValueType::Function(Box::new(ValueType::Unknown), Box::new(t));
        }
        t
    }
    /*pub fn default_value<T>(self) -> Value<T>
    where T: ValueTypeMapper {
        Value {
            v_type: self,
            value: match &self {
                ValueType::Function(..) => (),
                ValueType::Object(..) => HashMap::new(),
                ValueType::Array(..) => Vec::new(),
                ValueType::Number => 0,
                ValueType::String => String::new(),
                ValueType::Boolean => false,
                ValueType::Undefined => (),
                ValueType::Unknown => (),
            }
        }
    }*/
}

#[allow(dead_code)]
impl Value {
    pub fn new<T, V>(value: V) -> Value<T>
    where
        T: ValueTypeMapper,
        V: Into<T>,
    {
        let v = value.into();
        Value {
            v_type: T::get_type_for(&v),
            value: v,
        }
    }
    pub fn from_type<T>(v_type: ValueType, value: T) -> Value<T>
    where
        T: ValueTypeMapper,
    {
        Value { v_type, value }
    }
    pub fn function_n(num_args: ParamCount) -> Self {
        Self::from_type(ValueType::function_n(num_args), ())
    }
    pub fn function(params: Vec<ValueType>, ret_type: ValueType) -> Self {
        let t = ValueType::function(params, ret_type);
        if let ValueType::Function(..) = t {
            Self {
                v_type: t,
                value: (), //TODO: function value
            }
        } else {
            Self::undefined()
        }
    }
    pub fn get_num_args(&self) -> ParamCount {
        let mut c = 0;
        let mut t = &self.v_type;
        while let ValueType::Function(_, r) = t {
            c += 1;
            t = r;
        }
        c
    }
    pub fn undefined() -> Value<()> {
        Value {
            v_type: ValueType::Undefined,
            value: (),
        }
    }
}
#[allow(dead_code)]
impl<T> Value<T>
where
    T: ValueTypeMapper,
{
    pub fn get(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for Value<T>
where
    T: ValueTypeMapper,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, ValueType as VT};
    #[test]
    fn test_undefined_type() {
        assert_eq!(
            Value::new(()),
            Value {
                v_type: VT::Unknown,
                value: (),
            }
        );
        assert_eq!(
            Value::undefined(),
            Value {
                v_type: VT::Undefined,
                value: (),
            }
        );
    }
    #[test]
    fn test_string_type() {
        assert_eq!(
            Value::new("string"),
            Value {
                v_type: VT::String,
                value: "string".to_owned(),
            }
        );
    }
    #[test]
    fn test_string_optional() {
        assert_eq!(
            Value::new(Some("string".to_owned())),
            Value {
                v_type: VT::String,
                value: Some("string".to_owned()),
            }
        );
        assert_eq!(
            Value::new::<Option<String>, Option<String>>(None),
            Value {
                v_type: VT::Undefined,
                value: None,
            }
        );
    }
    #[test]
    fn test_number_type() {
        assert_eq!(
            Value::new(3.14),
            Value {
                v_type: VT::Number,
                value: 3.14f64,
            }
        );
        assert_eq!(
            Value::new(5u8),
            Value {
                v_type: VT::Number,
                value: 5u8,
            }
        );
    }
    #[test]
    fn test_function_type() {
        assert_eq!(
            Value::from_type(VT::function_n(2), ()),
            Value {
                v_type: VT::Function(
                    Box::new(VT::Unknown),
                    Box::new(VT::Function(Box::new(VT::Unknown), Box::new(VT::Unknown)))
                ),
                value: (),
            }
        );
        assert_eq!(
            Value::function(vec![VT::String, VT::String, VT::Number], VT::Number),
            Value {
                v_type: VT::Function(
                    Box::new(VT::String),
                    Box::new(VT::Function(
                        Box::new(VT::String),
                        Box::new(VT::Function(Box::new(VT::Number), Box::new(VT::Number),))
                    ))
                ),
                value: (),
            }
        );
        assert_eq!(
            VT::Function(
                Box::new(VT::String),
                Box::new(VT::Function(
                    Box::new(VT::Number),
                    Box::new(VT::Array(vec![VT::String, VT::Number]))
                ))
            )
            .func_param_types(),
            Some(vec![VT::String, VT::Number])
        );
    }
}
