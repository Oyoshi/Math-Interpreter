use std::io;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    INTEGER(i32),
    PLUS,
    MINUS,
    EOF,
}

pub struct Interpreter {
    input: String,
    current_pos: i32,
    current_token: Option<Token>,
}

impl Interpreter {
    fn new(input: String) -> Interpreter {
        let interpreter = Interpreter {
            input: input,
            current_pos: 0,
            current_token: None,
        };
        interpreter
    }

    fn get_next_token(&mut self) -> Token {
        if self.current_pos > self.input.len() as i32 - 1 {
            return Token::EOF
        }

        let current_char = self.input.as_bytes()[self.current_pos as usize] as char;

        if current_char.is_digit(10) {
            self.current_pos += 1;
            return Token::INTEGER(current_char.to_digit(10).unwrap() as i32);
        }

        if current_char == '+' {
            self.current_pos += 1;
            return Token::PLUS;
        }

        if current_char == '-' {
            self.current_pos += 1;
            return Token::MINUS;
        }

        // panic or error
        Token::EOF

    }

    fn eat(&mut self, token: Token) {
        if token == self.current_token.clone().unwrap() {
            self.current_token = Some(self.get_next_token());
        }
        // else error
    }

    fn expr(&mut self) -> i32 {
        let mut result: i32;
        self.current_token = Some(self.get_next_token());
        let lhs = self.current_token.clone().unwrap();
        match lhs {
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i));
                result = i;
            },
            _ => panic!("Invalid syntax"),
        }
        let op = self.current_token.clone().unwrap();
        if op == Token::PLUS {
            self.eat(Token::PLUS);
        }
        else {
            self.eat(Token::MINUS);
        }
        let rhs = self.current_token.clone().unwrap();
        match rhs {
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i));
                if op == Token::PLUS {
                    result += i;
                }
                else {
                    result -= i;
                }
            },
            _ => panic!("Invalid syntax"),
        }
        return result;
    }
}

fn main() {
    loop {
        let mut input = String::new();

        let _ = io::stdout().write(b">>> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input).unwrap();

        let text = String::from(input.trim());
        let mut interpreter = Interpreter::new(text);
        let result = interpreter.expr();
        println!("{}", result);
    }
}

#[test]
fn test_interpreter() {
    let text = String::from("5-9");
    let mut interpreter = Interpreter::new(text);
    assert_eq!(interpreter.expr(), -4);
}