use crate::ast::{ASTNode, ASTNodeType};
use crate::lexical_analyzer::Lexer;
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
        let mut node = self.pow();

        while self.current_token == Token::PLUS || self.current_token == Token::MINUS {
            match self.current_token {
                Token::PLUS => {
                    self.eat(Token::PLUS);
                    let children: Vec<ASTNode> = vec![node, self.pow()];
                    node = ASTNode::new(ASTNodeType::BINOP, Token::PLUS, children);
                }
                Token::MINUS => {
                    self.eat(Token::MINUS);
                    let children: Vec<ASTNode> = vec![node, self.pow()];
                    node = ASTNode::new(ASTNodeType::BINOP, Token::MINUS, children);
                }
                _ => panic!("Invalid syntax"),
            }
        }

        if node.token == Token::EOF {
            node = ASTNode::new(ASTNodeType::INTEGER, Token::INTEGER(0), vec![]);
        }

        node
    }

    fn pow(&mut self) -> ASTNode {
        let mut node = self.term();
        while self.current_token == Token::POW {
            let token = self.current_token.clone();
            self.eat(Token::POW);
            node = ASTNode::new(ASTNodeType::BINOP, token, vec![node, self.term()]);
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
                    node = ASTNode::new(ASTNodeType::BINOP, Token::MUL, children);
                }
                Token::DIV => {
                    self.eat(Token::DIV);
                    let children: Vec<ASTNode> = vec![node, self.factor()];
                    node = ASTNode::new(ASTNodeType::BINOP, Token::DIV, children);
                }
                _ => panic!("Invalid syntax"),
            }
        }

        node
    }

    fn factor(&mut self) -> ASTNode {
        match self.current_token {
            Token::PLUS => {
                self.eat(Token::PLUS);
                return ASTNode::new(ASTNodeType::UNOP, Token::PLUS, vec![self.factor()]);
            }
            Token::MINUS => {
                self.eat(Token::MINUS);
                return ASTNode::new(ASTNodeType::UNOP, Token::MINUS, vec![self.factor()]);
            }
            Token::INTEGER(i) => {
                let token = self.current_token.clone();
                self.eat(Token::INTEGER(i));
                return ASTNode::new(ASTNodeType::INTEGER, token, vec![]);
            }
            Token::LPAREN => {
                self.eat(Token::LPAREN);
                let node = self.expr();
                self.eat(Token::RPAREN);
                return node;
            }
            Token::EOF => {
                return ASTNode::new(ASTNodeType::INTEGER, Token::EOF, vec![]);
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
    use crate::ast::{ASTNode, ASTNodeType};
    use crate::syntax_analyzer::Parser;
    use crate::token::Token;

    #[test]
    fn test_empty_string() {
        let text = String::from("");
        let mut parser = Parser::new(&text);
        assert_eq!(
            parser.parse(),
            ASTNode {
                node_type: ASTNodeType::INTEGER,
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
                node_type: ASTNodeType::INTEGER,
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
                node_type: ASTNodeType::BINOP,
                token: Token::PLUS,
                children: vec![
                    ASTNode {
                        node_type: ASTNodeType::INTEGER,
                        token: Token::INTEGER(2),
                        children: vec![]
                    },
                    ASTNode {
                        node_type: ASTNodeType::INTEGER,
                        token: Token::INTEGER(3),
                        children: vec![]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_unary_operator() {
        let text = String::from("-3");
        let mut parser = Parser::new(&text);
        assert_eq!(
            parser.parse(),
            ASTNode {
                node_type: ASTNodeType::UNOP,
                token: Token::MINUS,
                children: vec![ASTNode {
                    node_type: ASTNodeType::INTEGER,
                    token: Token::INTEGER(3),
                    children: vec![]
                }]
            }
        );
    }
}
