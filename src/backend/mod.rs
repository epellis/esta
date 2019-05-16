pub mod program;

use self::program::{AsmCtx, Program};
use crate::frontend::ast::*;
use crate::middleend::MetaData;
use crate::util::fold::*;
use crate::util::{bool_to_i64, string_hash32};
use crate::vm::bytecode::*;
use std::collections::HashMap;

pub fn generate(stmts: Stmt, md: MetaData) -> Result<Program, &'static str> {
    //    let mut ctx = AsmCtx::new(md);
    //    let insts = dispatch_stmt(&mut ctx, &stmts)?;
    //    let insts = bootstrap_startup(insts);
    //    let insts = assemble(insts, &ctx);
    //    let data_segment = ctx.assemble_data_segment();
    //    Ok((insts, data_segment))
    //    Ok((Default::default()))
    Assembler::assemble(&stmts)
}

pub struct Assembler;

impl Assembler {
    pub fn assemble(body: &Stmt) -> Result<Program, &'static str> {
        let ctx: AsmCtx = Default::default();
        let ctx = Assembler::fold_stmt(&ctx, body).ok_or("Failed to assemble program")?;

        let instructions = ctx.assemble();

        Err("Not implemented")
    }
}

impl Fold for Assembler {
    type UpT = AsmCtx;
    type DownT = AsmCtx;

    fn reduce(children: Vec<Option<Self::UpT>>) -> Option<Self::UpT> {
        if children.len() > 0 {
            let base = children.last().cloned().unwrap().unwrap().base;
            let suffix = children.last().cloned().unwrap().unwrap().suffix;
            let blocks = children
                .into_iter()
                .flatten()
                .map(|c: AsmCtx| -> Vec<MetaInst> { c.blocks })
                .flatten()
                .collect();
            Some(AsmCtx {
                base,
                blocks,
                suffix,
            })
        } else {
            None
        }
    }

    fn fold_block(down: &Self::DownT, body: &Vec<Box<Stmt>>, is_scope: &bool) -> Option<Self::UpT> {
        let mut block = Vec::new();
        block.push(MetaInst::ByteCode(ByteCode::PUSHF));
        block.push(MetaInst::Number(0));

        let children = body.iter().map(|b| Self::fold_stmt(down, b)).collect();

        if let Some(mut child) = Assembler::reduce(children) {
            block.extend(child.blocks);
            block.push(MetaInst::ByteCode(ByteCode::POPF));
            child.blocks = block;
            return Some(child);
        } else {
            block.push(MetaInst::ByteCode(ByteCode::POPF));
            let mut ctx = down.clone();
            ctx.blocks = block;
            return Some(ctx);
        }
    }
}

//fn dispatch_stmt(ctx: &mut AsmCtx, s: &Stmt) -> DispatchRet {
//    match s {
//        Stmt::Block(body, is_scope) => make_block(ctx, body, is_scope),
//        Stmt::If(test, body, alter) => make_if(ctx, test, body, alter),
//        Stmt::While(test, body) => make_while(ctx, test, body),
//        Stmt::Return(value) => make_return(ctx, value),
//        Stmt::Declaration(id) => make_declaration(ctx, id),
//        Stmt::FunDecl(id, args, body) => make_fun(ctx, id, &args, body),
//        Stmt::Assignment(lhs, rhs) => make_assignment(ctx, lhs, rhs),
//        Stmt::Struct(id, fields) => make_struct(ctx, id, fields),
//    }
//}
//
//fn dispatch_expr(ctx: &mut AsmCtx, e: &Expr, l_value: bool) -> DispatchRet {
//    match e {
//        Expr::Id(id) => make_identifier(ctx, id, l_value),
//        Expr::Literal(lit) => make_literal(ctx, &lit),
//        Expr::BinaryOp(lhs, op, rhs) => make_binary(ctx, &lhs, &op, &rhs),
//        Expr::UnaryOp(op, rhs) => make_unary(ctx, &op, rhs),
//        Expr::FunCall(id, args) => make_funcall(ctx, &id, &args),
//        Expr::List(xs) => make_list(ctx, &xs),
//        Expr::Dot(this, action) => make_dot(ctx, &this, &action),
//    }
//}
//
//fn make_block(ctx: &mut AsmCtx, body: &Vec<Box<Stmt>>, is_scope: &bool) -> DispatchRet {
//    if *is_scope {
//        ctx.push_scope();
//    }
//    let mut insts = Vec::new();
//    for b in body {
//        insts.extend(dispatch_stmt(ctx, b)?);
//    }
//    if *is_scope {
//        ctx.pop_scope();
//    }
//    Ok(insts)
//}
//
//fn make_if(ctx: &mut AsmCtx, test: &Box<Expr>, body: &Box<Stmt>, alter: &Box<Stmt>) -> DispatchRet {
//    let alter_lbl = ctx.next_label();
//    let cont_lbl = ctx.next_label();
//
//    let test = dispatch_expr(ctx, test, false)?;
//    let body = dispatch_stmt(ctx, body)?;
//    let alter = dispatch_stmt(ctx, alter)?;
//
//    let mut insts = Vec::new();
//    insts.extend(test);
//    insts.push(MetaAsm::Inst(MetaInst::new_label(
//        ByteCode::JUMPZ,
//        alter_lbl.clone(),
//    )));
//    insts.extend(body);
//    insts.push(MetaAsm::Inst(MetaInst::new_label(
//        ByteCode::JUMP,
//        cont_lbl.clone(),
//    )));
//    insts.push(MetaAsm::Lbl(alter_lbl.clone()));
//    insts.extend(alter);
//    insts.push(MetaAsm::Lbl(cont_lbl.clone()));
//    Ok(insts)
//}
//
//fn make_while(ctx: &mut AsmCtx, test: &Box<Expr>, body: &Box<Stmt>) -> DispatchRet {
//    let test_lbl = ctx.next_label();
//    let cont_lbl = ctx.next_label();
//
//    let test = dispatch_expr(ctx, test, false)?;
//    let body = dispatch_stmt(ctx, body)?;
//
//    let mut insts = Vec::new();
//    insts.push(MetaAsm::Lbl(test_lbl.clone()));
//    insts.extend(test);
//    insts.push(MetaAsm::Inst(MetaInst::new_label(
//        ByteCode::JUMPZ,
//        cont_lbl.clone(),
//    )));
//    insts.extend(body);
//    insts.push(MetaAsm::Inst(MetaInst::new_label(
//        ByteCode::JUMP,
//        test_lbl.clone(),
//    )));
//    insts.push(MetaAsm::Lbl(cont_lbl.clone()));
//    Ok(insts)
//}
//
//fn make_return(ctx: &mut AsmCtx, value: &Option<Box<Expr>>) -> DispatchRet {
//    let mut insts = Vec::new();
//    match value {
//        Some(rhs) => {
//            let rhs = dispatch_expr(ctx, rhs, false)?;
//            insts.extend(rhs);
//            insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, -3)));
//            insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
//            insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));
//        }
//        None => {
//            insts.push(MetaAsm::Inst(MetaInst::new_data(
//                ByteCode::RET,
//                ctx.args as i64 + 2,
//            )));
//        }
//    };
//
//    Ok(insts)
//}
//
//fn make_declaration(ctx: &mut AsmCtx, id: &Identifier) -> DispatchRet {
//    // TODO: Make this better
//    ctx.define(&id.id);
//    Ok(Vec::new())
//}
//
//fn make_fun(
//    ctx: &mut AsmCtx,
//    id: &Identifier,
//    args: &Vec<Identifier>,
//    body: &Box<Stmt>,
//) -> DispatchRet {
//    ctx.add_fun(&id.id);
//    ctx.args = args.len(); // TODO: Do we need this?
//    let mut insts = Vec::new();
//    insts.push(MetaAsm::Lbl(id.id.clone()));
//
//    for id in args {
//        ctx.define_arg(&id.id);
//    }
//
//    insts.push(MetaAsm::Inst(MetaInst::new_local_alloc(
//        ByteCode::ALLOC,
//        id.id.clone(),
//    )));
//
//    insts.extend(dispatch_stmt(ctx, body)?);
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));
//
//    ctx.pop_fun();
//    Ok(insts)
//}
//
//fn make_assignment(ctx: &mut AsmCtx, lhs: &Box<Expr>, rhs: &Box<Expr>) -> DispatchRet {
//    let rhs = dispatch_expr(ctx, rhs, false)?;
//    let lhs = dispatch_expr(ctx, lhs, true)?;
//
//    let mut insts = Vec::new();
//    insts.extend(rhs);
//    insts.extend(lhs);
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));
//
//    Ok(insts)
//}
//
//// Create a new constructor function for this struct which will allocate space
//// on the heap with the right tags and return the address to the caller
//fn make_struct(ctx: &mut AsmCtx, id: &String, fields: &Vec<Identifier>) -> DispatchRet {
//    ctx.add_fun(id);
//    let mut insts = Vec::new();
//
//    let esta_struct = ctx.get_esta_struct(id).unwrap();
//    insts.push(MetaAsm::Lbl(id.clone()));
//
//    // Alloc new space on the heap to store the struct
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::LOADC,
//        esta_struct.size as i64,
//    )));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::NEW)));
//
//    // Set the first space to the struct's tag
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::LOADC,
//        esta_struct.tag as i64,
//    )));
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::LOADRC,
//        0 as i64,
//    )));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 0 as i64)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::ADD)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STOREH)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));
//
//    // Set the second space to the struct's size
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::LOADC,
//        esta_struct.size as i64,
//    )));
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::LOADRC,
//        0 as i64,
//    )));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, 1 as i64)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::ADD)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STOREH)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::POP)));
//
//    // Return the starting address of the struct
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, -3)));
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::STORE)));
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::RET, 2)));
//
//    ctx.pop_fun();
//    Ok(insts)
//}
//
//fn make_identifier(ctx: &mut AsmCtx, id: &Identifier, l_value: bool) -> DispatchRet {
//    let mut insts = Vec::new();
//    debug!("Making: {}", &id.id);
//    let offset = ctx.get(&id.id)? as i64;
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, offset)));
//    if !l_value {
//        insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
//    }
//    Ok(insts)
//}
//
//fn make_literal(_ctx: &mut AsmCtx, lit: &Literal) -> DispatchRet {
//    let insts = match lit {
//        Literal::Number(num) => vec![MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADC, *num))],
//        Literal::Boolean(bool) => vec![MetaAsm::Inst(MetaInst::new_data(
//            ByteCode::LOADC,
//            bool_to_i64(*bool),
//        ))],
//        _ => Vec::new(),
//    };
//    Ok(insts)
//}
//
//fn make_binary(ctx: &mut AsmCtx, lhs: &Box<Expr>, op: &Opcode, rhs: &Box<Expr>) -> DispatchRet {
//    let lhs = dispatch_expr(ctx, lhs, false)?;
//    let rhs = dispatch_expr(ctx, rhs, false)?;
//    let op: ByteCode = BIN_OP_TO_BYTE.get(op).unwrap().clone();
//    let mut insts = Vec::new();
//    insts.extend(lhs);
//    insts.extend(rhs);
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(op)));
//    Ok(insts)
//}
//
//fn make_unary(ctx: &mut AsmCtx, op: &Opcode, rhs: &Box<Expr>) -> DispatchRet {
//    let rhs = dispatch_expr(ctx, rhs, false)?;
//    let op: ByteCode = UN_OP_TO_BYTE.get(op).unwrap().clone();
//    let mut insts = Vec::new();
//    insts.extend(rhs);
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(op)));
//    Ok(insts)
//}
//
//fn make_funcall(ctx: &mut AsmCtx, id: &String, args: &Vec<Expr>) -> DispatchRet {
//    let mut insts = Vec::new();
//
//    // Allocate space for the return value
//    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::ALLOC, 1)));
//
//    // Push arguments to stack from right to left (e.g. backwards)
//    for arg in args.iter().rev() {
//        let rhs = dispatch_expr(ctx, arg, false)?;
//        insts.extend(rhs);
//    }
//
//    // Save FP
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)));
//    // Calculate the starting address of the called function
//    insts.push(MetaAsm::Inst(MetaInst::new_label(
//        ByteCode::LOADC,
//        id.clone(),
//    )));
//    // Pass control over to the called function
//    insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)));
//
//    // Once the called function returns, slide the return value to the top of the stack
//    insts.push(MetaAsm::Inst(MetaInst::new_data(
//        ByteCode::SLIDE,
//        args.len() as i64,
//    )));
//
//    Ok(insts)
//}
//
//fn make_list(ctx: &mut AsmCtx, xs: &Vec<Box<Expr>>) -> DispatchRet {
//    Ok(Vec::new())
//}
//
//fn make_dot(ctx: &mut AsmCtx, this: &Identifier, action: &Box<Expr>) -> DispatchRet {
//    // TODO: Right now, this only works for accessing values
//    match *action.clone() {
//        Expr::Id(identifier) => {}
//        _ => {}
//    }
//    //    let mut insts = Vec::new();
//    //    debug!("Making: {}", &id.id);
//    //    let offset = ctx.get(&id.id)? as i64;
//    //    insts.push(MetaAsm::Inst(MetaInst::new_data(ByteCode::LOADRC, offset)));
//    //    if !l_value {
//    //        insts.push(MetaAsm::Inst(MetaInst::new_inst(ByteCode::LOAD)));
//    //    }
//    //    Ok(insts)
//
//    Ok(Vec::new())
//}
//
//fn bootstrap_startup(insts: Vec<MetaAsm>) -> Vec<MetaAsm> {
//    let mut prelude = vec![
//        MetaAsm::Inst(MetaInst::new_data(ByteCode::ALLOC, 1)),
//        MetaAsm::Inst(MetaInst::new_inst(ByteCode::MARK)),
//        MetaAsm::Inst(MetaInst::new_label(ByteCode::LOADC, "main".to_string())),
//        MetaAsm::Inst(MetaInst::new_inst(ByteCode::CALL)),
//        MetaAsm::Inst(MetaInst::new_inst(ByteCode::HALT)),
//    ];
//    prelude.extend(insts);
//    prelude
//}
//
//fn assemble(insts: Vec<MetaAsm>, ctx: &AsmCtx) -> Vec<Inst> {
//    let mut labels = HashMap::new();
//    let insts = insts.iter().fold(Vec::new(), |mut v, i| match i {
//        MetaAsm::Inst(i) => {
//            v.push(i);
//            v
//        }
//        MetaAsm::Lbl(l) => {
//            labels.insert(l, v.len());
//            v
//        }
//    });
//    insts
//        .iter()
//        .map(|i| match i.var.clone() {
//            MetaVar::Data(d) => Inst::new_data(i.inst.clone(), d),
//            MetaVar::Label(l) => Inst::new_data(
//                i.inst.clone(),
//                *labels.get(&l).expect("Couldn't find label") as i64,
//            ),
//            MetaVar::LocalAlloc(id) => Inst::new_data(
//                i.inst.clone(),
//                *ctx.locals.get(&id).expect("Couldn't find local") as i64,
//            ),
//            MetaVar::None => Inst::new_inst(i.inst.clone()),
//        })
//        .collect()
//}
