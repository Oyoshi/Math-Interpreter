use std::io;
use std::io::Write;

mod ast;
mod interpreter;
mod lexical_analyzer;
mod syntax_analyzer;
mod token;

use crate::interpreter::Interpreter;

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
        let mut interpreter = Interpreter::new(&text);
        let result = interpreter.interpret();
        println!(
            "\n{} = {}\n",
            if text == "" { String::from("0") } else { text },
            result
        );
    }
}
