use super::allocation::Alloc;
use crate::util::stack::Stack;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AsmCtx {
    base: String,                          // Tracks which function is being built
    scopes: HashMap<String, Stack<Alloc>>, // Tracks all scoped variables and their locations
    pub locals: HashMap<String, usize>,    // Tracks all local variables allocated within a function
    suffix: usize,                         // Keeps a unique suffix for each label
}

impl AsmCtx {
    pub fn new() -> AsmCtx {
        AsmCtx {
            base: "GLOBAL".to_string(),
            scopes: HashMap::new(),
            locals: HashMap::new(),
            suffix: 0,
        }
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
        self.locals.insert(self.base.clone(), 0);
        self.push_scope();
    }
    pub fn pop_fun(&mut self) {
        self.pop_scope();
        self.base = "GLOBAL".to_string();
    }
    // TODO: This is begging for some and_then() combinators
    // TODO: Also clean up pop/push
    pub fn define(&mut self, id: &str) {
        let local = self.locals.get_mut(&self.base).unwrap();
        *local += 1;
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
