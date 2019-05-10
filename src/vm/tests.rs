use crate::vm::bytecode::*;
use crate::vm::*;

fn run_instructions(instructions: &Vec<MetaInst>) -> VirtualMachine {
    let instructions = assemble_metainst(&instructions);
    println!("Instructions: {:?}", disassemble_u8(&instructions));
    println!("Instructions: {:x?}", instructions.as_slice());

    let mut vm = VirtualMachine::new(instructions);
    assert!(vm.run().is_ok());
    vm
}

#[test]
fn test_vm_halt() {
    let instructions = vec![MetaInst::ByteCode(ByteCode::HALT)];

    run_instructions(&instructions);
}

#[test]
fn test_vm_add() {
    let instructions = vec![
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(4),
        MetaInst::ByteCode(ByteCode::LOADC),
        MetaInst::Number(4),
        MetaInst::ByteCode(ByteCode::ADD),
        MetaInst::ByteCode(ByteCode::HALT),
    ];

    run_instructions(&instructions);
}
