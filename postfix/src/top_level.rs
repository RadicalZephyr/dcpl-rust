use std::collections::HashMap;

use dcpl::SExp;

use crate::program::{Error as ProgramError, Program};
use crate::read::{BuiltIn, Command, Error as ParseError};

pub struct TopLevel {
    programs: HashMap<String, Program>,
}

macro_rules! builtin_program {
    { $programs:ident[$name:expr] = $builtin:path : $arg_count:expr } => {
        $programs.insert(
            $name.into(),
            Program::new($arg_count, vec![Command::BuiltIn($builtin)]),
        );
    };
}

impl TopLevel {
    pub fn new() -> TopLevel {
        let mut programs = HashMap::new();
        builtin_program!(programs["add"] = BuiltIn::Add : 2);
        builtin_program!(programs["sub"] = BuiltIn::Sub : 2);
        builtin_program!(programs["mul"] = BuiltIn::Mul : 2);
        builtin_program!(programs["div"] = BuiltIn::Div : 2);
        builtin_program!(programs["eq"] = BuiltIn::Eq : 2);
        builtin_program!(programs["lt"] = BuiltIn::Lt : 2);
        builtin_program!(programs["gt"] = BuiltIn::Gt : 2);
        TopLevel { programs }
    }

    pub fn interpret(&mut self, sexp: SExp) -> Option<String> {
        match sexp {
            SExp::List(exprs) => match TopLevelCommand::read(exprs) {
                Ok(cmd) => match self.apply(cmd) {
                    Ok(result) => result,
                    Err(e) => Some(format!("error: {:?}", e)),
                },
                Err(e) => Some(format!("Error: {:?}", e)),
            },

            expr => Some(format!("{}", expr)),
        }
    }

    fn apply(&mut self, command: TopLevelCommand) -> Result<Option<String>, Error> {
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
                    .ok_or_else(|| Error::ProgramNotFound(name))?;
                let result = program.apply(args)?;
                Ok(Some(format!("{}", result)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    IllegalArgumentType(SExp),
    NotASymbol,
    NotAnInteger,
    NotEnoughArgs(&'static str),
    ProgramNotFound(String),
    WrongNumberOfArgs { expected: usize, actual: usize },
    ReadError(ParseError),
    ProgramError(ProgramError),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::ReadError(err)
    }
}

impl From<ProgramError> for Error {
    fn from(err: ProgramError) -> Error {
        Error::ProgramError(err)
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
    fn read(exprs: impl IntoIterator<Item = SExp>) -> Result<TopLevelCommand, Error> {
        let mut exprs = exprs.into_iter();
        exprs
            .next()
            .ok_or_else(|| Error::NotEnoughArgs("()"))
            .and_then(|expr| expr.into_symbol().ok_or(Error::NotASymbol))
            .and_then(|name| TopLevelCommand::read_symbol(&name, exprs))
    }

    fn read_symbol(
        name: &str,
        rest: impl IntoIterator<Item = SExp>,
    ) -> Result<TopLevelCommand, Error> {
        if name == "def" {
            TopLevelCommand::def(rest)
        } else {
            TopLevelCommand::call(name, rest)
        }
    }

    fn def(exprs: impl IntoIterator<Item = SExp>) -> Result<TopLevelCommand, Error> {
        let mut exprs = exprs.into_iter();
        let name = exprs
            .next()
            .ok_or_else(|| Error::NotEnoughArgs("def"))?
            .into_symbol()
            .ok_or(Error::NotASymbol)?;
        let num_args = exprs
            .next()
            .ok_or_else(|| Error::NotEnoughArgs("def"))?
            .into_integer()
            .ok_or(Error::NotAnInteger)? as usize;
        let commands = exprs
            .map(Command::read)
            .collect::<Result<Vec<Command>, ParseError>>()?;
        Ok(TopLevelCommand::Def {
            name,
            num_args,
            commands,
        })
    }

    fn call(name: &str, exprs: impl IntoIterator<Item = SExp>) -> Result<TopLevelCommand, Error> {
        let name = name.to_string();
        let mut args = vec![];
        for expr in exprs {
            match expr {
                SExp::Integer(value) => args.push(value),
                expr => return Err(Error::IllegalArgumentType(expr)),
            }
        }
        Ok(TopLevelCommand::Call { name, args })
    }
}

#[cfg(test)]
mod test {
    use super::TopLevelCommand::*;
    use super::*;

    use crate::read::BuiltIn::*;
    use crate::read::Command::*;

    use dcpl::SExpParser;

    fn read_top_level(sexp_str: impl AsRef<str>) -> Result<TopLevelCommand, Error> {
        let sexp = SExpParser::parse_line(sexp_str).expect("unexpected parse error");
        match sexp {
            SExp::List(exprs) => TopLevelCommand::read(exprs),
            _ => panic!("must pass a list to `read_top_level`"),
        }
    }

    #[test]
    fn test_top_level_read_def() {
        let expected = Def {
            name: "foo".into(),
            num_args: 2,
            commands: vec![Integer(4), Integer(7), BuiltIn(Sub)],
        };
        assert_eq!(Ok(expected), read_top_level("(def foo 2 4 7 sub)"))
    }

    #[test]
    fn test_top_level_read_call() {
        let expected = Call {
            name: "bar".into(),
            args: vec![1, 2],
        };
        assert_eq!(Ok(expected), read_top_level("(bar 1 2)"))
    }
}
