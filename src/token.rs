#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    INTEGER(i32),
    PLUS,
    MINUS,
    MUL,
    DIV,
    POW,
    LPAREN,
    RPAREN,
    EOF,
}
