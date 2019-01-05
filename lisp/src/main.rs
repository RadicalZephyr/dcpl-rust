use dcpl::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("L.I.S.P.", move |_expr| Some("".into()));
    interpreter.run();
}
