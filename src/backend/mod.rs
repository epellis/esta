mod allocator;

use self::allocator::Allocator;
use crate::frontend::ast::{Expr, ExprNode, Literal, Opcode, Stmt};
use crate::frontend::visitor::{walk_expr, walk_stmt, Visitor};
use crate::vm::bytecode::{ByteCode, Inst, OP_TO_BYTE};

// TODO: Split off bool to t conversion funcs
use crate::vm::VirtualMachine;

pub fn generate(stmts: Stmt) -> Result<Vec<Inst<i64>>, &'static str> {
    let mut assembler = Assembler::new();
    assembler
        .assemble(&stmts)
        .map_err(|_| "Couldn't Assemble")?;
    assembler.inst.push(Inst::new_inst(ByteCode::HALT));
    Ok(assembler.inst)
}

pub struct Assembler {
    inst: Vec<Inst<i64>>,
    rho: usize,
    alloc: Allocator,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            inst: Vec::new(),
            rho: 0,
            alloc: Allocator::new(),
        }
    }

    pub fn assemble(&mut self, stmt: &Stmt) -> Result<(), ()> {
        self.visit_stmt(stmt);
        Ok(())
    }

    pub fn l_value(&mut self, lhs: &ExprNode) -> i64 {
        match &*lhs.expr {
            Expr::Identifier(id) => {
                if let Some((rho, offset)) = self.alloc.lookup(id) {
                    return rho as i64 + offset as i64;
                }
                -1
            }
            _ => -1,
        }
    }
}

impl Visitor<()> for Assembler {
    fn visit_stmt(&mut self, s: &Stmt) {
        match s {
            Stmt::Block(_) => {
                self.alloc.push_level();
                walk_stmt(self, s);
                self.alloc.pop_level();
            }
            Stmt::Declaration(id, rhs) => {
                self.visit_expr(rhs);
                self.alloc.define(id, rhs);
                if let Some((rho, offset)) = self.alloc.lookup(id) {
                    let offset: i64 = rho as i64 + offset as i64;
                    self.inst.push(Inst::new_data(ByteCode::LOADC, offset));
                }
                self.inst.push(Inst::new_inst(ByteCode::STORE));
            }
            Stmt::Assignment(lhs, rhs) => {
                self.visit_expr(rhs);
                let offset = self.l_value(lhs);
                self.inst.push(Inst::new_data(ByteCode::LOADC, offset));
                self.inst.push(Inst::new_inst(ByteCode::STORE));
            }
            Stmt::If(cond, stmt, alt) => {
                self.visit_expr(cond);
                let jump_a = self.inst.len();
                self.inst.push(Inst::new_data(ByteCode::JUMPZ, -1));
                self.visit_stmt(stmt);
                let jump_b = self.inst.len();
                self.inst.push(Inst::new_data(ByteCode::JUMP, -1));
                self.visit_stmt(alt);

                // Now retroactively edit the jump locations
                self.inst[jump_a] = Inst::new_data(ByteCode::JUMPZ, jump_b as i64);
                self.inst[jump_b] = Inst::new_data(ByteCode::JUMPZ, self.inst.len() as i64);
            }
            _ => {
                walk_stmt(self, s);
            }
        }
    }

    fn visit_expr(&mut self, e: &ExprNode) {
        walk_expr(self, e);
        match &*e.expr {
            Expr::BinaryOp(_, op, _) => {
                let bytecode: ByteCode = OP_TO_BYTE.get(op).unwrap().clone();
                self.inst.push(Inst::new_inst(bytecode));
            }
            Expr::UnaryOp(op, _) => {
                // TODO: Deal with ambiguity of minus operator
                let bytecode: ByteCode = OP_TO_BYTE.get(op).unwrap().clone();
                self.inst.push(Inst::new_inst(bytecode));
            }
            Expr::Identifier(id) => match self.alloc.lookup(id) {
                Some((rho, offset)) => {
                    let offset: i64 = rho as i64 + offset as i64;
                    self.inst.push(Inst::new_data(ByteCode::LOADC, offset));
                    self.inst.push(Inst::new_inst(ByteCode::LOAD));
                }
                None => panic!("Could not find id"),
            },
            Expr::Literal(value) => {
                // TODO: Handle String and Nil
                match value {
                    Literal::Number(value) => {
                        self.inst.push(Inst::new_data(ByteCode::LOADC, *value));
                    }
                    Literal::Boolean(value) => {
                        let value: i64 = VirtualMachine::bool_to_t(*value);
                        self.inst.push(Inst::new_data(ByteCode::LOADC, value));
                    }
                    _ => {}
                }
            }
            _ => println!("Couldn't match {}", &*e.expr),
        }
    }
}
