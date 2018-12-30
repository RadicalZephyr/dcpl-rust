use std::iter::FromIterator;

use crate::parse::{BuiltIn, Command};
use crate::top_level::Error as TopLevelError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {}

#[derive(Clone, Debug, PartialEq)]
enum StackValue {
    ExecutableSequence(Vec<Command>),
    Integer(i128),
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
        Err(TopLevelError::Unknown)
    }

    fn apply_command(mut stack: Stack, command: Command) -> Result<Stack, Error> {
        match command {
            Command::Integer(inner) => {
                stack.push(StackValue::from(inner));
                Ok(stack)
            }
            Command::ExecutableSequence(inner) => {
                stack.push(StackValue::from(inner));
                Ok(stack)
            }
            Command::BuiltIn(builtin) => Program::apply_builtin(stack, builtin),
        }
    }

    fn apply_builtin(mut stack: Stack, builtin: BuiltIn) -> Result<Stack, Error> {
        Ok(stack)
    }
}
