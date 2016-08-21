use std::rc::Rc;
use std::cell::RefCell;

use evaluating::lexical_env::*;
use evaluating::value::*;

pub struct LexicalStack {
    envs: Vec<LexicalEnvPtr>
}

impl LexicalStack {
    pub fn new() -> LexicalStack {
        let env = Rc::new(RefCell::new(LexicalEnv::new_outer()));
        let stack = LexicalStack{envs: vec![env]};
        return stack;
    }

    pub fn enter_scope(&mut self) {
        let len = self.envs.len();
        assert!(len > 0);
        let top = self.envs[len-1].clone();
        let env = Rc::new(RefCell::new(LexicalEnv::new_inner(top)));
        self.envs.push(env);
    }

    pub fn leave_scope(&mut self) {
        self.envs.pop();
    }

    pub fn insert(&mut self, id: String, value: Value) {
        let len = self.envs.len();
        self.envs[len-1].borrow_mut().insert(id, value);
    }

    pub fn get(&self, id: &String) -> Value {
        let len = self.envs.len();
        return self.envs[len-1].borrow().get(id);
    }
}