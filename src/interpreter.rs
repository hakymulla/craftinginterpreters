use crate::generate_ast::{Expr, LiteralsAst};
use crate::parser::Stmt;
use crate::environment;

use environment::Environment;
use once_cell::sync::Lazy;
use std::alloc::Layout;
use std::env;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;
// use environment::{GLOBAL_ENV};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]

pub struct Interpreter{
    environment: Rc<Environment>
}

impl Interpreter {
    pub fn new() -> Self {
        println!("New Interpreter");
        Self {
            environment: Rc::new(Environment::new())
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> LiteralsAst {
        println!("interpret called");

      
        for statement in statements {
            println!("STatement: {:?}", statement);
            match statement {
                Stmt::Expression { expression } => {
                    let value = expression.evaluate( Rc::get_mut(&mut self.environment).expect("Expected a mutable environment")).unwrap();
                    return value;
                },
                Stmt::Print { expression } => {
                    println!("Stmt Print");
                    // let env = Rc::make_mut(&mut self.environment);
                    let value = expression.evaluate(Rc::get_mut(&mut self.environment).expect("Expected a mutable environment"));
                    let val = match value {
                        Ok(val) => val,
                        Err(err) => LiteralsAst::Strings(format!("Variable {} {}", expression.to_string(), err.to_string()))
                    };
                    println!("Stmt::Print: {:?}", val);
                    return val;
                },
                Stmt::Var  { name, initializer } => {
                    let value = initializer.evaluate(Rc::get_mut(&mut self.environment).expect("Expected a mutable environment")).unwrap();
                    let env = Rc::get_mut(&mut self.environment).expect("Expected a mutable environment");
                    env.define(name.lexeme, value.clone());
                    println!(" Stmt::Var: {:?}", value.clone());
                    return value;
                },
                Stmt::Block {statements } => {

                    println!("Stmt Block");
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(Rc::clone(&self.environment));  // Clone the Rc, not the Environment
                    let previous_environment = std::mem::replace(&mut self.environment, Rc::new(new_environment));
                    let result = self.execute(statements);
                    
                    // Restore previous environment
                    self.environment = previous_environment;
                    
                    return result;
                },
            }
        }
        todo!()
    }

    fn execute(&mut self, statement: Vec<Stmt>) -> LiteralsAst {
        // todo!();
        println!("In Execute {:?}", statement);
        self.interpret(statement)
    }

}