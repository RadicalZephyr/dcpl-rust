use crate::{List, Symbol, Value};
use dcpl::SExp;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BeginError,
    EPrognError,
    IfError,
    UndefinedSymbol,
    QuoteError,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq)]
struct Env {}

impl Env {
    fn lookup(&self, _name: Symbol) -> Option<Value> {
        println!("failed to look up: '{:?}'", _name);
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
            let list = expr.into_list().unwrap();
            if let Some(sym) = list.first().cloned() {
                if let Some(symbol) = sym.into_symbol() {
                    match symbol.0.as_ref() {
                        "quote" => list.second().cloned().ok_or(Error::QuoteError),
                        "if" => {
                            let condition = list.nth(1).ok_or(Error::IfError)?;
                            let consequent = list.nth(2).ok_or(Error::IfError)?;
                            let alternate = list.nth(3).ok_or(Error::IfError)?;

                            let cond_res = self.eval(condition.clone())?;

                            if cond_res.is_truthy() {
                                self.eval(consequent.clone())
                            } else {
                                self.eval(alternate.clone())
                            }
                        }
                        "begin" => {
                            let rest = list.rest().ok_or(Error::BeginError)?;
                            self.eprogn(rest)
                        }
                        _ => Err(Error::NotImplemented),
                    }
                } else {
                    Err(Error::NotImplemented)
                }
            } else {
                Err(Error::NotImplemented)
            }
        }
    }

    pub fn eprogn(&mut self, mut exprs: &Value) -> Result<Value, Error> {
        let mut last = Value::List(List::Nil);
        while exprs.is_list() && exprs.as_list().unwrap().is_pair() {
            let cell = exprs.as_list().unwrap();
            last = self.eval(cell.first().cloned().unwrap())?;
            exprs = cell.rest().unwrap();
        }
        Ok(last)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_integer() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::integer(10)), rt.eval(Value::integer(10)));
    }

    #[test]
    fn test_eval_float() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::double(1.0)), rt.eval(Value::double(1.0)));
    }

    #[test]
    fn test_eval_string() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::string("foo")), rt.eval(Value::string("foo")));
    }

    macro_rules! lisp {
        { $e:expr } => {
            dcpl::SExpParser::parse_line($e).expect("unexpected parse failure").into()
        }
    }

    #[test]
    fn test_eval_bool() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::bool(true)), rt.eval(lisp!("true")));
    }

    #[test]
    fn test_eval_quote() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::bool(true)), rt.eval(lisp!("(quote true)")));
    }

    #[test]
    fn test_eval_if() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::integer(1)), rt.eval(lisp!("(if true 1 2)")));
    }

    #[test]
    fn test_eval_begin() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::integer(100)), rt.eval(lisp!("(begin 100)")));
    }

    #[test]
    fn test_eval_begin_multiple() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Value::integer(100)), rt.eval(lisp!("(begin 2 3 100)")));
    }
}
