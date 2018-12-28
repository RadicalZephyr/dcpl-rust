use dcpl::{Interpreter, SExp};

fn main() {
    let mut interpreter = Interpreter::new("Postfix", interpret);
    interpreter.run();
}

fn interpret(sexp: SExp) -> Option<String> {
    match sexp {
        SExp::List(exprs) => top_level_eval(exprs),
        expr => Some(format!("{}", expr)),
    }
}

fn top_level_eval(_sexp: Vec<SExp>) -> Option<String> {
    None
}

#[derive(Clone, Debug, PartialEq)]
enum Cmd {
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

impl Cmd {
    pub fn eval(name: String) -> Result<Cmd, Error> {
        use self::Cmd::*;
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

            _ => Err(Error::Unknown),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    ExecutableSequence(Vec<Command>),
    Integer(i128),
    BuiltIn(Cmd),
}

#[derive(Clone, Debug, PartialEq)]
enum Error {
    Unknown,
    UsingFloat,
    UsingString,
}

fn eval(sexp: SExp) -> Result<Command, Error> {
    use dcpl::SExp::*;
    match sexp {
        List(exprs) => Ok(Command::ExecutableSequence(eval_ex_seq(exprs)?)),
        Integer(val) => Ok(Command::Integer(val)),
        Symbol(name) => Ok(Command::BuiltIn(Cmd::eval(name)?)),

        Float(_) => Err(Error::UsingFloat),
        String(_) => Err(Error::UsingString),
    }
}

fn eval_ex_seq(exprs: Vec<SExp>) -> Result<Vec<Command>, Error> {
    let mut result = vec![];
    for expr in exprs {
        result.push(eval(expr)?);
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::Cmd::*;
    use super::Command::*;
    use super::*;

    use dcpl::SExpParser;

    fn eval_str(sexp_str: impl AsRef<str>) -> Result<Command, Error> {
        eval(SExpParser::parse_line(sexp_str).expect("unexpected parse error"))
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
