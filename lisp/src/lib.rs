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
    pub fn is_atom(&self) -> bool {
        match self {
            Value::List(_) => false,
            _ => true,
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
