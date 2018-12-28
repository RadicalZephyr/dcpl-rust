use dcpl::Interpreter;

fn main() {
    println!("Welcome to the Postfix interpreter!");
    let interpreter = Interpreter::new("postfix>");
    interpreter.run();
}
