use std::cell::RefCell;
use std::collections::HashMap;

use crate::statements::RuntimeError;
use crate::tokens::Literal;

#[allow(dead_code)]
pub struct Environment {
    values: HashMap<String, RefCell<Literal>>
}

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }
    // I might need to convert it to a RefCell in this function
    pub fn define(&mut self, name: String, value: RefCell<Literal>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: String) -> Result<&RefCell<Literal>, RuntimeError> {
        match self.values.get(&name) {
            Some(v) => Ok(v),
            // I need to figure out line_no or construct a different error
            None => Err(RuntimeError { line_no: 0, message: format!("Variable '{name}' is undefined.")})

        }
    }
}

