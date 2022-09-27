use crate::ast::ASTNode;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &String) -> Parser {
        let mut parser = Parser {
            lexer: Lexer::new(&input),
            current_token: Token::EOF,
        };
        parser.current_token = parser.lexer.get_next_token();

        parser
    }

    pub fn parse(&mut self) -> ASTNode {
        return self.expr();
    }

    fn expr(&mut self) -> ASTNode {
        let mut node = self.term();

        while self.current_token == Token::PLUS || self.current_token == Token::MINUS {
            match self.current_token {
                Token::PLUS => {
                    self.eat(Token::PLUS);
                    let children: Vec<ASTNode> = vec![node, self.term()];
                    node = ASTNode::new(Token::PLUS, children);
                }
                Token::MINUS => {
                    self.eat(Token::MINUS);
                    let children: Vec<ASTNode> = vec![node, self.term()];
                    node = ASTNode::new(Token::MINUS, children);
                }
                _ => panic!("Invalid syntax"),
            }
        }

        if node.token == Token::EOF {
            node = ASTNode::new(Token::INTEGER(0), vec![]);
        }

        node
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();

        while self.current_token == Token::MUL || self.current_token == Token::DIV {
            match self.current_token {
                Token::MUL => {
                    self.eat(Token::MUL);
                    let children: Vec<ASTNode> = vec![node, self.factor()];
                    node = ASTNode::new(Token::MUL, children);
                }
                Token::DIV => {
                    self.eat(Token::DIV);
                    let children: Vec<ASTNode> = vec![node, self.factor()];
                    node = ASTNode::new(Token::DIV, children);
                }
                _ => panic!("Invalid syntax"),
            }
        }

        node
    }

    fn factor(&mut self) -> ASTNode {
        match self.current_token {
            Token::INTEGER(i) => {
                let token = self.current_token.clone();
                self.eat(Token::INTEGER(i));
                return ASTNode::new(token, vec![]);
            }
            Token::LPAREN => {
                self.eat(Token::LPAREN);
                let node = self.expr();
                self.eat(Token::RPAREN);
                return node;
            }
            Token::EOF => {
                return ASTNode::new(Token::EOF, vec![]);
            }
            _ => panic!("Invalid syntax"),
        }
    }

    fn eat(&mut self, token: Token) {
        if token == self.current_token {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Invalid token");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ASTNode;
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_empty_string() {
        let text = String::from("");
        let mut parser = Parser::new(&text);
        assert_eq!(
            parser.parse(),
            ASTNode {
                token: Token::INTEGER(0),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_integer() {
        let text = String::from("2137");
        let mut parser = Parser::new(&text);
        assert_eq!(
            parser.parse(),
            ASTNode {
                token: Token::INTEGER(2137),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_binary_operator() {
        let text = String::from("2  + 3");
        let mut parser = Parser::new(&text);
        assert_eq!(
            parser.parse(),
            ASTNode {
                token: Token::PLUS,
                children: vec![
                    ASTNode {
                        token: Token::INTEGER(2),
                        children: vec![]
                    },
                    ASTNode {
                        token: Token::INTEGER(3),
                        children: vec![]
                    }
                ]
            }
        );
    }
}
