use dcpl::Interpreter;

use lisp::Runtime;

fn main() {
    let mut runtime = Runtime::new();
    let mut interpreter = Interpreter::new("L.I.S.P.", move |expr| runtime.rep_iter(expr));
    interpreter.run();
}
