use std::iter::FromIterator;

use crate::read::{BuiltIn, Command};
use crate::top_level::Error as TopLevelError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
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
    pub fn into_integer(self) -> Result<i128, Error> {
        match self {
            StackValue::Integer(value) => Ok(value),
            _ => Err(Error::NotANumber),
        }
    }
}

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

#[derive(Debug, PartialEq)]
struct Stack(Vec<StackValue>);

impl Stack {
    pub fn pop(&mut self) -> Result<StackValue, Error> {
        self.0.pop().ok_or(Error::NotEnoughValues)
    }

    pub fn push(&mut self, value: StackValue) {
        self.0.push(value);
    }

    pub fn swap(mut self) -> Result<Stack, Error> {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        self.push(v1);
        self.push(v2);
        Ok(self)
    }
}

impl FromIterator<StackValue> for Stack {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = StackValue>,
    {
        Stack(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    num_args: usize,
    commands: Vec<Command>,
}

macro_rules! arith_op {
    { $stack:ident, $op:tt } => {{
        let v1 = $stack
            .pop()?
            .into_integer()?;
        let v2 = $stack
            .pop()?
            .into_integer()?;

        $stack.push(StackValue::Integer(v2 $op v1));
        Ok($stack)
    }};
}

macro_rules! bool_op {
    { $stack:ident, $op:tt } => {{
        let v1 = $stack
            .pop()?
            .into_integer()?;
        let v2 = $stack
            .pop()?
            .into_integer()?;

        $stack.push(StackValue::Integer(if v2 $op v1 {1} else {0}));
        Ok($stack)
    }};
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
        match final_stack.pop()? {
            StackValue::Integer(value) => Ok(value),
            StackValue::ExecutableSequence(_) => {
                Err(TopLevelError::from(Error::FinalValueNotAnInteger))
            }
        }
    }

    fn apply_command(mut stack: Stack, command: &Command) -> Result<Stack, Error> {
        use crate::read::Command::*;
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
        use crate::read::BuiltIn::*;
        match builtin {
            Add => arith_op!(stack, +),
            Sub => arith_op!(stack, -),
            Mul => arith_op!(stack, *),
            Div => arith_op!(stack, /),
            Rem => arith_op!(stack, %),
            Eq => bool_op!(stack, ==),
            Gt => bool_op!(stack, >),
            Lt => bool_op!(stack, <),
            Pop => {
                stack.pop()?;
                Ok(stack)
            }
            Swap => stack.swap(),
            Sel => {
                let v1 = stack.pop()?;
                let v2 = stack.pop()?;
                let v3 = stack.pop()?;
                if v3.into_integer()? == 0 {
                    stack.push(v1);
                } else {
                    stack.push(v2);
                }
                Ok(stack)
            }
            Exec => Ok(stack),
            Nget => Ok(stack),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! stack {
        { $($val:expr),* }=> {{
            let v: Vec<i128> = vec![ $($val),* ];
            v.into_iter().map(StackValue::from).collect::<Stack>()
        }}
    }

    macro_rules! arith_op_test {
        { $name:ident : $operator:expr => [ $($stack_val:expr),* ] == $expected:expr } => {
            #[test]
            fn $name() {
                assert_eq!(Ok(stack![$expected]), Program::apply_builtin(stack![ $($stack_val),* ], &$operator));
            }
        }
    }

    macro_rules! boolean {
        (true) => {
            stack![1]
        };
        (false) => {
            stack![0]
        };
    }

    macro_rules! bool_op_test {
        { $name:ident : $operator:expr => [ $($stack_val:expr),* ] -> $expected:tt } => {
            #[test]
            fn $name() {
                assert_eq!(Ok(boolean!($expected)), Program::apply_builtin(stack![ $($stack_val),* ], &$operator));
            }
        };
    }

    arith_op_test!(test_add: BuiltIn::Add => [2, 1] == 3);
    arith_op_test!(test_sub: BuiltIn::Sub => [2, 1] == 1);
    arith_op_test!(test_mul: BuiltIn::Mul => [2, 3] == 6);
    arith_op_test!(test_div: BuiltIn::Div => [6, 2] == 3);

    bool_op_test!(test_eq: BuiltIn::Eq => [1, 1] -> true);
    bool_op_test!(test_not_eq: BuiltIn::Eq => [1, 2] -> false);

    bool_op_test!(test_gt: BuiltIn::Gt => [2, 1] -> true);
    bool_op_test!(test_not_gt: BuiltIn::Gt => [1, 2] -> false);

    bool_op_test!(test_lt: BuiltIn::Lt => [1, 2] -> true);
    bool_op_test!(test_not_lt: BuiltIn::Lt => [2, 1] -> false);

    #[test]
    fn test_pop_empty() {
        assert_eq!(
            Err(Error::NotEnoughValues),
            Program::apply_builtin(stack![], &BuiltIn::Pop)
        );
    }

    #[test]
    fn test_pop() {
        assert_eq!(
            Ok(stack![1]),
            Program::apply_builtin(stack![1, 2], &BuiltIn::Pop)
        );
    }

    #[test]
    fn test_swap_empty() {
        assert_eq!(
            Err(Error::NotEnoughValues),
            Program::apply_builtin(stack![], &BuiltIn::Swap)
        );
    }

    #[test]
    fn test_swap_one() {
        assert_eq!(
            Err(Error::NotEnoughValues),
            Program::apply_builtin(stack![1], &BuiltIn::Swap)
        );
    }

    #[test]
    fn test_swap() {
        assert_eq!(
            Ok(stack![1, 9]),
            Program::apply_builtin(stack![9, 1], &BuiltIn::Swap)
        )
    }

    #[test]
    fn test_sel_then() {
        assert_eq!(
            Ok(stack![3]),
            Program::apply_builtin(stack![0, 2, 3], &BuiltIn::Sel)
        )
    }

    #[test]
    fn test_sel_else() {
        assert_eq!(
            Ok(stack![2]),
            Program::apply_builtin(stack![1, 2, 3], &BuiltIn::Sel)
        )
    }
}
