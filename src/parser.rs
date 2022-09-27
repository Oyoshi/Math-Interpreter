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
            _ => panic!("Invalid syntax"),
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
