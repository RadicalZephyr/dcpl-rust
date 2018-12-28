use dcpl::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("Postfix");
    interpreter.run();
}
