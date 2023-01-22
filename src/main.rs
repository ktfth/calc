mod interpreter;

use interpreter::interpret;

fn main() {
    let expr = &std::env::args().nth(1).unwrap();
    let comparison_label = &std::env::args().nth(2).unwrap_or("=".into());
    let result = interpret(expr);
    println!("{} {} {}", expr, comparison_label, result);
}
