use crate::generate_ast::{Expr, LiteralsAst};
use crate::parser::Stmt;

pub struct Interpreter{

}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, statements: Vec<Stmt>) -> LiteralsAst {
        for statement in statements {
            match statement {
                Stmt::Expression { expression } => {
                    let value = expression.evaluate().unwrap();
                    println!("{:?}", value);
                    return value;
                },
                Stmt::Print { expression } => {
                    let value = expression.evaluate().unwrap();
                    println!("{:?}", value);
                    return value;
                },
                _ => {
                    todo!()
                }
            }
        }
        todo!()
    }

    fn execute(&self, statements: Vec<Stmt>) {
        todo!();
    }
}