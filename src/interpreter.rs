use crate::ast::{ASTNode, ASTNodeType};
use crate::syntax_analyzer::Parser;
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
        let lhs = self.visit(&node.children[0]);
        let rhs = self.visit(&node.children[1]);

        match node.token {
            Token::PLUS => {
                return lhs + rhs;
            }
            Token::MINUS => {
                return lhs - rhs;
            }
            Token::MUL => {
                return lhs * rhs;
            }
            Token::DIV => {
                return lhs / rhs;
            }
            Token::POW => {
                return self.exp_by_squaring(lhs, rhs);
            }
            _ => panic!("Visitor unmatch"),
        }
    }

    // TODO - typing for 1 / x
    fn exp_by_squaring(&self, x: i32, n: i32) -> i32 {
        if n < 0 {
            return self.exp_by_squaring(1 / x, -n);
        }
        else if n == 0 {
            return 1;
        }
        else if n == 1 {
            return x;
        }
        else if n % 2 == 0 {
            return self.exp_by_squaring(x * x,  n / 2);
        }
        else if n % 2 == 1 {
            return x * self.exp_by_squaring(x * x, (n - 1) / 2);
        }
        else {
            panic!("Invalid arguments");
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
    fn test_complex_expression_with_parenthesis_1() {
        let text = String::from("(3*     -(4 - 1) + 6)    *   2");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), -6);
    }

    #[test]
    fn test_complex_expression_with_parenthesis_2() {
        let text = String::from("3          + 2^4 - (2*3)   ^ 2");
        let mut interpreter = Interpreter::new(&text);
        assert_eq!(interpreter.interpret(), -17);
    }
}
