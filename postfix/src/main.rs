use dcpl::{Interpreter, SExp};

fn main() {
    let mut interpreter = Interpreter::new("Postfix", interpret);
    interpreter.run();
}

fn interpret(sexp: SExp) -> Option<String> {
    None
}
