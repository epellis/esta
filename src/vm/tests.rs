use crate::vm::bytecode::*;
use crate::vm::serialize::*;
use crate::vm::*;

// TODO: Calling a function in a for loop has strange effects
// TODO: Calling a function in a while loop has strange effects

#[test]
fn test_convert_raw() {
    let raw = "ALLOC 1
    LOADRC 3
    LOADRC -3
    HALT";
    let res = Converter::raw_to_inst(raw);
    assert_eq!(
        Ok(vec![
            Inst::new_data(ByteCode::ALLOC, 1),
            Inst::new_data(ByteCode::LOADRC, 3),
            Inst::new_data(ByteCode::LOADRC, -3),
            Inst::new_inst(ByteCode::HALT)
        ]),
        res
    )
}

#[test]
fn test_empty_main() {
    let raw = "ALLOC 1
    MARK
    LOADC 5
    CALL
    HALT
    ALLOC 0
    RET 2";
    let instructions = Converter::raw_to_inst(raw).unwrap();
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    let mut count = 0;
    let max_count = 20;
    while let Ok(StepCode::CONTINUE) = vm.step() {
        count += 1;
        if count > max_count {
            break;
        }
    }
    assert_eq!(count < max_count, true);
}

#[test]
/// # Function
/// '''
/// fun main() {
///    return 9;
/// }
/// ```
fn test_returning_main() {
    let raw = "ALLOC 1
    MARK
    LOADC 5
    CALL
    HALT
    ALLOC 0
    LOADC 9
    LOADRC -3
    STORE
    RET 1
    RET 2";
    let instructions = Converter::raw_to_inst(raw).unwrap();
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    let mut count = 0;
    let max_count = 20;
    while let Ok(StepCode::CONTINUE) = vm.step() {
        count += 1;
        if count > max_count {
            break;
        }
    }
    assert_eq!(count < max_count, true);
    assert_eq!(9, vm.stack[0]);
}

#[test]
/// # Function
/// '''
/// fun main() {
///    var a = 9;
///    return a;
///  }
/// ```
fn test_returning_main_var() {
    let raw = "ALLOC 1
    MARK
    LOADC 5
    CALL
    HALT
    ALLOC 1
    LOADC 3
    LOADRC 0
    STORE
    POP
    LOADRC 0
    LOAD
    LOADRC -3
    STORE
    RET 1
    RET 2";
    let instructions = Converter::raw_to_inst(raw).unwrap();
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    let mut count = 0;
    let max_count = 20;
    while let Ok(StepCode::CONTINUE) = vm.step() {
        count += 1;
        if count > max_count {
            break;
        }
    }
    assert_eq!(count < max_count, true);
    assert_eq!(3, vm.stack[0]);
}

#[test]
fn test_halt() {
    let instructions: Vec<Inst> = vec![Inst::new_inst(ByteCode::HALT)];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
}

#[test]
fn test_loadc() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_loadrc() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADRC, 1),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[2].to_vec(), &vm.stack);
}

#[test]
fn test_load() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::STORE),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::LOAD),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[2, 2].to_vec(), &vm.stack);
}

#[test]
fn test_store() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::STORE),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[2].to_vec(), &vm.stack);
}

#[test]
fn test_pop() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::POP),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[2].to_vec(), &vm.stack);
}

#[test]
fn test_new() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 4),
        Inst::new_inst(ByteCode::NEW),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_jump() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::JUMP, 3),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_jumpz() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_data(ByteCode::JUMPZ, 4),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_add() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::ADD),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[4].to_vec(), &vm.stack);
}

#[test]
fn test_sub() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::SUB),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_mul() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::MUL),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[4].to_vec(), &vm.stack);
}

#[test]
fn test_div() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::DIV),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_mod() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 4),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::MOD),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_and() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::AND),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_or() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::OR),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_eq() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::EQ),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_neq() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::NEQ),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_le() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::LE),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_neg() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::NEG),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[-1].to_vec(), &vm.stack);
}

#[test]
fn test_not() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::NOT),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_loadh() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::LOADH),
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::LOADH),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    vm.heap = vec![5, 6];
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[5, 6].to_vec(), &vm.stack);
}

#[test]
fn test_storeh() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 5),
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::STOREH),
        Inst::new_inst(ByteCode::POP),
        Inst::new_data(ByteCode::LOADC, 6),
        Inst::new_data(ByteCode::LOADC, 1),
        Inst::new_inst(ByteCode::STOREH),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions, vec![]);
    vm.heap = vec![0, 0];
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[5, 6].to_vec(), &vm.heap);
}
