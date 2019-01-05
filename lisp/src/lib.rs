use dcpl::SExp;

mod interpreter;
pub use crate::interpreter::Runtime;

#[derive(Clone, Debug, PartialEq)]
pub struct List(Vec<Value>);

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol(String);

impl Symbol {
    pub fn read(self) -> Value {
        match self.0.as_ref() {
            "true" => Value::Bool(Bool(true)),
            "false" => Value::Bool(Bool(false)),
            _ => Value::Symbol(self),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integer(i128);

#[derive(Clone, Debug, PartialEq)]
pub struct Double(f64);

#[derive(Clone, Debug, PartialEq)]
pub struct Bool(bool);

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    List(List),
    Symbol(Symbol),
    String(String),
    Integer(Integer),
    Double(Double),
    Bool(Bool),
}

impl Value {
    pub fn string(value: impl Into<String>) -> Value {
        Value::String(value.into())
    }

    pub fn symbol(name: impl Into<String>) -> Value {
        Value::Symbol(Symbol(name.into()))
    }

    pub fn integer(value: i128) -> Value {
        Value::Integer(Integer(value))
    }

    pub fn double(value: f64) -> Value {
        Value::Double(Double(value))
    }

    pub fn bool(value: bool) -> Value {
        Value::Bool(Bool(value))
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Value::List(_) => false,
            _ => true,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        use self::Value::*;
        match self {
            Integer(_) | Double(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Double(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
}

impl From<SExp> for Value {
    fn from(expr: SExp) -> Value {
        match expr {
            SExp::List(list) => Value::List(List(list.into_iter().map(Value::from).collect())),
            SExp::Integer(value) => Value::Integer(Integer(value)),
            SExp::Float(value) => Value::Double(Double(value)),
            SExp::String(value) => Value::String(value),
            SExp::Symbol(name) => Symbol(name).read(),
        }
    }
}
