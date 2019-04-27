use crate::frontend::ast::{Expr, ExprNode, Stmt};
use crate::frontend::visitor::{walk_expr, walk_stmt, Visitor};
use std::{error, fmt};

pub fn generate(stmts: Stmt) -> Result<(), &'static str> {
    let mut assembler = Assembler::new();
    assembler.assemble(&stmts).map_err(|err| "Failed!")
}

pub struct Assembler;

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    pub fn assemble(&mut self, stmt: &Stmt) -> Result<(), ()> {
        self.visit_stmt(stmt);
        Ok(())
    }
}

impl Visitor<()> for Assembler {
    fn visit_stmt(&mut self, s: &Stmt) {
        println!("Assembling harder!");
        walk_stmt(self, s);
    }

    fn visit_expr(&mut self, e: &ExprNode) {
        match &*e.expr {
            Expr::BinaryOp(lhs, op, rhs) => {
                walk_expr(self, lhs);
                walk_expr(self, rhs);
                println!("+ {} {} {}", "r1", "r2", "r3");
            }
            _ => {}
        }
        walk_expr(self, e);
    }
}

#[derive(Debug)]
struct StackError;

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stack empty")
    }
}

impl error::Error for StackError {}
