use crate::generate_ast::{Expr, LiteralsAst};
use crate::parser::Stmt;
use crate::environment;

use environment::Environment;
use once_cell::sync::Lazy;
use std::alloc::Layout;
use std::{env, vec};
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
        println!("statements: {:?}\n", statements);
        for statement in statements {
            // println!("STatement: {:?}", statement);
            match statement {
                Stmt::Expression { expression } => {
                    println!("Stmt Expre");

                    let value = expression.evaluate( Rc::get_mut(&mut self.environment).expect("Expected a mutable environment")).unwrap();
                    return value;
                },
                Stmt::If { condition, then_branch, else_branch } => {
                    println!("Stmt If");

                    // println!("condition: {:?}", condition);
                    // println!("then_branch: {:?}", then_branch);
                    // println!("else_branch: {:?}", else_branch);

                    let value = condition.evaluate(Rc::get_mut(&mut self.environment).expect("Expected a mutable environment")).unwrap();
                    println!("value: {:?}", value);

                    if value.is_truthy() {
                        self.execute(vec![*then_branch]);
                    } else if *else_branch.clone().unwrap()  != Stmt::Null {
                        self.execute(vec![*else_branch.unwrap()]);
                    }
                    // return LiteralsAst::Null
                },
                Stmt::Print { expression } => {
                    println!("Stmt Print");
                    // let env = Rc::make_mut(&mut self.environment);
                    let value = expression.evaluate(Rc::get_mut(&mut self.environment).expect("Expected a mutable environment"));
                    let val = match value {
                        Ok(val) => val,
                        Err(err) => LiteralsAst::Strings(format!("Variable {} {}", expression.to_string(), err.to_string()))
                    };
                    println!("Print ENvironmet: {:?}", Rc::get_mut(&mut self.environment).expect("Expected a mutable environment"));

                    println!("Stmt::Print: {:?}", val);
                    return val;
                },
                Stmt::Var  { name, initializer } => {
                    println!("Stmt Var");

                    let value = initializer.evaluate(Rc::get_mut(&mut self.environment).expect("Expected a mutable environment")).unwrap();
                    let env = Rc::get_mut(&mut self.environment).expect("Expected a mutable environment");
                    env.define(name.lexeme, value.clone());
                    // println!(" Stmt::Var: {:?}", value.clone());
                    return value;
                },
                Stmt::Block {statements } => {

                    println!("Stmt Block");
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(Rc::clone(&self.environment)); 
                    let previous_environment = std::mem::replace(&mut self.environment, Rc::new(new_environment));
                    let result = self.execute(statements);
                    
                    // Restore previous environment
                    self.environment = previous_environment;
                    
                    return result;
                },
                Stmt::Null => {
                    println!("Stmt Null");
                    return LiteralsAst::Null;
                }
            }
        }
        // todo!()
        LiteralsAst::Null
    }

    fn execute(&mut self, statement: Vec<Stmt>) -> LiteralsAst {
        // todo!();
        // println!("In Execute {:?}", statement);
        println!("execute ENvironmet: {:?}", Rc::get_mut(&mut self.environment).expect("Expected a mutable environment"));

        self.interpret(statement)
    }

}