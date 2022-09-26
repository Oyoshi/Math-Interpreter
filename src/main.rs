use std::io;
use std::io::Write;

mod interpreter;
mod lexer;

fn main() {
    loop {
        let mut input = String::new();

        let _ = io::stdout().write(b">>> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input).unwrap();

        let text = String::from(input.trim());
        let lexer = lexer::Lexer::new(text);
        let mut interpreter = interpreter::Interpreter::new(lexer);
        let result = interpreter.expr();
        println!("{}", result);
    }
}
