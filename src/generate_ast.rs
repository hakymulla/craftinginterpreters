use crate::environment::{self, Environment};
use crate::{token::Token, scanner::*};
use std::env;
use std::fmt::format;
use std::{collections::btree_map::Values, fmt};
use std::ops::Neg;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralsAst {
    Number(f64),
    Strings(String),
    True,
    False,
    Null
}

impl Neg for LiteralsAst {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            LiteralsAst::Number(n) => LiteralsAst::Number(-n), // Negate the number
            _ => panic!("Cannot apply negation to non-numeric literal"), // Panic for invalid cases
        }
    }
}

impl fmt::Display for LiteralsAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match on the enum variant and write the desired string representation
        let description = match self {
            LiteralsAst::Number(x) => x.to_string(),
            LiteralsAst::Strings(x) => x.to_string(),
            LiteralsAst::True => true.to_string(),
            LiteralsAst::False => false.to_string(),
            LiteralsAst::Null => "nil".to_string()
        };
        write!(f, "{}", description)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign {name: Token, value: Box<Expr>},
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping {expression: Box<Expr>},
    Literal {value: LiteralsAst},
    Unary {operator: Token, right: Box<Expr>},
    Variable {name: Token},
    Null
}

impl LiteralsAst {
    fn is_truthy(&self) -> bool {
        match self {
            LiteralsAst::Number(x) => {
                if *x == 0 as f64{
                    true
                } else {
                    false
                }
            },
            LiteralsAst::Strings(x) => {
                if x.len() == 0 {
                    true
                } else {
                    false
                }
            },
            LiteralsAst::True => false,
            LiteralsAst::False => true,
            LiteralsAst::Null => true
        }
    }

    fn is_equal(a: &LiteralsAst, b: &LiteralsAst) -> bool {   
        println!("is equal function");
        if *a == LiteralsAst::Null && *b == LiteralsAst::Null {
            return true;
        }
        if *a == LiteralsAst::Null {
            return false;
        }
        return a == b;

    }
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
            },
            Expr::Variable { name } => {
                return format!("{}", name.lexeme);
            },
            Expr::Assign { name, value } => {
                return format!("({} {})", name.lexeme, value.to_string());
            },
            Expr::Null => {
                return "".to_string();
            }
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.to_string())
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<LiteralsAst, String> {
        // println!("environment: {:?}\n", environment);

        match self {
            Expr::Assign { name, value } => {
                println!("Assign in Expr generate_ast");
                // let value = value.evaluate(environment)?;
                let val = environment.get(name.lexeme.clone()).unwrap().clone();
                environment.assign(name.clone(), val.clone());
                return Ok(val.clone());
             },
            Expr::Variable { name } => {
                let value = environment.get(name.lexeme.clone());
                match value {
                    Some(val) => return Ok(val.clone()),
                    None => return Err("is not declared".to_string())
                };
                // Ok(environment.get(&name.lexeme.clone()).unwrap())
             },
            Expr::Literal { value } => {
                Ok(value.clone())
            },
            Expr::Grouping { expression } => {
               expression.evaluate(environment)
            },
            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment)?;
                match (&operator.tokentype, right) {
                    (TokenType::Minus, LiteralsAst::Number(x)) => {
                        return Ok(LiteralsAst::Number(-x));
                    },
                    (TokenType::Minus, _) => {
                        return Err("Minus Not Implemeted for String".to_string());
                    },
                    (TokenType::Bang, x) => {
                        if x.is_truthy() {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (_, _) => {
                        return Err("Not Implemented".to_string());
                    }
                }
            },
            Expr::Binary { left, operator, right } => {
                let right = right.evaluate(environment)?;
                let left = left.evaluate(environment)?;

                if operator.tokentype == TokenType::Plus {
                    match (&left, &right) {
                        (LiteralsAst::Number(left), LiteralsAst::Number(right)) => {
                            return Ok(LiteralsAst::Number((*left) as f64 + (*right) as f64));
                        },
                        (LiteralsAst::Strings(left), LiteralsAst::Strings(right)) => {
                            return Ok(LiteralsAst::Strings(format!("{}{}", left, right))) ;
                        },
                        (LiteralsAst::Strings(_), LiteralsAst::Number(_)) => {
                            return Err("Operands must be two numbers or two strings.".to_string());
                        },
                        (LiteralsAst::Number(_), LiteralsAst::Strings(_)) => {
                            return Err("Operands must be two numbers or two strings.".to_string());
                        },
                        (_, _) => {
                            return Err("Operands must be two numbers or two strings.".to_string());
                        }
                    }
                }

                match (&left, &operator.tokentype, &right) {
                    (LiteralsAst::Number(left),  TokenType::Minus, LiteralsAst::Number(right)) => {
                        return Ok(LiteralsAst::Number((*left)  - (*right))) ;
                    },
                    (LiteralsAst::Number(left),  TokenType::Slash, LiteralsAst::Number(right)) => {
                        return Ok(LiteralsAst::Number((*left) as f64 / (*right) as f64));
                    },
                    (LiteralsAst::Number(left),  TokenType::Star, LiteralsAst::Number(right)) => {
                        return Ok(LiteralsAst::Number((*left) as f64 * (*right) as f64));
                    },
                    (LiteralsAst::Number(left),  TokenType::Greater, LiteralsAst::Number(right)) => {
                        let value = left > right;
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (LiteralsAst::Number(left),  TokenType::GreaterEqual, LiteralsAst::Number(right)) => {
                        let value = left >= right;
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (LiteralsAst::Number(left),  TokenType::Less, LiteralsAst::Number(right)) => {
                        let value = left < right;
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (LiteralsAst::Number(left),  TokenType::LessEqual, LiteralsAst::Number(right)) => {
                        let value = left <= right;
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (left,  TokenType::EqualEqual, right) => {
                        let value = LiteralsAst::is_equal(left, right);
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (left,  TokenType::BangEqual, right) => {
                        let value = !LiteralsAst::is_equal(left, right);
                        if value {
                            return Ok(LiteralsAst::True);
                        } else {
                            return Ok(LiteralsAst::False);
                        }
                    },
                    (_, _, _) => {
                        Ok(LiteralsAst::Null)
                    }
                }

            },
            Expr::Null => {
                Ok(LiteralsAst::Null)
            },
        }
    
    }

}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::scanner::{Scanner};
//     use crate::parser::{Parser};
//     use crate::interpreter::{Interpreter};
//     #[test]
//     fn ast_print() {
//         let expression = Box::new(Expr::Binary { 
//             left: Box::new( Expr::Unary { 
//                     operator: Token { tokentype: TokenType::Minus, lexeme: "-".to_string(), literal: Literals::Null, line: 1 }, 
//                     right:Box::new( Expr::Literal { value: LiteralsAst::Number(123 as f64) } )}), 
//             operator: Token { tokentype: TokenType::Star, lexeme: "*".to_string(), literal: Literals::Null, line: 1 }, 
//             right: Box::new(Expr::Grouping { expression:Box::new( Expr::Literal { value: LiteralsAst::Number(45.67) }) } )
//             }
//         );

//         let expr_result = expression.to_string();
//         assert_eq!(expr_result, "(* (- 123) (group 45.67))");
//     }

//     #[test]
//     fn test_addition() {
//         let source = "2 + 2;".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();

//         let environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::Number(4.0));
//     }

//     #[test]
//     fn test_subtraction() {
//         let source = "42-10;".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
        
//         let environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::Number(32.0));
//     }

//     #[test]
//     fn test_multiplication() {
//         let source = "4 * 10;".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::Number(40.0));
//     }

//     #[test]
//     fn test_division() {
//         let source = "4 / 2;".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::Number(2.0));
//     }

//     #[test]
//     fn test_concatenation() {
//         let source = "\"Hello\" + \"World\";".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         let mut parser = Parser::new(tokens);

//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::Strings("HelloWorld".to_string()));
//     }

//     #[test]
//     fn test_equal_equal() {
//         let source = "2 == 2".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::True);
//     }

//     #[test]
//     fn test_bang_equal() {
//         let source = "2 != 3".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::True);
//     }

//     #[test]
//     fn test_greater_than() {
//         let source = "2 > 3".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::False);
//     }

//     #[test]
//     fn test_lesser_than() {
//         let source = "2 < 3".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//         assert_eq!(value, LiteralsAst::True);
//     }

//     #[test]
//     #[should_panic(expected = "Operands must be two numbers or two strings.")]
//     fn test_addition_fail() {
//         let source = "2 + \"Test\";".to_string();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         let mut parser = Parser::new(tokens);
//         let parse = parser.parse();
//         let mut environment = Environment::new(); 
//         let mut interpreter = Interpreter::new(environment);
//         let value = interpreter.interpret(parse);
//     }
// }