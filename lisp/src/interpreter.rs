use dcpl::SExp;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Undefined,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn rep_iter(&mut self, expr: SExp) -> Option<String> {
        match self.eval(expr) {
            Ok(expr) => Some(format!("{:?}", expr)),
            Err(error) => Some(format!("{:?}", error)),
        }
    }

    pub fn eval(&mut self, expr: SExp) -> Result<SExp, Error> {
        if expr.is_atom() {
            Ok(expr)
        } else {
            Err(Error::Undefined)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use dcpl::SExp;
    use dcpl::SExp::*;

    #[test]
    fn test_eval_integer() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Integer(10)), rt.eval(Integer(10)));
    }

    #[test]
    fn test_eval_float() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(Float(1.0)), rt.eval(Float(1.0)));
    }

    #[test]
    fn test_eval_string() {
        let mut rt = Runtime::new();
        assert_eq!(Ok(SExp::string("foo")), rt.eval(SExp::string("foo")));
    }
}
