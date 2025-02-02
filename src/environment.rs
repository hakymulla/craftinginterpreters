use std::collections::HashMap;
use crate::{token::Token};
use crate::generate_ast::{Expr, LiteralsAst};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GLOBAL_ENV: Lazy<Mutex<Environment>> = Lazy::new(|| Mutex::new(Environment::new()));


#[derive(Debug)]

pub struct Environment {
    values: HashMap<String, LiteralsAst>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: LiteralsAst) {
        print!("define: {:?} {:?} \n", name, value);
        self.values.insert(name.clone(), value);

        print!("get: {:?} \n", self.values);
    }

    pub fn get(&self, name: String) -> Result<LiteralsAst, String> {
        print!("get: {:?} \n", self.values);

        print!("get: {:?} \n", name);
        if self.values.contains_key(&name) {
            return Ok(self.values.get(&name).unwrap().clone());
        }
        return Err(format!("Undefined variable '{}'.", name));
    }

    pub fn assign(&mut self, name: Token, value: LiteralsAst) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
        }

    }
}
