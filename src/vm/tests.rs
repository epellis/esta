use crate::vm::bytecode::*;
use crate::vm::*;

#[test]
fn test_empty_main() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::ALLOC, 1),
        Inst::new_inst(ByteCode::MARK),
        Inst::new_data(ByteCode::LOADC, 5),
        Inst::new_inst(ByteCode::CALL),
        Inst::new_inst(ByteCode::HALT),
        Inst::new_data(ByteCode::RET, 2),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    let mut count = 0;
    let max_count = 20;
    println!("{}", vm.info());
    while let Ok(StepCode::CONTINUE) = vm.step() {
        println!("{}", vm.info());
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
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::ALLOC, 1),
        Inst::new_inst(ByteCode::MARK),
        Inst::new_data(ByteCode::LOADC, 5),
        Inst::new_inst(ByteCode::CALL),
        Inst::new_inst(ByteCode::HALT),
        Inst::new_data(ByteCode::ALLOC, 0),
        Inst::new_data(ByteCode::LOADC, 9),
        Inst::new_data(ByteCode::LOADRC, -3),
        Inst::new_inst(ByteCode::STORE),
        Inst::new_data(ByteCode::RET, 1),
        Inst::new_data(ByteCode::RET, 2),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    let mut count = 0;
    let max_count = 20;
    println!("{}", vm.info());
    while let Ok(StepCode::CONTINUE) = vm.step() {
        println!("{}", vm.info());
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
//    var a = 9;
//    return a;
//  }
/// ```
fn test_returning_main_var() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::ALLOC, 1),
        Inst::new_inst(ByteCode::MARK),
        Inst::new_data(ByteCode::LOADC, 5),
        Inst::new_inst(ByteCode::CALL),
        Inst::new_inst(ByteCode::HALT),
        Inst::new_data(ByteCode::ALLOC, 1),
        Inst::new_data(ByteCode::LOADC, 9),
        Inst::new_data(ByteCode::LOADRC, 0),
        Inst::new_inst(ByteCode::STORE),
        Inst::new_inst(ByteCode::POP),
        Inst::new_data(ByteCode::LOADRC, 0),
        Inst::new_inst(ByteCode::LOAD),
        Inst::new_data(ByteCode::LOADRC, -3),
        Inst::new_inst(ByteCode::STORE),
        Inst::new_data(ByteCode::RET, 1),
        Inst::new_data(ByteCode::RET, 2),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    let mut count = 0;
    let max_count = 20;
    println!("{}", vm.info());
    while let Ok(StepCode::CONTINUE) = vm.step() {
        println!("{}", vm.info());
        count += 1;
        if count > max_count {
            break;
        }
    }
    assert_eq!(count < max_count, true);
    assert_eq!(9, vm.stack[0]);
}

#[test]
fn test_halt() {
    let instructions: Vec<Inst> = vec![Inst::new_inst(ByteCode::HALT)];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    assert_eq!(vm.run().is_ok(), true);
}

#[test]
fn test_loadc() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 0),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}

#[test]
fn test_loadrc() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADRC, 1),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[1].to_vec(), &vm.stack);
}

#[test]
fn test_mod() {
    let instructions: Vec<Inst> = vec![
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_data(ByteCode::LOADC, 2),
        Inst::new_inst(ByteCode::MOD),
        Inst::new_inst(ByteCode::HALT),
    ];
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
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
    let mut vm: VirtualMachine = VirtualMachine::new(instructions);
    assert_eq!(vm.run().is_ok(), true);
    assert_eq!(&[0].to_vec(), &vm.stack);
}
