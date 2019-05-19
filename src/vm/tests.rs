use crate::backend::program::*;
use crate::vm::bytecode::*;
use crate::vm::*;
use std::collections::HashMap;

extern crate env_logger;
extern crate log;

fn run_instructions(insts: &Vec<MetaInst>, consts: Vec<EstaData>) -> VirtualMachine {
    let _ = env_logger::builder()
        .default_format_timestamp(false)
        .try_init();

    // Use the AsmCtx method to create a new program
    let ctx = AsmCtx::new_metainst(insts.clone());
    let mut prog: Program = ctx.assemble();
    prog.consts = consts;

    run_program(prog)
}

fn run_program(prog: Program) -> VirtualMachine {
    let mut vm = VirtualMachine::new(prog);
    let res = vm.run();
    if let Err(e) = res {
        error!("Test finished with error: {}", e);
    }
    assert!(res.is_ok());
    vm
}

#[test]
fn test_vm_halt() {
    let instructions = vec![MetaInst::ByteCode(ByteCode::HALT)];

    run_instructions(&instructions, Vec::new());
}

#[test]
fn test_vm_add() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::ADD),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let consts = vec![EstaData::new_int(4)];

    run_instructions(&instructions, consts);
}

#[test]
fn test_vm_loadc() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let consts = vec![EstaData::new_int(4)];

    run_instructions(&instructions, consts);
}

#[test]
fn test_vm_loadv() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::PUSHE),
        MetaInst::Number(1),
        MetaInst::ByteCode(ByteCode::LOADV),
        MetaInst::Number(0),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let consts = vec![EstaData::new_int(4)];

    run_instructions(&instructions, consts);
}

#[test]
fn test_vm_storev() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::PUSHE),
        MetaInst::Number(1),
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::STOREV),
        MetaInst::Number(0),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let consts = vec![EstaData::new_int(4)];

    run_instructions(&instructions, consts);
}

#[test]
fn test_vm_jump() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::JUMP),
        MetaInst::Number(2),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    run_instructions(&instructions, Vec::new());
}

#[test]
fn test_vm_jumpf() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::JUMPF),
        MetaInst::Number(6),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let consts = vec![EstaData::new_bool(false)];

    run_instructions(&instructions, consts);
}

#[test]
fn test_vm_const() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Const(EstaData::new_int(0)),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let ctx = AsmCtx::new_metainst(instructions);
    let prog = ctx.assemble();

    run_program(prog);
}
