use std::collections::HashMap;

use dcpl::SExp;

use crate::parse::{Command, Error};
use crate::program::Program;

#[derive(Default)]
pub struct TopLevel {
    programs: HashMap<String, Program>,
}

impl TopLevel {
    pub fn interpret(&mut self, sexp: SExp) -> Option<String> {
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
                let program = Program::new(num_args, commands);
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
pub enum TopLevelError {
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

#[cfg(test)]
mod test {
    use super::TopLevelCommand::*;
    use super::*;

    use crate::parse::BuiltIn::*;
    use crate::parse::Command::*;

    use dcpl::SExpParser;

    fn eval_top_level(sexp_str: impl AsRef<str>) -> Result<TopLevelCommand, TopLevelError> {
        let sexp = SExpParser::parse_line(sexp_str).expect("unexpected parse error");
        match sexp {
            SExp::List(exprs) => TopLevelCommand::eval(exprs),
            _ => panic!("must pass a list to `eval_top_level`"),
        }
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
}
