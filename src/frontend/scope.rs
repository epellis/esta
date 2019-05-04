use super::visitor::{walk_expr, walk_stmt, Visitor};
use crate::frontend::ast::{Expr, ExprNode, Stmt};
use std::collections::HashMap;

/// Scope Checker
///
/// Performs a pre-order traversal of the AST and ensures that each declared
/// variable exists in a scope.
/// TODO: Keep a persistent record of each scope
pub struct Scope {
    enclosures: Vec<HashMap<String, ExprNode>>,
}

pub fn discover_scope(stmt: Stmt) -> Result<Stmt, &'static str> {
    let mut scope = Scope::new_root();
    scope.visit_stmt(&stmt);
    Ok(stmt)
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
        self.enclosures.pop().expect("popped the global stack");
    }

    pub fn define(&mut self, id: &str) {
        // TODO: Find a less destructive way of pushing to the upper stack
        let mut top = self.enclosures.pop().expect("popped the global stack");
        //        top.insert(id.to_string(), val.clone());
        top.insert(id.to_string(), ExprNode::new_nil());
        self.enclosures.push(top);
    }

    pub fn lookup_var(&mut self, id: &str) -> Option<&ExprNode> {
        for encl in self.enclosures.iter().rev() {
            if let Some(val) = encl.get(id) {
                return Some(val);
            }
        }
        println!("Failed searching:\t");
        for encl in self.enclosures.iter().rev() {
            print!("{}\t", level_to_string(encl));
        }
        None
    }
}

impl Visitor<()> for Scope {
    fn visit_stmt(&mut self, s: &Stmt) {
        match s {
            Stmt::Block(_) => {
                self.push_level();
                walk_stmt(self, s);
                self.pop_level();
            }
            Stmt::Declaration(id) => {
                self.define(id);
            }
            Stmt::FunDecl(id, params, ret, body) => {
                self.push_level();
                for param in params {
                    if let Expr::Identifier(p) = &*param.expr {
                        self.define(p);
                    }
                }
                walk_stmt(self, s);
                self.pop_level();
            }
            _ => walk_stmt(self, s),
        }
    }

    fn visit_expr(&mut self, e: &ExprNode) {
        match &*e.expr {
            Expr::Identifier(id) => {
                self.lookup_var(id)
                    .ok_or_else(|| println!("{} could not be found", id))
                    .unwrap();
            }
            _ => walk_expr(self, e),
        }
        walk_expr(self, e);
    }
}

// Quick way to print any level in the enclosure stack. Great for debugging
fn level_to_string(level: &HashMap<String, ExprNode>) -> String {
    let pairs: Vec<String> = level.iter().map(|(k, v)| format!("{}->{}", k, v)).collect();
    let pairs = pairs.join(", ");
    pairs
}
