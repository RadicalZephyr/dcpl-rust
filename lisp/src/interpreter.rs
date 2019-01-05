use crate::{Symbol, Value};
use dcpl::SExp;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    UndefinedSymbol,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq)]
struct Env {}

impl Env {
    fn lookup(&self, _name: Symbol) -> Option<Value> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime {
    env: Env,
}

impl Runtime {
    pub fn new() -> Runtime {
        let env = Env {};
        Runtime { env }
    }

    pub fn rep_iter(&mut self, expr: SExp) -> Option<String> {
        match self.eval(expr.into()) {
            Ok(expr) => Some(format!("{:?}", expr)),
            Err(error) => Some(format!("{:?}", error)),
        }
    }

    pub fn eval(&mut self, expr: Value) -> Result<Value, Error> {
        if expr.is_atom() {
            match expr {
                Value::Symbol(name) => self.env.lookup(name).ok_or(Error::UndefinedSymbol),
                _ => Ok(expr),
            }
        } else {
            Err(Error::NotImplemented)
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    // use dcpl::SExp;
    // use dcpl::SExp::*;

    // #[test]
    // fn test_eval_integer() {
    //     let mut rt = Runtime::new();
    //     assert_eq!(Ok(Integer(10)), rt.eval(Integer(10)));
    // }

    // #[test]
    // fn test_eval_float() {
    //     let mut rt = Runtime::new();
    //     assert_eq!(Ok(Float(1.0)), rt.eval(Float(1.0)));
    // }

    // #[test]
    // fn test_eval_string() {
    //     let mut rt = Runtime::new();
    //     assert_eq!(Ok(SExp::string("foo")), rt.eval(SExp::string("foo")));
    // }
}
