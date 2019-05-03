mod allocation;
mod assembly_context;

use self::assembly_context::AsmCtx;
use crate::frontend::ast::{Expr, ExprNode, Literal, Opcode, Stmt, Type};
use crate::frontend::visitor::{walk_expr, walk_stmt, Visitor};
use crate::vm::bytecode::*;
use std::collections::HashMap;

// TODO: Split off bool to t conversion funcs
use crate::vm::VirtualMachine;

type DispatchRet = Result<Vec<MetaAsm>, &'static str>;

pub fn generate(stmts: Stmt) -> Result<Vec<Inst>, &'static str> {
    let mut ctx = AsmCtx::new();
    let inst = dispatch_stmt(&mut ctx, &stmts)?;
    for i in &inst {
        println!("{:?}", i);
    }
    let inst = assemble(inst, ctx);

    Ok(inst)
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

// TODO: May be able to use a try_fold() iterator on this one
fn make_block(mut ctx: &mut AsmCtx, body: &Vec<Box<Stmt>>) -> DispatchRet {
    let mut inst = Vec::new();
    ctx.push_scope();
    for b in body {
        let sub_inst = dispatch_stmt(ctx, b)?;
        inst.extend(sub_inst);
    }
    ctx.pop_scope();
    Ok(inst)
}

fn make_flat_block(mut ctx: &mut AsmCtx, body: &Vec<Box<Stmt>>) -> DispatchRet {
    let mut inst = Vec::new();
    for b in body {
        let sub_inst = dispatch_stmt(ctx, b)?;
        inst.extend(sub_inst);
    }
    Ok(inst)
}

fn make_if(
    mut ctx: &mut AsmCtx,
    test: &ExprNode,
    body: &Box<Stmt>,
    alter: &Box<Stmt>,
) -> DispatchRet {
    let alter_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let test = dispatch_expr(ctx, test, false)?;
    let body = dispatch_stmt(ctx, body)?;
    let alter = dispatch_stmt(ctx, alter)?;

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
    Ok(inst)
}

fn make_while(mut ctx: &mut AsmCtx, test: &ExprNode, body: &Box<Stmt>) -> DispatchRet {
    let test_lbl = ctx.next_label();
    let cont_lbl = ctx.next_label();

    let test = dispatch_expr(ctx, test, false)?;
    let body = dispatch_stmt(ctx, body)?;

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
    Ok(inst)
}

fn make_return(ctx: &mut AsmCtx, value: &Option<ExprNode>) -> DispatchRet {
    Err("Not Implemented")
}

fn make_declaration(mut ctx: &mut AsmCtx, id: &String) -> DispatchRet {
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
    let mut inst = Vec::new();
    inst.push(MetaAsm::Lbl(id.clone()));

    for var in args {
        if let Expr::Identifier(id) = &*var.expr {
            ctx.define(&id);
        }
    }

    inst.push(MetaAsm::Inst(MetaInst::new_local_alloc(
        ByteCode::ALLOC,
        id.clone(),
    )));

    inst.extend(dispatch_stmt(ctx, body)?);
    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));

    ctx.pop_fun();
    Ok(inst)
}

fn make_assignment(ctx: &mut AsmCtx, lhs: &ExprNode, rhs: &ExprNode) -> DispatchRet {
    let rhs = dispatch_expr(ctx, rhs, false)?;
    let lhs = dispatch_expr(ctx, lhs, true)?;

    let mut inst = Vec::new();
    inst.extend(rhs);
    inst.extend(lhs);
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));

    Ok(inst)
}

fn make_identifier(ctx: &mut AsmCtx, id: &String, l_value: bool) -> DispatchRet {
    let mut inst = Vec::new();
    let offset = ctx.get(id)? as i64;
    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, offset)));
    if !l_value {
        inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
    }
    Ok(inst)
}

fn make_literal(ctx: &mut AsmCtx, lit: &Literal) -> DispatchRet {
    let inst = match lit {
        Literal::Number(num) => vec![MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, *num))],
        Literal::Boolean(bool) => vec![MetaAsm::Inst(MetaInst::new_data(
            ByteCode::LOADC,
            VirtualMachine::bool_to_t(*bool),
        ))],
        _ => Vec::new(),
    };
    Ok(inst)
}

fn make_binary(ctx: &mut AsmCtx, lhs: &ExprNode, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    let lhs = dispatch_expr(ctx, lhs, false)?;
    let rhs = dispatch_expr(ctx, rhs, false)?;
    let op: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
    let mut inst = Vec::new();
    inst.extend(lhs);
    inst.extend(rhs);
    inst.push(MetaAsm::Inst(MetaInst::new_inst(op)));
    Ok(inst)
}

fn make_unary(ctx: &mut AsmCtx, op: &Opcode, rhs: &ExprNode) -> DispatchRet {
    Err("Not Implemented")
}

fn make_funcall(ctx: &mut AsmCtx, id: &String, args: &Vec<ExprNode>) -> DispatchRet {
    let mut inst = Vec::new();
    for arg in args.iter().rev() {
        let lhs = dispatch_expr(ctx, arg, false)?;
        inst.extend(lhs);
    }

    //    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 0)));
    //    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 0)));
    //    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 0)));
    //    inst.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 0)));
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)));
    inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::LOADC,
        id.clone(),
    )));
    inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)));

    Ok(inst)
}

fn assemble(inst: Vec<MetaAsm>, ctx: AsmCtx) -> Vec<Inst> {
    let mut machine_inst = Vec::new();
    let mut locations: HashMap<String, usize> = HashMap::new();

    let mut pre_inst = Vec::new();
    pre_inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)));
    pre_inst.push(MetaAsm::Inst(MetaInst::new_label(
        ByteCode::LOADC,
        "main".to_string(),
    )));
    pre_inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)));
    pre_inst.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::HALT)));

    pre_inst.extend(inst);
    let inst = pre_inst;

    let mut meta_inst = Vec::new();
    for i in inst {
        match i {
            MetaAsm::Inst(i) => meta_inst.push(i),
            MetaAsm::Lbl(s) => {
                locations.insert(s, meta_inst.len());
            }
        }
    }

    println!("Locations: {:?}", &locations);

    for i in meta_inst {
        match i.var {
            MetaVar::Data(d) => machine_inst.push(Inst::new_data(i.inst, d)),
            MetaVar::Label(s) => {
                machine_inst.push(Inst::new_data(
                    i.inst,
                    *locations.get(&s).clone().unwrap() as i64,
                ));
            }
            MetaVar::LocalAlloc(id) => {
                let locals = ctx.locals.get(&id).unwrap();
                machine_inst.push(Inst::new_data(i.inst, *locals as i64))
            }
            MetaVar::None => machine_inst.push(Inst::new_inst(i.inst)),
        }
    }

    //    machine_inst.push(Inst::new_inst(ByteCode::HALT));
    machine_inst
}
