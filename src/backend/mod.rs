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
    Ok(assembler.inst)
}

pub struct Assembler {
    inst: Vec<Inst<i64>>,
    rho: usize,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            inst: Vec::new(),
            rho: 0,
        }
    }

    pub fn assemble(&mut self, stmt: &Stmt) -> Result<(), ()> {
        self.visit_stmt(stmt);
        Ok(())
    }
}

impl Visitor<()> for Assembler {
    fn visit_stmt(&mut self, s: &Stmt) {
        walk_stmt(self, s);
    }

    fn visit_expr(&mut self, e: &ExprNode) {
        walk_expr(self, e);
        match &*e.expr {
            Expr::BinaryOp(lhs, op, rhs) => {
                //                walk_expr(self, lhs);
                //                walk_expr(self, rhs);
                let bytecode: ByteCode = OP_TO_BYTE.get(op).unwrap().clone();
                self.inst.push(Inst::new_inst(bytecode));
            }
            Expr::Literal(value) => {
                // TODO: Handle String and Float
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
            _ => {}
        }
    }
}
