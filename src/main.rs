use std::io;
use std::io::Write;

mod interpreter;
mod lexer;

fn get_input() -> String {
    let mut input = String::new();

    let _ = io::stdout().write(b">>> ");
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut input).unwrap();

    String::from(input.trim())
}

fn main() {
    loop {
        let text = get_input();
        let lexer = lexer::Lexer::new(text.clone());
        let mut interpreter = interpreter::Interpreter::new(lexer);
        let result = interpreter.expr();
        println!("\n{} = {}\n", text, result);
    }
}
