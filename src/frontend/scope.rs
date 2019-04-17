use crate::ast::{Expr, ExprNode, Stmt};
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

    pub fn traverse_stmt(&mut self, stmt: &Stmt) -> Result<(), &'static str> {
        match stmt {
            Stmt::Block(stmts) => {
                self.push_level();
                for stmt in stmts {
                    self.traverse_stmt(stmt)?;
                }
                self.pop_level();
            }
            Stmt::Declaration(id, rhs) => {
                self.define(id, rhs);
                self.traverse_expr(rhs)?;
            }
            Stmt::Assignment(lhs, rhs) => {
                self.traverse_expr(lhs)?;
                self.traverse_expr(rhs)?;
            }
            Stmt::FunDecl(id, params, ret, body) => {
                // TODO: Add function to some type of table, return type of such
                // TODO: Define parameters in their scope
                // TODO: Traverse body
            }
            _ => {}
        }
        Ok(())
    }

    pub fn traverse_expr(&mut self, expr: &ExprNode) -> Result<(), &'static str> {
        match &*expr.expr {
            Expr::Identifier(id) => {
                self.lookup_var(id).ok_or_else(|| {
                    eprintln!("Couldn't find: {}", id);
                    "No variable declaration"
                })?;
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn scope(stmt: Stmt) -> Result<Stmt, &'static str> {
    let mut scope = Scope::new_root();
    scope.traverse_stmt(&stmt)?;
    Ok(stmt)
}
