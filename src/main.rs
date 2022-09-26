use std::io;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    INTEGER(i32),
    PLUS,
    MINUS,
    MUL,
    DIV,
    EOF,
}

pub struct Lexer {
    input: String,
    current_pos: i32,
    current_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            current_pos: -1,
            current_char: None,
        };
        lexer.advance();

        lexer
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
                }
                '-' => {
                    self.advance();
                    return Token::MINUS;
                }
                '*' => {
                    self.advance();
                    return Token::MUL;
                }
                '/' => {
                    self.advance();
                    return Token::DIV;
                }
                _ => panic!("Invalid character"),
            }
        }

        Token::EOF
    }

    fn advance(&mut self) {
        self.current_pos += 1;
        if self.current_pos > self.input.len() as i32 - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.as_bytes()[self.current_pos as usize] as char);
        }
    }

    // TODO - refactor this method
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
}

pub struct Interpreter {
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    fn new(lexer: Lexer) ->Interpreter  {
        let mut interpreter = Interpreter {
            lexer: lexer,
            current_token: Token::EOF,
        };
        interpreter.current_token = interpreter.lexer.get_next_token();

        interpreter
    }

    fn expr(&mut self) -> i32 {
        let mut result = self.term();

        while self.current_token.clone() == Token::PLUS
            || self.current_token.clone() == Token::MINUS
        {
            match self.current_token.clone() {
                Token::PLUS => {
                    self.eat(Token::PLUS);
                    result += self.term();
                }
                Token::MINUS => {
                    self.eat(Token::MINUS);
                    result -= self.term();
                }
                _ => panic!("Invalid syntax"),
            }
        }

        result
    }

    fn term(&mut self) -> i32 {
        let mut result = self.factor();

        while self.current_token.clone() == Token::MUL
            || self.current_token.clone() == Token::DIV
        {
            match self.current_token.clone() {
                Token::MUL => {
                    self.eat(Token::MUL);
                    result *= self.term();
                }
                Token::DIV => {
                    self.eat(Token::DIV);
                    result /= self.term();
                }
                _ => panic!("Invalid syntax"),
            }
        }

        result
    }

    fn factor(&mut self) -> i32 {
        match self.current_token.clone() {
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i));
                return i;
            }
            _ => panic!("Invalid factor"),
        }
    }

    fn eat(&mut self, token: Token) {
        if token == self.current_token.clone() {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Invalid syntax");
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
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        let result = interpreter.expr();
        println!("{}", result);
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::Interpreter;

    #[test]
    fn test_basic_addition() {
        let text = String::from("5+9");
        let mut interpreter = Interpreter::new(text);
        assert_eq!(interpreter.expr(), 14);
    }

    #[test]
    fn test_basic_substraction_with_multiple_whitespaces() {
        let text = String::from("   5              -   3");
        let mut interpreter = Interpreter::new(text);
        assert_eq!(interpreter.expr(), 2);
    }

    #[test]
    fn test_mutiple_additions_and_substractions() {
        let text = String::from("5 + 9 - 3 - 1 + 2 -4");
        let mut interpreter = Interpreter::new(text);
        assert_eq!(interpreter.expr(), 8);
    }
}
*/
