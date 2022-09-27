use crate::token::Token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTNodeType {
    INTEGER,
    BINOP,
    UNOP,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub token: Token,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node_type: ASTNodeType, token: Token, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            node_type: node_type,
            token: token,
            children: children,
        }
    }
}
