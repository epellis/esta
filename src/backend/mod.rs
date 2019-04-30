mod allocator;

use self::allocator::Allocator;
use crate::frontend::ast::{Expr, ExprNode, Literal, Opcode, Stmt};
use crate::frontend::visitor::{walk_expr, walk_stmt, Visitor};
use crate::vm::bytecode::{ByteCode, Inst, BIN_OP_TO_BYTE, UN_OP_TO_BYTE};
use std::collections::HashMap;

// TODO: Split off bool to t conversion funcs
use crate::vm::VirtualMachine;

pub fn generate(stmts: Stmt) -> Result<Vec<Inst>, &'static str> {
    let mut assembler = Assembler::new();
    assembler
        .make_blocks(&stmts)
        .map_err(|_| "Couldn't Assemble")?;

    for (label, instructions) in &assembler.labels {
        println!("{}", label);
        for i in instructions {
            println!("\t{}", i);
        }
    }
    println!("");

    assembler.assemble()
}

pub struct Assembler {
    labels: HashMap<String, Vec<Inst>>,
    block: String,
    suffix: u32,
    rho: usize,
    alloc: Allocator,
}

impl Assembler {
    pub fn new() -> Assembler {
        let mut labels = HashMap::new();
        labels.insert("START".to_string(), Vec::new());
        Assembler {
            labels,
            block: "START".to_string(),
            suffix: 0,
            rho: 0,
            alloc: Allocator::new(),
        }
    }

    pub fn make_blocks(&mut self, stmt: &Stmt) -> Result<(), ()> {
        self.visit_stmt(stmt);
        Ok(())
    }

    pub fn assemble(&mut self) -> Result<Vec<Inst>, &'static str> {
        // Cap off the last instruction with a HALT
        let mut current_block = self.labels.get_mut(&self.block).unwrap();
        current_block.push(Inst::new_inst(ByteCode::HALT));

        let mut inst: Vec<Inst> = Vec::new();
        let mut label_locs: HashMap<String, usize> = HashMap::new();
        label_locs.insert("START".to_string(), 0);

        // Make sure to get the start first
        let start = self.labels.remove("START").unwrap();
        inst.extend(start);

        for (label, block) in &self.labels {
            label_locs.insert(label.clone(), inst.len());
            let block = block.clone();
            inst.extend(block);
        }

        for (label, offset) in &label_locs {
            inst = inst
                .iter()
                .map(|i| i.update_lbl(label, *offset as i64))
                .collect();
        }

        Ok(inst)
    }

    fn next_label(&mut self) -> String {
        let lbl = format!("{}_{}", self.block, self.suffix);
        self.suffix += 1;
        self.labels.insert(lbl.clone(), Vec::new());
        lbl
    }

    pub fn l_value(&mut self, lhs: &ExprNode) -> Vec<Inst> {
        let mut inst = Vec::new();
        if let Expr::Identifier(id) = &*lhs.expr {
            let offset = self.alloc.lookup(id).map_or(0, |x| x);
            let offset = self.rho as i64 + offset as i64;
            inst.push(Inst::new_data(ByteCode::LOADC, offset));
        }
        inst
    }

    pub fn r_value(&mut self, rhs: &ExprNode) -> Vec<Inst> {
        match &*rhs.expr {
            Expr::Identifier(id) => {
                let mut inst = self.l_value(rhs);
                inst.push(Inst::new_inst(ByteCode::LOAD));
                return inst;
            }
            Expr::Literal(literal) => {
                // TODO: Handle String and Nil
                let mut inst = Vec::new();
                match literal {
                    Literal::Number(literal) => {
                        inst.push(Inst::new_data(ByteCode::LOADC, *literal));
                    }
                    Literal::Boolean(literal) => {
                        let literal: i64 = VirtualMachine::bool_to_t(*literal);
                        inst.push(Inst::new_data(ByteCode::LOADC, literal));
                    }
                    _ => {}
                }
                return inst;
            }
            Expr::BinaryOp(lhs, op, rhs) => {
                // TODO: Handle ambiguity of minus operator
                let mut inst = Vec::new();
                inst.extend(self.r_value(lhs));
                inst.extend(self.r_value(rhs));
                let bytecode: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
                inst.push(Inst::new_inst(bytecode));
                inst
            }
            Expr::UnaryOp(op, rhs) => {
                let mut inst = Vec::new();
                inst.extend(self.r_value(rhs));
                let bytecode: ByteCode = UN_OP_TO_BYTE.get(op).unwrap().clone();
                inst.push(Inst::new_inst(bytecode));
                inst
            }
            _ => Vec::new(),
        }
    }
}

impl Visitor<()> for Assembler {
    fn visit_stmt(&mut self, s: &Stmt) {
        match s {
            Stmt::Block(_) => {
                //                self.rho = self.alloc.stack_top();
                self.alloc.push_level(self.alloc.stack_top());
                walk_stmt(self, s);
                self.alloc.pop_level();
                //                self.rho = self.alloc.stack_top();
            }
            Stmt::Declaration(id, rhs) => {
                let decl_block = self.r_value(rhs);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.extend(decl_block);
                self.alloc.define(id, rhs);
                let offset = self.alloc.lookup(id).unwrap() as i64;
                current_block.push(Inst::new_data(ByteCode::LOADC, offset));
                current_block.push(Inst::new_inst(ByteCode::STORE));
                current_block.push(Inst::new_inst(ByteCode::POP));
            }
            Stmt::Assignment(lhs, rhs) => {
                let rhs = self.r_value(rhs);
                let lhs = self.l_value(lhs);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.extend(rhs);
                current_block.extend(lhs);
                current_block.push(Inst::new_inst(ByteCode::STORE));
                current_block.push(Inst::new_inst(ByteCode::POP));
            }
            Stmt::If(cond, stmt, alt) => {
                let parent_lbl = self.block.clone();
                let stmt_lbl = self.next_label();
                let alt_lbl = self.next_label();
                let cont_lbl = self.next_label();

                // Evaluate cond
                let cond_block = self.r_value(cond);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.extend(cond_block);
                current_block.push(Inst::new_jump(ByteCode::JUMPZ, alt_lbl.clone()));
                current_block.push(Inst::new_jump(ByteCode::JUMP, stmt_lbl.clone()));

                // Evaluate stmt
                self.block = stmt_lbl;
                self.visit_stmt(stmt);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.push(Inst::new_jump(ByteCode::JUMP, cont_lbl.clone()));

                // Evaluate alt
                self.block = alt_lbl;
                self.visit_stmt(alt);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.push(Inst::new_jump(ByteCode::JUMP, cont_lbl.clone()));

                self.block = cont_lbl;
            }
            Stmt::While(cond, stmt) => {
                let parent_lbl = self.block.clone();
                let cond_lbl = self.next_label();
                let stmt_lbl = self.next_label();
                let cont_lbl = self.next_label();

                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.push(Inst::new_jump(ByteCode::JUMP, cond_lbl.clone()));

                // Evaluate expr
                self.block = cond_lbl.clone();
                let cond_expr = self.r_value(cond);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.extend(cond_expr);
                current_block.push(Inst::new_jump(ByteCode::JUMPZ, cont_lbl.clone()));
                current_block.push(Inst::new_jump(ByteCode::JUMP, stmt_lbl.clone()));

                // Evaluate stmt
                self.block = stmt_lbl;
                self.visit_stmt(stmt);
                let mut current_block = self.labels.get_mut(&self.block).unwrap();
                current_block.push(Inst::new_jump(ByteCode::JUMP, cond_lbl.clone()));

                self.block = cont_lbl;
            }
            Stmt::FunDecl(id, params, _, body) => {
                for param in params {
                    if let Expr::Identifier(id) = &*param.expr {
                        self.alloc.define(id, param);
                    }
                }
                let parent_lbl = self.block.clone();
                let fun_lbl = id.clone();
                self.block = fun_lbl;
                self.visit_stmt(body);
                self.block = parent_lbl;
            }
            _ => {
                walk_stmt(self, s);
            }
        }
    }

    fn visit_expr(&mut self, e: &ExprNode) {}
}
