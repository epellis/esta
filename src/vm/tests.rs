use crate::vm::bytecode::*;
use crate::vm::*;
use std::collections::HashMap;

extern crate env_logger;
extern crate log;

fn run_instructions(
    insts: &Vec<MetaInst>,
    consts: HashMap<String, Vec<EstaData>>,
    context_alloc: HashMap<String, usize>,
) -> VirtualMachine {
    let _ = env_logger::builder()
        .default_format_timestamp(false)
        .try_init();

    let insts = assemble_metainst(&insts);
    println!("Instructions: {:?}", disassemble_u8(&insts));
    println!("Instructions: {:x?}", insts.as_slice());

    let mut vm = VirtualMachine::new(insts, consts, context_alloc);
    let res = vm.run();
    match res {
        Ok(()) => {}
        Err(e) => println!("Test finished with error: {}", e),
    }
    assert!(res.is_ok());
    vm
}

#[test]
fn test_vm_halt() {
    let instructions = vec![MetaInst::ByteCode(ByteCode::HALT)];

    run_instructions(&instructions, HashMap::new(), HashMap::new());
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

    let mut consts = HashMap::new();
    let globals = vec![EstaData::new_int(4)];
    consts.insert("GLOBAL".to_string(), globals);

    run_instructions(&instructions, consts, HashMap::new());
}

#[test]
fn test_vm_loadc() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let mut consts = HashMap::new();
    let globals = vec![EstaData::new_int(4)];
    consts.insert("GLOBAL".to_string(), globals);

    run_instructions(&instructions, consts, HashMap::new());
}

#[test]
fn test_vm_loadv() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADV),
        MetaInst::Number(0),
        MetaInst::Number(0),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    let mut consts = HashMap::new();
    let globals = vec![EstaData::new_int(4)];
    consts.insert("GLOBAL".to_string(), globals);

    let mut context_alloc = HashMap::new();
    context_alloc.insert("GLOBAL".to_string(), 1);

    run_instructions(&instructions, consts, context_alloc);
}
