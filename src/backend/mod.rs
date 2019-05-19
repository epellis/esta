pub mod program;

use self::program::{AsmCtx, Program};
use crate::frontend::ast::*;
use crate::middleend::MetaData;
use crate::util::fold::*;
use crate::util::{bool_to_i64, string_hash32};
use crate::vm::bytecode::*;
use crate::vm::{EstaData, EstaType};
use itertools::Itertools;
use std::collections::HashMap;

pub fn generate(stmts: Stmt, md: MetaData) -> Result<Program, &'static str> {
    Assembler::assemble(&stmts)
}

pub struct Assembler;

impl Assembler {
    pub fn assemble(body: &Stmt) -> Result<Program, &'static str> {
        let ctx: AsmCtx = Default::default();
        let ctx = Assembler::fold_stmt(&ctx, body).ok_or("Failed to assemble program")?;

        let instructions = ctx.assemble();
        Ok(instructions)

        //        Err("Not implemented")
    }
}

impl Fold for Assembler {
    type UpT = AsmCtx;
    type DownT = AsmCtx;

    /// To reduce, collect all program blocks into a single vector and adopt the
    /// last (aka latest) base and suffix as it's own.
    fn reduce(children: Vec<Option<Self::UpT>>) -> Option<Self::UpT> {
        let children = children.into_iter().flatten().collect::<Vec<Self::UpT>>();

        if children.len() > 0 {
            let last_ctx = children.last().cloned().unwrap();
            let declarations = children
                .iter()
                .cloned()
                .map(|c: AsmCtx| -> Vec<String> { c.declarations })
                .flatten()
                .collect();
            let blocks = children
                .into_iter()
                .map(|c: AsmCtx| -> Vec<MetaInst> { c.blocks })
                .flatten()
                .collect();
            Some(AsmCtx {
                blocks,
                declarations,
                ..last_ctx
            })
        } else {
            None
        }
    }

    fn fold_block(down: &Self::DownT, body: &Vec<Box<Stmt>>, is_scope: &bool) -> Option<Self::UpT> {
        let children = body.iter().map(|b| Self::fold_stmt(down, b)).collect();

        if let Some(mut child) = Assembler::reduce(children) {
            let mut block = Vec::new();
            block.push(MetaInst::ByteCode(ByteCode::PUSHE));
            block.push(MetaInst::Number(child.declarations.len() as i16));
            for id in child.declarations {
                block.push(MetaInst::Declaration(id.clone()));
            }
            child.declarations = Vec::new();

            block.extend(child.blocks);
            block.push(MetaInst::ByteCode(ByteCode::POPE));
            child.blocks = block;
            return Some(child);
        } else {
            return Some(Default::default());
        }
    }

    //    fn fold_if(
    //        down: &Self::DownT,
    //        test: &Box<Expr>,
    //        body: &Box<Stmt>,
    //        alter: &Box<Stmt>,
    //    ) -> Option<Self::UpT> {
    //        let mut down = down.clone();
    //        let alter_lbl = down.next_label();
    //        let cont_lbl = down.next_label();
    //
    //        let test = Self::fold_expr(&down, test).unwrap_or_default();
    //        let body = Self::fold_stmt(&down, body).unwrap_or_default();
    //        let alter = Self::fold_stmt(&down, alter).unwrap_or_default();
    //
    //        let base = alter.base.clone();
    //        let suffix = alter.suffix.clone();
    //
    //        // Test Block
    //        let mut blocks = Vec::new();
    //        blocks.extend(test.blocks);
    //        // TODO: It is inelegant that we need to use labels as jump arguments and as
    //        //  undetermined points to jump to
    //        blocks.push(MetaInst::ByteCode(ByteCode::JUMPF));
    //        blocks.push(MetaInst::Label(alter_lbl.clone()));
    //
    //        // Body Block
    //        blocks.extend(body.blocks);
    //        blocks.push(MetaInst::ByteCode(ByteCode::JUMP));
    //        blocks.push(MetaInst::Label(cont_lbl.clone()));
    //
    //        // Alternate Block
    //        blocks.push(MetaInst::Label(alter_lbl.clone()));
    //        blocks.extend(alter.blocks);
    //
    //        // Continuation Block
    //        blocks.push(MetaInst::Label(cont_lbl.clone()));
    //
    //        // TODO: Replace with ..alter constructor
    //        Some(AsmCtx {
    //            base,
    //            blocks,
    //            suffix,
    //        })
    //    }

    // A declaration sets up a scope-local variable
    fn fold_declaration(down: &Self::DownT, id: &Identifier) -> Option<Self::UpT> {
        let mut down = down.clone();
        down.declarations.push(id.id.clone());
        Some(down)
    }

    // The LHS will become a location in the environment (e.g. 0, 0). The RHS is a value,
    // which will be stored at this location.
    fn fold_assignment(down: &Self::DownT, lhs: &Box<Expr>, rhs: &Box<Expr>) -> Option<Self::UpT> {
        let children = [rhs, lhs]
            .iter()
            .map(|e| Self::fold_expr(down, e))
            .collect();

        let mut ctx = Self::reduce(children).unwrap();
        ctx.blocks.push(MetaInst::ByteCode(ByteCode::STOREV));
        // TODO: StoreV needs to get the arguments as to where to store the value here
        ctx.blocks.push(MetaInst::ByteCode(ByteCode::POP));
        Some(ctx)
    }

    // Pushes an identifier instruction, which will resolve to the location of the declared
    // identifier
    fn fold_id(down: &Self::DownT, id: &Identifier) -> Option<Self::UpT> {
        let mut ctx = down.clone();
        ctx.blocks.push(MetaInst::Identifier(id.id.clone()));
        Some(ctx)
    }

    fn fold_literal(down: &Self::DownT, lit: &Literal) -> Option<Self::UpT> {
        let mut ctx = down.clone();

        // TODO: It is a bit strange that there are two different literal types, and
        //  in the future, this should be combined into one.
        let data = match lit {
            Literal::Number(n) => EstaData::new_int(*n as i32),
            Literal::Boolean(b) => EstaData::new_bool(*b),
            _ => Default::default(),
        };

        ctx.blocks.push(MetaInst::Const(data));

        Some(ctx)
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
