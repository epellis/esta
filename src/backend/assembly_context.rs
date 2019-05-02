use super::allocation::Alloc;
use crate::util::stack::Stack;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AsmCtx {
    base: String,
    scopes: HashMap<String, Stack<Alloc>>,
    suffix: usize,
}

impl AsmCtx {
    pub fn new() -> AsmCtx {
        let mut me = AsmCtx {
            base: "main".to_string(),
            scopes: HashMap::new(),
            suffix: 0,
        };
        me.push_scope();
        me
    }
    pub fn next_label(&mut self) -> String {
        self.suffix += 1;
        format!("{}_{}", self.base, self.suffix)
    }
    pub fn push_scope(&mut self) {
        self.scopes.get_mut(&self.base).unwrap().push(Alloc::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.get_mut(&self.base).unwrap().pop();
    }
    pub fn add_fun(&mut self, id: &str) {
        self.base = id.to_string();
        self.scopes.insert(self.base.clone(), Stack::new());
    }
    pub fn pop_fun(&mut self) {
        self.base = "main".to_string();
    }
    // TODO: This is begging for some and_then() combinators
    // TODO: Also clean up pop/push
    pub fn define(&mut self, id: &str) {
        let stack = self.scopes.get_mut(&self.base).unwrap();
        let mut top = stack.pop().unwrap();
        top.define(id);
        stack.push(top);
    }
    pub fn get(&self, id: &str) -> Result<usize, &'static str> {
        let stack = self.scopes.get(&self.base).unwrap();
        for scope in stack.iter() {
            if scope.get(id).is_some() {
                return Ok(scope.get(id).unwrap());
            }
        }
        Err("Couldn't find id")
    }
}
