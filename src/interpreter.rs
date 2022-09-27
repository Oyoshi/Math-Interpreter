use crate::ast::ASTNode;
use crate::ast::ASTNodeType;
use crate::parser::Parser;
use crate::token::Token;

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new(input: &String) -> Interpreter {
        let interpreter = Interpreter {
            parser: Parser::new(&input),
        };

        interpreter
    }

    pub fn interpret(&mut self) -> i32 {
        let tree = self.parser.parse();
        return self.visit(&tree);
    }

    fn visit(&self, node: &ASTNode) -> i32 {
        match node.node_type {
            ASTNodeType::INTEGER => {
                return self.visit_num(node);
            }
            ASTNodeType::BINOP => {
                return self.visit_binop(node);
            }
            ASTNodeType::UNOP => {
                return self.visit_unop(node);
            }
        }
    }

    fn visit_binop(&self, node: &ASTNode) -> i32 {
        let left_val = self.visit(&node.children[0]);
        let right_val = self.visit(&node.children[1]);

        match node.token {
            Token::PLUS => {
                return left_val + right_val;
            }
            Token::MINUS => {
                return left_val - right_val;
            }
            Token::MUL => {
                return left_val * right_val;
            }
            Token::DIV => {
                return left_val / right_val;
            }
            _ => panic!("Visitor unmatch"),
        }
    }

    fn visit_unop(&self, node: &ASTNode) -> i32 {
        let val = self.visit(&node.children[0]);

        match node.token {
            Token::PLUS => {
                return val;
            }
            Token::MINUS => {
                return -val;
            }
            _ => panic!("Visitor unmatch"),
        }
    }

    fn visit_num(&self, node: &ASTNode) -> i32 {
        match node.token {
            Token::INTEGER(i) => {
                return i;
            }
            _ => panic!("Visitor unmatch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;

    #[test]
    fn test_basic_addition() {
        let text = String::from("5+9");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), 14);
    }

    #[test]
    fn test_basic_substraction_with_multiple_whitespaces() {
        let text = String::from("   5              -   3");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), 2);
    }

    #[test]
    fn test_mutiple_additions_and_substractions() {
        let text = String::from("5 + 9 - 3 - 1 + 2 -4");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), 8);
    }

    #[test]
    fn test_complex_expression_with_parenthesis() {
        let text = String::from("(3*     -(4 - 1) + 6)    *   2");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), -6);
    }
}
