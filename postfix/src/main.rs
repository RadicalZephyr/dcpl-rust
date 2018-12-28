use dcpl::{Interpreter, SExp};

use std::collections::HashMap;

fn main() {
    let mut top_level = TopLevel::default();
    let mut interpreter = Interpreter::new("Postfix", move |expr| top_level.interpret(expr));
    interpreter.run();
}

struct Program {
    num_args: usize,
    commands: Vec<Command>,
}

impl Program {
    fn apply(&self, args: Vec<i128>) -> Result<i128, TopLevelError> {
        let num_args = args.len();
        if self.num_args != num_args {
            return Err(TopLevelError::WrongNumberOfArgs {
                expected: self.num_args,
                actual: num_args,
            });
        }

        Err(TopLevelError::Unknown)
    }
}

#[derive(Default)]
struct TopLevel {
    programs: HashMap<String, Program>,
}

impl TopLevel {
    fn interpret(&mut self, sexp: SExp) -> Option<String> {
        match sexp {
            SExp::List(exprs) => match TopLevelCommand::eval(exprs) {
                Ok(cmd) => match self.apply(cmd) {
                    Ok(result) => result,
                    Err(e) => Some(format!("error: {:?}", e)),
                },
                Err(e) => Some(format!("Error: {:?}", e)),
            },

            expr => Some(format!("{}", expr)),
        }
    }

    fn apply(&mut self, command: TopLevelCommand) -> Result<Option<String>, TopLevelError> {
        use self::TopLevelCommand::*;
        match command {
            Def {
                name,
                num_args,
                commands,
            } => {
                let program = Program { num_args, commands };
                self.programs.insert(name, program);
                Ok(None)
            }
            Call { name, args } => {
                let program = self
                    .programs
                    .get(&name)
                    .ok_or_else(|| TopLevelError::ProgramNotFound(name))?;
                let result = program.apply(args)?;
                Ok(Some(format!("{}", result)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TopLevelError {
    Unknown,
    IllegalArgumentType(SExp),
    NotASymbol,
    NotAnInteger,
    NotEnoughArgs(&'static str),
    ProgramNotFound(String),
    WrongNumberOfArgs { expected: usize, actual: usize },
    EvalError(Error),
}

impl From<Error> for TopLevelError {
    fn from(err: Error) -> TopLevelError {
        TopLevelError::EvalError(err)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TopLevelCommand {
    Def {
        name: String,
        num_args: usize,
        commands: Vec<Command>,
    },

    Call {
        name: String,
        args: Vec<i128>,
    },
}

impl TopLevelCommand {
    fn eval(exprs: impl IntoIterator<Item = SExp>) -> Result<TopLevelCommand, TopLevelError> {
        let mut exprs = exprs.into_iter();
        exprs
            .next()
            .ok_or_else(|| TopLevelError::NotEnoughArgs("()"))
            .and_then(|expr| expr.into_symbol().ok_or(TopLevelError::NotASymbol))
            .and_then(|name| TopLevelCommand::eval_symbol(&name, exprs))
    }

    fn eval_symbol(
        name: &str,
        rest: impl IntoIterator<Item = SExp>,
    ) -> Result<TopLevelCommand, TopLevelError> {
        if name == "def" {
            TopLevelCommand::def(rest)
        } else {
            TopLevelCommand::call(name, rest)
        }
    }

    fn def(exprs: impl IntoIterator<Item = SExp>) -> Result<TopLevelCommand, TopLevelError> {
        let mut exprs = exprs.into_iter();
        let name = exprs
            .next()
            .ok_or_else(|| TopLevelError::NotEnoughArgs("def"))?
            .into_symbol()
            .ok_or(TopLevelError::NotASymbol)?;
        let num_args = exprs
            .next()
            .ok_or_else(|| TopLevelError::NotEnoughArgs("def"))?
            .into_integer()
            .ok_or(TopLevelError::NotAnInteger)? as usize;
        let commands = exprs
            .map(Command::eval)
            .collect::<Result<Vec<Command>, Error>>()?;
        Ok(TopLevelCommand::Def {
            name,
            num_args,
            commands,
        })
    }

    fn call(
        name: &str,
        exprs: impl IntoIterator<Item = SExp>,
    ) -> Result<TopLevelCommand, TopLevelError> {
        let name = name.to_string();
        let mut args = vec![];
        for expr in exprs {
            match expr {
                SExp::Integer(value) => args.push(value),
                expr => return Err(TopLevelError::IllegalArgumentType(expr)),
            }
        }
        Ok(TopLevelCommand::Call { name, args })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum BuiltIn {
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
enum Command {
    ExecutableSequence(Vec<Command>),
    Integer(i128),
    BuiltIn(BuiltIn),
}

impl Command {
    fn eval(sexp: SExp) -> Result<Command, Error> {
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
enum Error {
    UnknownBuiltin(String),
    UsingFloat,
    UsingString,
}

#[cfg(test)]
mod test {
    use super::BuiltIn::*;
    use super::Command::*;
    use super::TopLevelCommand::*;
    use super::*;

    use dcpl::SExpParser;

    fn eval_top_level(sexp_str: impl AsRef<str>) -> Result<TopLevelCommand, TopLevelError> {
        let sexp = SExpParser::parse_line(sexp_str).expect("unexpected parse error");
        match sexp {
            SExp::List(exprs) => TopLevelCommand::eval(exprs),
            _ => panic!("must pass a list to `eval_top_level`"),
        }
    }

    fn eval_str(sexp_str: impl AsRef<str>) -> Result<Command, Error> {
        Command::eval(SExpParser::parse_line(sexp_str).expect("unexpected parse error"))
    }

    #[test]
    fn test_top_level_eval_def() {
        let expected = Def {
            name: "foo".into(),
            num_args: 2,
            commands: vec![Integer(4), Integer(7), BuiltIn(Sub)],
        };
        assert_eq!(Ok(expected), eval_top_level("(def foo 2 4 7 sub)"))
    }

    #[test]
    fn test_top_level_eval_call() {
        let expected = Call {
            name: "bar".into(),
            args: vec![1, 2],
        };
        assert_eq!(Ok(expected), eval_top_level("(bar 1 2)"))
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
