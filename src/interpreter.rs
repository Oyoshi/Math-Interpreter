use crate::lexer::Lexer;
use crate::lexer::Token;

pub struct Interpreter {
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    pub fn new(lexer: Lexer) -> Interpreter {
        let mut interpreter = Interpreter {
            lexer: lexer,
            current_token: Token::EOF,
        };
        interpreter.current_token = interpreter.lexer.get_next_token();

        interpreter
    }

    pub fn expr(&mut self) -> i32 {
        let mut result = self.term();

        while self.current_token == Token::PLUS || self.current_token == Token::MINUS {
            match self.current_token {
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

        while self.current_token == Token::MUL || self.current_token == Token::DIV {
            match self.current_token {
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
        match self.current_token {
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i));
                return i;
            }
            Token::LPAREN => {
                self.eat(Token::LPAREN);
                let result = self.expr();
                self.eat(Token::RPAREN);
                return result;
            }
            _ => panic!("Invalid factor"),
        }
    }

    fn eat(&mut self, token: Token) {
        if token == self.current_token {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Invalid syntax");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::lexer::Lexer;

    #[test]
    fn test_basic_addition() {
        let text = String::from("5+9");
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        assert_eq!(interpreter.expr(), 14);
    }

    #[test]
    fn test_basic_substraction_with_multiple_whitespaces() {
        let text = String::from("   5              -   3");
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        assert_eq!(interpreter.expr(), 2);
    }

    #[test]
    fn test_mutiple_additions_and_substractions() {
        let text = String::from("5 + 9 - 3 - 1 + 2 -4");
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        assert_eq!(interpreter.expr(), 8);
    }

    #[test]
    fn test_complex_expression_with_parenthesis() {
        let text = String::from("(3*     (4 - 1) + 6)    *   2");
        let lexer = Lexer::new(text);
        let mut interpreter = Interpreter::new(lexer);
        assert_eq!(interpreter.expr(), 30);
    }
}
