use crate::token::Token;

pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(token: Token, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            token: token,
            children: children,
        }
    }
}
