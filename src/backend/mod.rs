mod allocation;
mod assembly_context;

use self::assembly_context::AsmCtx;
use crate::frontend::ast::{Expr, ExprNode, Literal, Opcode, Stmt, Type};
use crate::util::{bool_to_i64, i64_to_bool};
use crate::vm::bytecode::*;
use std::cmp;
use std::collections::HashMap;

type DispatchRet = Result<Vec<MetaAsm>, &'static str>;

pub fn generate(stmts: Stmt) -> Result<Vec<Inst>, &'static str> {
    let mut ctx = AsmCtx::new();
    let insts = dispatch_stmt(&mut ctx, &stmts)?;
    let insts = bootstrap_startup(insts);
    let insts = assemble(insts, ctx);
    Ok(insts)
}

// TODO: If we want to refactor into a Trait, each make_* is a necessary defined function implemented
//  by the type that inherits this trait
fn dispatch_stmt(ctx: &mut AsmCtx, s: &Stmt) -> DispatchRet {
    match s {
        Stmt::Block(body) => make_block(ctx, body),
        Stmt::FlatBlock(body) => make_flat_block(ctx, body),
        Stmt::If(test, body, alter) => make_if(ctx, test, body, alter),
        Stmt::While(test, body) => make_while(ctx, test, body),
        Stmt::Return(value) => make_return(ctx, value),
        Stmt::Declaration(id) => make_declaration(ctx, id),
        Stmt::FunDecl(id, args, ret, body) => make_fun(ctx, id, args, ret, body),
        Stmt::Assignment(lhs, rhs) => make_assignment(ctx, lhs, rhs),
    }
}

fn dispatch_expr(ctx: &mut AsmCtx, e: &ExprNode, l_value: bool) -> DispatchRet {
    match &*e.expr {
        Expr::Identifier(id) => make_identifier(ctx, id, l_value),
        Expr::Literal(lit) => make_literal(ctx, lit),
        Expr::BinaryOp(lhs, op, rhs) => make_binary(ctx, lhs, op, rhs),
        Expr::UnaryOp(op, rhs) => make_unary(ctx, op, rhs),
        Expr::FunCall(id, args) => make_funcall(ctx, id, args),
    }
}

fn make_block(ctx: &mut AsmCtx, body: &Vec<Box<Stmt>>) -> DispatchRet {
    ctx.push_scope();
    let mut insts = Vec::new();
    for b in body {
        insts.extend(dispatch_stmt(ctx, b)?);
    }
    ctx.pop_scope();
    Ok(insts)
}

fn make_flat_block(ctx: &mut AsmCtx, body: &Vec<Box<Stmt>>) -> DispatchRet {
    let mut insts = Vec::new();
    for b in body {
        insts.extend(dispatch_stmt(ctx, b)?);
    }
    Ok(insts)
}

fn make_if(ctx: &mut AsmCtx, test: &ExprNode, body: &Box<Stmt>, alter: &Box<Stmt>) -> DispatchRet {
    let alter_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let test = dispatch_expr(ctx, test, false)?;
    let body = dispatch_stmt(ctx, body)?;
    let alter = dispatch_stmt(ctx, alter)?;

    let mut insts = Vec::new();
    insts.extend(test);
    insts.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMPZ,
        alter_lbl.clone(),
    )));
    insts.extend(body);
    insts.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMP,
        cont_lbl.clone(),
    )));
    insts.push(MetaAsm::Lbl(alter_lbl.clone()));
    insts.extend(alter);
    insts.push(MetaAsm::Lbl(cont_lbl.clone()));
    Ok(insts)
}

fn make_while(ctx: &mut AsmCtx, test: &ExprNode, body: &Box<Stmt>) -> DispatchRet {
    let test_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let test = dispatch_expr(ctx, test, false)?;
    let body = dispatch_stmt(ctx, body)?;

    let mut insts = Vec::new();
    insts.push(MetaAsm::Lbl(test_lbl.clone()));
    insts.extend(test);
    insts.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMPZ,
        cont_lbl.clone(),
    )));
    insts.extend(body);
    insts.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::JUMP,
        test_lbl.clone(),
    )));
    insts.push(MetaAsm::Lbl(cont_lbl.clone()));
    Ok(insts)
}

fn make_return(ctx: &mut AsmCtx, value: &Option<ExprNode>) -> DispatchRet {
    let mut insts = Vec::new();
    match value {
        Some(rhs) => {
            let rhs = dispatch_expr(ctx, rhs, false)?;
            insts.extend(rhs);
            insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, -3)));
            insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
            insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));
        }
        None => {
            insts.push(MetaAsm::Inst(MetaInst::new_data(
                ByteCode::RET,
                ctx.args as i64 + 2,
            )));
        }
    };

    Ok(insts)
}

fn make_declaration(ctx: &mut AsmCtx, id: &String) -> DispatchRet {
    ctx.define(id);
    Ok(Vec::new())
}

fn make_fun(
    ctx: &mut AsmCtx,
    id: &String,
    args: &Vec<ExprNode>,
    ret: &Type,
    body: &Box<Stmt>,
) -> DispatchRet {
    ctx.add_fun(id);
    ctx.args = args.len();
    let mut insts = Vec::new();
    insts.push(MetaAsm::Lbl(id.clone()));

    for var in args {
        if let Expr::Identifier(id) = &*var.expr {
            ctx.define_arg(&id);
        }
    }

    insts.push(MetaAsm::Inst(MetaInst::new_local_alloc(
        ByteCode::ALLOC,
        id.clone(),
    )));

    insts.extend(dispatch_stmt(ctx, body)?);
    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));

    ctx.pop_fun();
    Ok(insts)
}

fn make_assignment(ctx: &mut AsmCtx, lhs: &ExprNode, rhs: &ExprNode) -> DispatchRet {
    let rhs = dispatch_expr(ctx, rhs, false)?;
    let lhs = dispatch_expr(ctx, lhs, true)?;

    let mut insts = Vec::new();
    insts.extend(rhs);
    insts.extend(lhs);
    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));

    Ok(insts)
}

fn make_identifier(ctx: &mut AsmCtx, id: &String, l_value: bool) -> DispatchRet {
    let mut insts = Vec::new();
    let offset = ctx.get(id)? as i64;
    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, offset)));
    if !l_value {
        insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
    }
    Ok(insts)
}

fn make_literal(_ctx: &mut AsmCtx, lit: &Literal) -> DispatchRet {
    let insts = match lit {
        Literal::Number(num) => vec![MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, *num))],
        Literal::Boolean(bool) => vec![MetaAsm::Inst(MetaInst::new_data(
            ByteCode::LOADC,
            bool_to_i64(*bool),
        ))],
        _ => Vec::new(),
    };
    Ok(insts)
}

fn make_binary(ctx: &mut AsmCtx, lhs: &ExprNode, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    let lhs = dispatch_expr(ctx, lhs, false)?;
    let rhs = dispatch_expr(ctx, rhs, false)?;
    let op: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
    let mut insts = Vec::new();
    insts.extend(lhs);
    insts.extend(rhs);
    insts.push(MetaAsm::Inst(MetaInst::new_inst(op)));
    Ok(insts)
}

fn make_unary(ctx: &mut AsmCtx, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    let rhs = dispatch_expr(ctx, rhs, false)?;
    let op: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
    let mut insts = Vec::new();
    insts.extend(rhs);
    insts.push(MetaAsm::Inst(MetaInst::new_inst(op)));
    Ok(insts)
}

fn make_funcall(ctx: &mut AsmCtx, id: &String, args: &Vec<ExprNode>) -> DispatchRet {
    let mut insts = Vec::new();

    // Allocate space for the return value
    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::ALLOC, 1)));

    // Push arguments to stack from right to left (e.g. backwards)
    for arg in args.iter().rev() {
        let rhs = dispatch_expr(ctx, arg, false)?;
        insts.extend(rhs);
    }

    // Save FP
    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)));
    // Calculate the starting address of the called function
    insts.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::LOADC,
        id.clone(),
    )));
    // Pass control over to the called function
    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)));

    // Once the called function returns, slide the return value to the top of the stack
    insts.push(MetaAsm::Inst(MetaInst::new_data(
        ByteCode::SLIDE,
        args.len() as i64,
    )));

    Ok(insts)
}

fn bootstrap_startup(insts: Vec<MetaAsm>) -> Vec<MetaAsm> {
    let mut prelude = vec![
        MetaAsm::Inst(MetaInst::new_data(ByteCode::ALLOC, 1)),
        MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)),
        MetaAsm::Inst(MetaInst::new_label(ByteCode::LOADC, "main".to_string())),
        MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)),
        MetaAsm::Inst(MetaInst::new_inst(ByteCode::HALT)),
    ];
    prelude.extend(insts);
    prelude
}

fn assemble(insts: Vec<MetaAsm>, ctx: AsmCtx) -> Vec<Inst> {
    let mut labels = HashMap::new();
    //    let mut new_insts = Vec::new();
    let insts = insts.iter().fold(Vec::new(), |mut v, i| match i {
        MetaAsm::Inst(i) => {
            v.push(i);
            v
        }
        MetaAsm::Lbl(l) => {
            labels.insert(l, v.len());
            v
        }
    });
    insts
        .iter()
        .map(|i| match i.var.clone() {
            MetaVar::Data(d) => Inst::new_data(i.inst.clone(), d),
            MetaVar::Label(l) => Inst::new_data(i.inst.clone(), *labels.get(&l).unwrap() as i64),
            MetaVar::LocalAlloc(id) => {
                Inst::new_data(i.inst.clone(), *ctx.locals.get(&id).unwrap() as i64)
            }
            MetaVar::None => Inst::new_inst(i.inst.clone()),
        })
        .collect()
}
