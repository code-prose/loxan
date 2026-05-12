// he wants map and hashmap?
// what is a map?
use std::collections::HashMap;

use crate::tokens::Literal;

// could I use &'a str?
// seems sloppy
pub struct Environment {
    values: HashMap<String, Literal>
}


