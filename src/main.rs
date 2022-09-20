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
    input: String, // TODO - move it out the state
    current_pos: i32,
    current_token: Option<Token>,
    current_char: Option<char>,
}

impl Interpreter {
    fn new(input: String) -> Interpreter {
        let mut interpreter = Interpreter {
            input: input,
            current_pos: -1,
            current_token: None,
            current_char: None,
        };
        interpreter.advance();

        interpreter
    }

    fn expr(&mut self) -> i32 {
        self.current_token = Some(self.get_next_token());
        let mut result = self.term();

        while self.current_token.clone().unwrap() == Token::PLUS || self.current_token.clone().unwrap() == Token::MINUS {
            match self.current_token.clone().unwrap() {
                Token::PLUS => {
                    self.eat(Token::PLUS);
                    result += self.term();
                },
                Token::MINUS => {
                    self.eat(Token::MINUS);
                    result -= self.term();
                },
                _ => panic!("Invalid syntax"),
            }
        }
        return result;
    }

    fn get_next_token(&mut self) -> Token {
        while self.current_char != None {
            if self.current_char.unwrap().is_whitespace() {
                self.advance();
                continue;
            }

            if self.current_char.unwrap().is_digit(10) {
                return Token::INTEGER(self.integer());
            }

            match self.current_char.unwrap() {
                '+' => {
                    self.advance();
                    return Token::PLUS;
                },
                '-' => {
                    self.advance();
                    return Token::MINUS;
                },
                _ => panic!("Invalid character"),
            }
        }

        Token::EOF
    }

    fn advance(&mut self) {
        self.current_pos += 1;
        if self.current_pos > self.input.len() as i32 - 1 {
            self.current_char = None;
        }
        else {
            self.current_char = Some(self.input.as_bytes()[self.current_pos as usize] as char);
        }
    }

        // TODO - transform this into string and avoid break statement
        fn integer(&mut self) -> i32 {
            let mut result = String::new();
            while let Some(current_char) = self.current_char {
                if current_char.is_digit(10) {
                    result.push(current_char);
                    self.advance();
                } else {
                    break;
                }
            }

            result.parse::<i32>().unwrap()
        }

    fn eat(&mut self, token: Token) {
        if token == self.current_token.clone().unwrap() {
            self.current_token = Some(self.get_next_token());
        }
        else {
            panic!("Invalid token");
        }
    }

    fn term(&mut self) -> i32 {
        match self.current_token.clone().unwrap() {
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i));
                return i;
            },
            _ => panic!("Invalid term"),
        }
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