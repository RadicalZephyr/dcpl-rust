use std::iter::FromIterator;

use crate::parse::{BuiltIn, Command};
use crate::top_level::Error as TopLevelError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    EmptyFinalStack,
    FinalValueNotAnInteger,
    NotEnoughValues,
    NotANumber,
}

#[derive(Clone, Debug, PartialEq)]
enum StackValue {
    ExecutableSequence(Vec<Command>),
    Integer(i128),
}

impl StackValue {
    pub fn into_integer(self) -> Option<i128> {
        match self {
            StackValue::Integer(value) => Some(value),
            _ => None,
        }
    }
}

type Stack = Vec<StackValue>;

impl FromIterator<Command> for StackValue {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Command>,
    {
        StackValue::ExecutableSequence(iter.into_iter().collect())
    }
}

impl From<Vec<Command>> for StackValue {
    fn from(commands: Vec<Command>) -> StackValue {
        StackValue::ExecutableSequence(commands)
    }
}

impl From<i128> for StackValue {
    fn from(number: i128) -> StackValue {
        StackValue::Integer(number)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    num_args: usize,
    commands: Vec<Command>,
}

impl Program {
    pub fn new(num_args: usize, commands: Vec<Command>) -> Program {
        Program { num_args, commands }
    }

    pub fn apply(&self, args: Vec<i128>) -> Result<i128, TopLevelError> {
        let num_args = args.len();
        if self.num_args != num_args {
            return Err(TopLevelError::WrongNumberOfArgs {
                expected: self.num_args,
                actual: num_args,
            });
        }
        let stack: Stack = args.into_iter().rev().map(StackValue::from).collect();
        let mut final_stack = self
            .commands
            .iter()
            .try_fold(stack, Program::apply_command)?;
        match final_stack.pop() {
            Some(StackValue::Integer(value)) => Ok(value),
            Some(StackValue::ExecutableSequence(_)) => {
                Err(TopLevelError::from(Error::FinalValueNotAnInteger))
            }
            None => Err(TopLevelError::from(Error::EmptyFinalStack)),
        }
    }

    fn apply_command(mut stack: Stack, command: &Command) -> Result<Stack, Error> {
        use crate::parse::Command::*;
        match command {
            Integer(inner) => {
                stack.push(StackValue::from(*inner));
                Ok(stack)
            }
            ExecutableSequence(inner) => {
                stack.push(inner.iter().cloned().collect());
                Ok(stack)
            }
            BuiltIn(builtin) => Program::apply_builtin(stack, builtin),
        }
    }

    fn apply_builtin(mut stack: Stack, builtin: &BuiltIn) -> Result<Stack, Error> {
        use crate::parse::BuiltIn::*;
        match builtin {
            Add => {
                let v1 = stack
                    .pop()
                    .ok_or(Error::NotEnoughValues)?
                    .into_integer()
                    .ok_or(Error::NotANumber)?;
                let v2 = stack
                    .pop()
                    .ok_or(Error::NotEnoughValues)?
                    .into_integer()
                    .ok_or(Error::NotANumber)?;

                stack.push(StackValue::Integer(v2 + v1));
                Ok(stack)
            }
            Div => Ok(stack),
            Eq => Ok(stack),
            Exec => Ok(stack),
            Gt => Ok(stack),
            Lt => Ok(stack),
            Mul => Ok(stack),
            Nget => Ok(stack),
            Pop => Ok(stack),
            Rem => Ok(stack),
            Sel => Ok(stack),
            Sub => Ok(stack),
            Swap => Ok(stack),
        }
    }
}
