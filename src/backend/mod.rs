use crate::frontend::ast::{Expr, ExprNode, Stmt};
use crate::frontend::visitor::{walk_expr, walk_stmt, VisitResult, Visitor};
use std::error;
use std::fmt;

pub fn generate(stmts: Stmt) -> Result<(), &'static str> {
    let mut assembler = Assembler::new();
    assembler.assemble(&stmts).map_err(|err| "Failed!")
}

pub struct Assembler;

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    pub fn assemble(&mut self, stmt: &Stmt) -> VisitResult<()> {
        self.visit_stmt(stmt)
    }
}

impl Visitor<()> for Assembler {
    fn visit_stmt(&mut self, s: &Stmt) -> VisitResult<()> {
        println!("Assembling harder!");
        walk_stmt(self, s);
        Ok(())
    }

    fn visit_expr(&mut self, e: &ExprNode) -> VisitResult<()> {
        match &*e.expr {
            Expr::BinaryOp(lhs, op, rhs) => {
                walk_expr(self, lhs);
                walk_expr(self, rhs);
                println!("+ {} {} {}", "r1", "r2", "r3");
            }
            _ => {}
        }
        walk_expr(self, e);
        Ok(())
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
