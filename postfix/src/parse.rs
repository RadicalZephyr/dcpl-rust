use dcpl::SExp;

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltIn {
    Add,
    Div,
    Eq,
    Exec,
    Gt,
    Lt,
    Mul,
    Nget,
    Pop,
    Rem,
    Sel,
    Sub,
    Swap,
}

impl BuiltIn {
    pub fn eval(name: String) -> Result<BuiltIn, Error> {
        use self::BuiltIn::*;
        match name.as_ref() {
            "add" => Ok(Add),
            "div" => Ok(Div),
            "eq" => Ok(Eq),
            "exec" => Ok(Exec),
            "gt" => Ok(Gt),
            "lt" => Ok(Lt),
            "mul" => Ok(Mul),
            "nget" => Ok(Nget),
            "pop" => Ok(Pop),
            "rem" => Ok(Rem),
            "sel" => Ok(Sel),
            "sub" => Ok(Sub),
            "swap" => Ok(Swap),

            _ => Err(Error::UnknownBuiltin(name)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    ExecutableSequence(Vec<Command>),
    Integer(i128),
    BuiltIn(BuiltIn),
}

impl Command {
    pub fn eval(sexp: SExp) -> Result<Command, Error> {
        use dcpl::SExp::*;
        match sexp {
            List(exprs) => Ok(Command::ExecutableSequence(Command::eval_ex_seq(exprs)?)),
            Integer(val) => Ok(Command::Integer(val)),
            Symbol(name) => Ok(Command::BuiltIn(BuiltIn::eval(name)?)),

            Float(_) => Err(Error::UsingFloat),
            String(_) => Err(Error::UsingString),
        }
    }

    fn eval_ex_seq(exprs: impl IntoIterator<Item = SExp>) -> Result<Vec<Command>, Error> {
        exprs.into_iter().map(Command::eval).collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    UnknownBuiltin(String),
    UsingFloat,
    UsingString,
}

#[cfg(test)]
mod test {
    use super::BuiltIn::*;
    use super::Command::*;
    use super::*;

    use dcpl::SExpParser;

    fn eval_str(sexp_str: impl AsRef<str>) -> Result<Command, Error> {
        Command::eval(SExpParser::parse_line(sexp_str).expect("unexpected parse error"))
    }

    #[test]
    fn test_eval_executable_sequence() {
        assert_eq!(
            Ok(ExecutableSequence(vec![Integer(1), Integer(2), Integer(3)])),
            eval_str("(1 2 3)")
        )
    }

    #[test]
    fn test_eval_integer() {
        assert_eq!(Ok(Integer(10)), eval_str("10"));
    }

    #[test]
    fn test_eval_add() {
        assert_eq!(Ok(BuiltIn(Add)), eval_str("add"));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(Ok(BuiltIn(Div)), eval_str("div"));
    }
    #[test]
    fn test_eval_eq() {
        assert_eq!(Ok(BuiltIn(Eq)), eval_str("eq"));
    }
    #[test]
    fn test_eval_exec() {
        assert_eq!(Ok(BuiltIn(Exec)), eval_str("exec"));
    }
    #[test]
    fn test_eval_gt() {
        assert_eq!(Ok(BuiltIn(Gt)), eval_str("gt"));
    }
    #[test]
    fn test_eval_lt() {
        assert_eq!(Ok(BuiltIn(Lt)), eval_str("lt"));
    }
    #[test]
    fn test_eval_mul() {
        assert_eq!(Ok(BuiltIn(Mul)), eval_str("mul"));
    }
    #[test]
    fn test_eval_nget() {
        assert_eq!(Ok(BuiltIn(Nget)), eval_str("nget"));
    }
    #[test]
    fn test_eval_pop() {
        assert_eq!(Ok(BuiltIn(Pop)), eval_str("pop"));
    }
    #[test]
    fn test_eval_rem() {
        assert_eq!(Ok(BuiltIn(Rem)), eval_str("rem"));
    }
    #[test]
    fn test_eval_sel() {
        assert_eq!(Ok(BuiltIn(Sel)), eval_str("sel"));
    }
    #[test]
    fn test_eval_sub() {
        assert_eq!(Ok(BuiltIn(Sub)), eval_str("sub"));
    }
    #[test]
    fn test_eval_swap() {
        assert_eq!(Ok(BuiltIn(Swap)), eval_str("swap"));
    }

    #[test]
    fn test_eval_float() {
        assert_eq!(Err(Error::UsingFloat), eval_str("10.0"));
    }

    #[test]
    fn test_eval_string() {
        assert_eq!(Err(Error::UsingString), eval_str("\"hello\""));
    }
}
