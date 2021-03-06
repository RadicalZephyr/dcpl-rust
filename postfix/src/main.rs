use dcpl::Interpreter;

mod program;
mod read;
mod top_level;

use crate::top_level::TopLevel;

fn main() {
    let mut top_level = TopLevel::new();
    let mut interpreter = Interpreter::new("Postfix", move |expr| top_level.interpret(expr));
    interpreter.run();
}
