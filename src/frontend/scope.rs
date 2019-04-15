use crate::ast::{Expr, Literal, Stmt};
use std::collections::HashSet;

pub struct Scope {
    enclosures: Vec<HashSet<String>>,
}

impl Scope {
    pub fn new_root() -> Scope {
        let enclosures = vec![HashSet::new()];
        Scope { enclosures }
    }

    pub fn push_level(&mut self) {
        self.enclosures.push(HashSet::new());
    }

    pub fn pop_level(&mut self) {
        self.enclosures.pop().expect("popped the global stack");
    }

    pub fn define(&mut self, id: &str) {
        let mut top = self.enclosures.pop().expect("popped the global stack");
        top.insert(id.to_string());
        self.enclosures.push(top);
    }

    pub fn lookup_var(&mut self, id: &str) -> Option<usize> {
        for (i, encl) in self.enclosures.iter().rev().enumerate() {
            if encl.contains(id) {
                return Some(i);
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
                self.define(id);
                self.traverse_expr(rhs)?;
            }
            Stmt::Assignment(lhs, rhs) => {
                self.traverse_expr(lhs)?;
                self.traverse_expr(rhs)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn traverse_expr(&mut self, expr: &Expr) -> Result<(), &'static str> {
        match expr {
            Expr::Identifier(id) => {
                self.lookup_var(id)
                    .ok_or("could not find declaration of id")?;
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
