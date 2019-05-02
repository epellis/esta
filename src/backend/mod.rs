mod allocation;
mod assembly_context;

use self::assembly_context::AsmCtx;
use crate::frontend::ast::{Expr, ExprNode, Literal, Opcode, Stmt, Type};
use crate::frontend::visitor::{walk_expr, walk_stmt, Visitor};
use crate::vm::bytecode::*;
use std::collections::HashMap;

// TODO: Split off bool to t conversion funcs
use crate::vm::VirtualMachine;

type DispatchRet = Result<(Vec<MetaAsm>, AsmCtx), &'static str>;

pub fn generate(stmts: Stmt) -> Result<Vec<Inst>, &'static str> {
    let (inst, ctx) = dispatch_stmt(AsmCtx::new(), &stmts)?;

    for i in inst {
        println!("{:?}", i);
    }

    Ok(Vec::new())
}

// TODO: If we want to refactor into a Trait, each make_* is a necessary defined function implemented
//  by the type that inherits this trait
fn dispatch_stmt(ctx: AsmCtx, s: &Stmt) -> DispatchRet {
    match s {
        Stmt::Block(body) => make_block(ctx, body),
        Stmt::If(test, body, alter) => make_if(ctx, test, body, alter),
        Stmt::While(test, body) => make_while(ctx, test, body),
        Stmt::Return(value) => make_return(ctx, value),
        Stmt::Declaration(id) => make_declaration(ctx, id),
        Stmt::FunDecl(id, args, ret, body) => make_fun(ctx, id, args, ret, body),
        Stmt::Assignment(lhs, rhs) => make_assignment(ctx, lhs, rhs),
    }
}

fn dispatch_expr(ctx: AsmCtx, e: &ExprNode, l_value: bool) -> DispatchRet {
    match &*e.expr {
        Expr::Identifier(id) => make_identifier(ctx, id, l_value),
        Expr::Literal(lit) => make_literal(ctx, lit),
        Expr::BinaryOp(lhs, op, rhs) => make_binary(ctx, lhs, op, rhs),
        Expr::UnaryOp(op, rhs) => make_unary(ctx, op, rhs),
        Expr::FunCall(id, args) => make_funcall(ctx, id, args),
    }
}

// TODO: May be able to use a try_fold() iterator on this one
fn make_block(mut ctx: AsmCtx, body: &Vec<Box<Stmt>>) -> DispatchRet {
    let mut inst = Vec::new();
    ctx.push_scope();
    for b in body {
        let (sub_inst, ctx) = dispatch_stmt(ctx.clone(), b)?;
        inst.extend(sub_inst);
    }
    ctx.pop_scope();
    Ok((inst, ctx))
}

fn make_if(mut ctx: AsmCtx, test: &ExprNode, body: &Box<Stmt>, alter: &Box<Stmt>) -> DispatchRet {
    let alter_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let (test, ctx) = dispatch_expr(ctx.clone(), test, false)?;
    let (body, ctx) = dispatch_stmt(ctx.clone(), body)?;
    let (alter, ctx) = dispatch_stmt(ctx.clone(), alter)?;

    let mut inst = Vec::new();
    inst.extend(test);
    inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMPZ,
        alter_lbl.clone(),
    )));
    inst.extend(body);
    inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMP,
        cont_lbl.clone(),
    )));
    inst.push(MetaAsm::Lbl(alter_lbl.clone()));
    inst.extend(alter);
    inst.push(MetaAsm::Lbl(cont_lbl.clone()));
    Ok((inst, ctx))
}

fn make_while(mut ctx: AsmCtx, test: &ExprNode, body: &Box<Stmt>) -> DispatchRet {
    let test_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let (test, ctx) = dispatch_expr(ctx.clone(), test, false)?;
    let (body, ctx) = dispatch_stmt(ctx.clone(), body)?;

    let mut inst = Vec::new();
    inst.push(MetaAsm::Lbl(test_lbl.clone()));
    inst.extend(test);
    inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMPZ,
        cont_lbl.clone(),
    )));
    inst.extend(body);
    inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMP,
        test_lbl.clone(),
    )));
    inst.push(MetaAsm::Lbl(cont_lbl.clone()));
    Ok((inst, ctx))
}

fn make_return(ctx: AsmCtx, value: &Option<ExprNode>) -> DispatchRet {
    Err("Not Implemented")
}

fn make_declaration(mut ctx: AsmCtx, id: &String) -> DispatchRet {
    ctx.define(id);
    return Ok((Vec::new(), ctx));
}

fn make_fun(
    ctx: AsmCtx,
    id: &String,
    args: &Vec<ExprNode>,
    ret: &Type,
    body: &Box<Stmt>,
) -> DispatchRet {
    Err("Not Implemented")
}

fn make_assignment(ctx: AsmCtx, lhs: &ExprNode, rhs: &ExprNode) -> DispatchRet {
    let (rhs, ctx) = dispatch_expr(ctx.clone(), rhs, false)?;
    let (lhs, ctx) = dispatch_expr(ctx.clone(), lhs, true)?;

    let mut inst = Vec::new();
    inst.extend(rhs);
    inst.extend(lhs);
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));

    Ok((inst, ctx))
}

fn make_identifier(ctx: AsmCtx, id: &String, l_value: bool) -> DispatchRet {
    let mut inst = Vec::new();
    let offset = ctx.get(id)? as i64;
    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, offset)));
    if !l_value {
        inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
    }
    Ok((inst, ctx))
}

fn make_literal(ctx: AsmCtx, lit: &Literal) -> DispatchRet {
    let inst = match lit {
        Literal::Number(num) => MetaInst::new_data(ByteCode::LOADC, *num),
        Literal::Boolean(bool) => {
            MetaInst::new_data(ByteCode::LOADC, VirtualMachine::bool_to_t(*bool))
        }
        _ => MetaInst::new_nop(),
    };
    let inst = MetaAsm::Inst(inst);
    Ok((vec![inst], ctx))
}

fn make_binary(ctx: AsmCtx, lhs: &ExprNode, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    let (lhs, ctx) = dispatch_expr(ctx.clone(), lhs, false)?;
    let (rhs, ctx) = dispatch_expr(ctx.clone(), rhs, false)?;
    let op: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
    let mut inst = Vec::new();
    inst.extend(lhs);
    inst.extend(rhs);
    inst.push(MetaAsm::Inst(MetaInst::new_inst(op)));
    Ok((inst, ctx))
}

fn make_unary(ctx: AsmCtx, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    Err("Not Implemented")
}

fn make_funcall(ctx: AsmCtx, id: &String, args: &Vec<ExprNode>) -> DispatchRet {
    Err("Not Implemented")
}
