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
    pub fn read(name: String) -> Result<BuiltIn, Error> {
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
    pub fn read(sexp: SExp) -> Result<Command, Error> {
        use dcpl::SExp::*;
        match sexp {
            List(exprs) => Ok(Command::ExecutableSequence(Command::read_ex_seq(exprs)?)),
            Integer(val) => Ok(Command::Integer(val)),
            Symbol(name) => Ok(Command::BuiltIn(BuiltIn::read(name)?)),

            Float(_) => Err(Error::UsingFloat),
            String(_) => Err(Error::UsingString),
        }
    }

    fn read_ex_seq(exprs: impl IntoIterator<Item = SExp>) -> Result<Vec<Command>, Error> {
        exprs.into_iter().map(Command::read).collect()
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

    fn read_str(sexp_str: impl AsRef<str>) -> Result<Command, Error> {
        Command::read(SExpParser::parse_line(sexp_str).expect("unexpected parse error"))
    }

    #[test]
    fn test_read_executable_sequence() {
        assert_eq!(
            Ok(ExecutableSequence(vec![Integer(1), Integer(2), Integer(3)])),
            read_str("(1 2 3)")
        )
    }

    #[test]
    fn test_read_integer() {
        assert_eq!(Ok(Integer(10)), read_str("10"));
    }

    #[test]
    fn test_read_add() {
        assert_eq!(Ok(BuiltIn(Add)), read_str("add"));
    }

    #[test]
    fn test_read_div() {
        assert_eq!(Ok(BuiltIn(Div)), read_str("div"));
    }
    #[test]
    fn test_read_eq() {
        assert_eq!(Ok(BuiltIn(Eq)), read_str("eq"));
    }
    #[test]
    fn test_read_exec() {
        assert_eq!(Ok(BuiltIn(Exec)), read_str("exec"));
    }
    #[test]
    fn test_read_gt() {
        assert_eq!(Ok(BuiltIn(Gt)), read_str("gt"));
    }
    #[test]
    fn test_read_lt() {
        assert_eq!(Ok(BuiltIn(Lt)), read_str("lt"));
    }
    #[test]
    fn test_read_mul() {
        assert_eq!(Ok(BuiltIn(Mul)), read_str("mul"));
    }
    #[test]
    fn test_read_nget() {
        assert_eq!(Ok(BuiltIn(Nget)), read_str("nget"));
    }
    #[test]
    fn test_read_pop() {
        assert_eq!(Ok(BuiltIn(Pop)), read_str("pop"));
    }
    #[test]
    fn test_read_rem() {
        assert_eq!(Ok(BuiltIn(Rem)), read_str("rem"));
    }
    #[test]
    fn test_read_sel() {
        assert_eq!(Ok(BuiltIn(Sel)), read_str("sel"));
    }
    #[test]
    fn test_read_sub() {
        assert_eq!(Ok(BuiltIn(Sub)), read_str("sub"));
    }
    #[test]
    fn test_read_swap() {
        assert_eq!(Ok(BuiltIn(Swap)), read_str("swap"));
    }

    #[test]
    fn test_read_float() {
        assert_eq!(Err(Error::UsingFloat), read_str("10.0"));
    }

    #[test]
    fn test_read_string() {
        assert_eq!(Err(Error::UsingString), read_str("\"hello\""));
    }
}
