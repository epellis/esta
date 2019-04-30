use crate::frontend::ast::{Expr, ExprNode, Stmt, Type};
use std::collections::HashMap;

#[derive(Debug)]
struct StackInfo {
    top: usize,
    vars: HashMap<String, usize>,
}

impl StackInfo {
    pub fn new() -> StackInfo {
        StackInfo {
            top: 0,
            vars: HashMap::new(),
        }
    }
    pub fn define(&mut self, id: &str, val: &ExprNode) {
        self.vars.insert(id.to_string(), self.top);
        self.top += 1;
    }
    pub fn lookup(&self, id: &str) -> Option<usize> {
        if let Some(val) = self.vars.get(id) {
            return Some(val.clone());
        }
        return None;
    }
}

/// Static Allocator
///
/// Traverses a statement/expression, discovers variables and allocates them
/// on a stack
// TODO: Make a generic stack-map data structure and a trait to go with it
#[derive(Debug)]
pub struct Allocator {
    enclosures: Vec<StackInfo>,
}

impl Allocator {
    pub fn new() -> Allocator {
        Allocator {
            enclosures: vec![StackInfo::new()],
        }
    }

    pub fn push_level(&mut self) {
        self.enclosures.push(StackInfo::new());
    }

    pub fn pop_level(&mut self) {
        self.enclosures.pop().expect("popped the global stack");
    }

    pub fn define(&mut self, id: &str, val: &ExprNode) {
        let mut info = self.enclosures.pop().expect("popped the global stack");
        info.define(id, val);
        self.enclosures.push(info);
    }

    pub fn lookup(&mut self, id: &str) -> Option<usize> {
        for encl in self.enclosures.iter().rev() {
            if let Some(offset) = encl.lookup(id) {
                return Some(offset);
            }
        }
        None
    }
}
