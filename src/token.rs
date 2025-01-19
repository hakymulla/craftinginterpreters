use crate::scanner::{TokenType, Literals};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal: Literals,
    pub line: usize, 
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: String, literal: Literals, line: usize) -> Self {
        Self { tokentype, lexeme, literal, line }
    }

    pub fn toString(&self) -> String {
        format!("{:?} {:?} {:?}", self.tokentype, self.lexeme, self.literal)
    }
}