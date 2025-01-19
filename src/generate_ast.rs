use crate::{token::Token, scanner::*};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralsAst {
    Number(f64),
    Strings(String),
    True,
    False,
    Null
}

impl fmt::Display for LiteralsAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match on the enum variant and write the desired string representation
        let description = match self {
            LiteralsAst::Number(x) => x.to_string(),
            LiteralsAst::Strings(x) => x.to_string(),
            LiteralsAst::True => true.to_string(),
            LiteralsAst::False => false.to_string(),
            _ => "".to_string()
        };
        write!(f, "{}", description)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping {expression: Box<Expr>},
    Literal {value: LiteralsAst},
    Unary {operator: Token, right: Box<Expr>}
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                return format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string());
            },
            Expr::Grouping { expression } => {
                return format!("(group {})", expression.to_string());
            },
            Expr::Literal { value } => {
                return format!("{}", value.to_string());
            },
            Expr::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme, right.to_string());
            }
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast_print() {
        let expression = Box::new(Expr::Binary { 
            left: Box::new( Expr::Unary { 
                    operator: Token { tokentype: TokenType::Minus, lexeme: "-".to_string(), literal: Literals::Null, line: 1 }, 
                    right:Box::new( Expr::Literal { value: LiteralsAst::Number(123 as f64) } )}), 
                operator: Token { tokentype: TokenType::Star, lexeme: "*".to_string(), literal: Literals::Null, line: 1 }, 
                right: Box::new(Expr::Grouping { expression:Box::new( Expr::Literal { value: LiteralsAst::Number(45.67) }) } )
            }
        );

        let expr_result = expression.to_string();
        assert_eq!(expr_result, "(* (- 123) (group 45.67))");
    }
}
// fn main() {
//     let expression = Box::new(Expr::Binary { 
//         left: Box::new( Expr::Unary { 
//                 operator: Token { tokentype: TokenType::Minus, lexeme: "-".to_string(), literal: Literals::Null, line: 1 }, 
//                 right:Box::new( Expr::Literal { value: LiteralsAst::Number(123 as f64) } )}), 
//             operator: Token { tokentype: TokenType::Star, lexeme: "*".to_string(), literal: Literals::Null, line: 1 }, 
//             right: Box::new(Expr::Grouping { expression:Box::new( Expr::Literal { value: LiteralsAst::Number(45.67) }) } )
//         }
//     );


//     println!("Expression; {:?}", expression);
// }

// Expr expression = new Expr.Binary(
//     new Expr.Unary(
//         new Token(TokenType.MINUS, "-", null, 1),
//         new Expr.Literal(123)),
//     new Token(TokenType.STAR, "*", null, 1),
//     new Expr.Grouping(
//         new Expr.Literal(45.67)));