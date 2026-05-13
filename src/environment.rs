// he wants map and hashmap?
// what is a map?
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::tokens::Literal;

pub struct Environment {
    values: HashMap<String, RefCell<Literal>>
}

impl Environment {
    // I might need to convert it to a RefCell in this function
    fn define(&mut self, name: String, value: RefCell<Literal>) {
        self.values.insert(name, value);
    }

    fn get(&mut self, name: String) -> &RefCell<Literal> {
        self.values.get(&name).expect(format!("Variable '{name}' is undefined.").as_str())
    }
}

