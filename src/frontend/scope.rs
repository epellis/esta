use super::visitor::{walk_expr, walk_stmt, Visitor};
use crate::frontend::ast::{Expr, ExprNode, Stmt};
use std::collections::HashMap;

pub struct Scope {
    enclosures: Vec<HashMap<String, ExprNode>>,
}

impl Scope {
    pub fn new_root() -> Scope {
        let enclosures = vec![HashMap::new()];
        Scope { enclosures }
    }

    pub fn push_level(&mut self) {
        self.enclosures.push(HashMap::new());
    }

    pub fn pop_level(&mut self) {
        let top = self.enclosures.last().unwrap();

        for (key, val) in top.iter() {
            println!("{} {}", key, val);
        }

        self.enclosures.pop().expect("popped the global stack");
    }

    pub fn define(&mut self, id: &str, val: &ExprNode) {
        let mut top = self.enclosures.pop().expect("popped the global stack");
        top.insert(id.to_string(), val.clone());
        self.enclosures.push(top);
    }

    pub fn lookup_var(&mut self, id: &str) -> Option<&ExprNode> {
        for encl in self.enclosures.iter().rev() {
            if let Some(val) = encl.get(id) {
                return Some(val);
            }
        }
        None
    }
}

impl Visitor for Scope {
    fn visit_expr(&mut self, e: &ExprNode) {
        println!("Hey this is a custom scope operator");
        walk_expr(self, e);
    }
}

pub fn define_scope(stmt: Stmt) -> Result<Stmt, &'static str> {
    let mut scope = Scope::new_root();
    scope.visit_stmt(&stmt);
    Ok(stmt)
}
