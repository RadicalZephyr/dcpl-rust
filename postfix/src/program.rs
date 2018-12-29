use crate::parse::Command;
use crate::top_level::TopLevelError;

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

        Err(TopLevelError::Unknown)
    }
}
