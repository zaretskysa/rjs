use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


use evaluating::value::*;


pub struct LexicalEnv {
    outer: Option<LexicalEnvPtr>,
    records: HashMap<String, Value>,
}

pub type LexicalEnvPtr = Rc<RefCell<LexicalEnv>>;

impl LexicalEnv {
    pub fn new_outer() -> LexicalEnv {
        return LexicalEnv{outer: Option::None, records: HashMap::new()};
    }

    pub fn new_inner(outer: LexicalEnvPtr) -> LexicalEnv {
        return LexicalEnv{outer: Option::Some(outer), records: HashMap::new()};
    }

    pub fn insert(&mut self, id: String, value: Value) {
        self.records.insert(id.clone(), value.clone());
    }

    pub fn get(&self, id: &String) -> Value {
        match self.records.get(id) {
            Option::Some(ref value) => (*value).clone(),
            Option::None => match self.outer {
                Option::None => Value::Undefined,
                Option::Some(ref parent) => parent.borrow().get(id),
            }
        }
    }
}