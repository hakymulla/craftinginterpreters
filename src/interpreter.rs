use crate::generate_ast::{Expr, LiteralsAst};
use crate::parser::Stmt;
use crate::environment;

use environment::Environment;
use std::sync::MutexGuard;
use environment::{GLOBAL_ENV};

pub struct Interpreter{
    environment: Environment
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self {
            environment
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> LiteralsAst {
        let mut env = GLOBAL_ENV.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        for statement in statements {
            match statement {
                Stmt::Expression { mut expression } => {
                    let value = expression.evaluate(&mut env).unwrap();
                    println!("{:?}", value);
                    return value;
                },
                Stmt::Print { mut expression } => {
                    let value = expression.evaluate(&mut env).unwrap();
                    println!("{:?}", value);
                    return value;
                },
                Stmt::Var  { name, mut initializer } => {
                    let mut value= LiteralsAst::Null; 
                    if initializer != Expr::Null {
                        value = initializer.evaluate(&mut env).unwrap();
                    }
                    env.define(name.lexeme, value.clone());
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