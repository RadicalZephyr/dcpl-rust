use dcpl::SExp;

mod interpreter;
pub use crate::interpreter::Runtime;

mod list;

#[derive(Clone, Debug, PartialEq)]
pub enum List {
    Cell { first: Box<Value>, rest: Box<Value> },
    Nil,
}

impl List {
    pub fn is_pair(&self) -> bool {
        match self {
            List::Cell { .. } => true,
            List::Nil => false,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

macro_rules! into_fns {
    {
        $( fn $name:ident() -> $res_type:tt ; )*
    } => {
        $(
            pub fn $name(self) -> Option<$res_type> {
                match self {
                    Value::$res_type(value) => Some(value),
                    _ => None,
                }
            }
        )*
    }
}

macro_rules! as_fns {
    {
        $( fn $name:ident() -> $res_type:tt ; )*
    } => {
        $(
            pub fn $name(&self) -> Option<& $res_type> {
                match self {
                    Value::$res_type(ref value) => Some(value),
                    _ => None,
                }
            }
        )*
    }
}

macro_rules! is_fns {
    {
        $(
            fn $name:ident() -> $( $p:pat )|+ => true;
        )*
    } => {
        $(
            pub fn $name(&self) -> bool {
                match self {
                    $( $p )|+ => true,
                    _ => false,
                }
            }
        )*
    }
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

    into_fns! {
        fn into_list() -> List;

        fn into_symbol() -> Symbol;

        fn into_string() -> String;

        fn into_integer() -> Integer;

        fn into_double() -> Double;

        fn into_bool() -> Bool;
    }

    as_fns! {
        fn as_list() -> List;

        fn as_symbol() -> Symbol;

        fn as_string() -> String;

        fn as_integer() -> Integer;

        fn as_double() -> Double;

        fn as_bool() -> Bool;
    }

    is_fns! {
        fn is_list() -> Value::List(_) => true;

        fn is_symbol() -> Value::Symbol(_) => true;

        fn is_string() -> Value::String(_) => true;

        fn is_number() -> Value::Integer(_) | Value::Double(_) => true;

        fn is_integer() -> Value::Integer(_) => true;

        fn is_double() -> Value::Double(_) => true;

        fn is_bool() -> Value::Bool(_) => true;
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Value::List(_) => false,
            _ => true,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(Bool(false)) | Value::List(List::Nil) => false,
            _ => true,
        }
    }
}

impl From<SExp> for Value {
    fn from(expr: SExp) -> Value {
        match expr {
            SExp::List(list) => Value::List(list.into_iter().map(Value::from).collect()),
            SExp::Integer(value) => Value::Integer(Integer(value)),
            SExp::Float(value) => Value::Double(Double(value)),
            SExp::String(value) => Value::String(value),
            SExp::Symbol(name) => Symbol(name).read(),
        }
    }
}
