use std::collections::HashMap;
use std::rc::Rc;
use crate::{token::Token};
use crate::generate_ast::{Expr, LiteralsAst};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex,};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::{RwLock};
use std::borrow::BorrowMut;


#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<Environment>>,
    values: HashMap<String, LiteralsAst>
}

impl Environment {
    pub fn new() -> Self {
        println!("New Environment");
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralsAst) {
        // print!("define: {:?} {:?} \n", name, value);
        self.values.insert(name, value);
        // println!("define store");

    }

    pub fn get(&self, name: String) -> Option<&LiteralsAst> {
        print!("get: {:?} \n", self.values);

        let value = self.values.get(&name);

        match (value, &self.enclosing) {
            (Some(val), _) => {
                Some(val)
            },
            (None, Some(env)) => {
                env.get(name)
            },
            (None, None) => {
                None
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: LiteralsAst) {
        let name_lex = self.values.get(&name.lexeme);

        match (name_lex, &self.enclosing) {
            (Some(_), _) => {
                // println!("assign store 1");
                self.values.insert(name.lexeme.clone(), value);
            },
            (None, Some(env)) => {
                // println!("assign store 2");
                Rc::get_mut(&mut env.clone()).expect("Cannot get mut env").define(name.to_string(), value);

            },
            (None, None) => {
                // println!("assign store None");

                false;
            }
        }
    }  

}
