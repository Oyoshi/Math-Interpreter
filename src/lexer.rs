#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
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
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            current_pos: -1,
            current_char: None,
        };
        lexer.advance();

        lexer
    }

    pub fn get_next_token(&mut self) -> Token {
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

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::lexer::Token;

    #[test]
    fn test_empty_token() {
        let text = String::from("");
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_integer_token() {
        let text = String::from("2137");
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.get_next_token(), Token::INTEGER(2137));
    }

    #[test]
    fn test_non_integer_token() {
        let text = String::from("+");
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.get_next_token(), Token::PLUS);
    }

    #[test]
    #[should_panic]
    fn test_token_does_not_exist() {
        let text = String::from("&");
        let mut lexer = Lexer::new(text);
        lexer.get_next_token();
    }
}
