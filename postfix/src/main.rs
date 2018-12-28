use dcpl::Interpreter;

fn main() {
    println!("Welcome to the Postfix interpreter!");
    let mut interpreter = Interpreter::new("postfix>", ">");
    interpreter.run();
}
