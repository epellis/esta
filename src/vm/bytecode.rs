use crate::vm::{EstaData, EstaType};
use std::collections::HashMap;
use std::convert::From;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Clone, Copy, EnumIter, Display, Hash)]
pub enum ByteCode {
    HALT,   // Halts the Virtual Machine
    JUMP,   // Set PC to argument
    JUMPF,  // Pops top, if top == False, then set PC to argument
    POP,    // Pops off the top item on the stack
    ADD,    // Pops off the top two items from the stack, tries to add and push a result
    LOADC,  // Loads an EstaData from the constant's pool for that function and pushes to stack
    LOADV,  // Loads an EstaData variable from the environment's pool and pushes to stack
    STOREV, // Stores the top of stack to the environment's pool
    PUSHE,  // Pushes a new environment frame, one argument is the number of local variables
    POPE,   // Pops the first environment frame
    PUSHS,  // Push a new stack frame
    POPS,   // Pop a new stack frame
}

impl From<u8> for ByteCode {
    fn from(value: u8) -> Self {
        if value as usize >= BYTECODE_ARRAY.len() {
            panic!("Bytecode Index out of range")
        } else {
            BYTECODE_ARRAY[value as usize]
        }
    }
}

impl From<ByteCode> for u8 {
    fn from(value: ByteCode) -> Self {
        BYTECODE_ARRAY.iter().position(|&x| x == value).unwrap() as u8
    }
}

lazy_static! {
    pub static ref BYTECODE_ARRAY: Vec<ByteCode> = { ByteCode::iter().collect() };
    pub static ref BYTECODE_ARITY: HashMap<ByteCode, u32> = {
        let mut m = HashMap::new();
        m.insert(ByteCode::HALT, 0);
        m.insert(ByteCode::JUMP, 1);
        m.insert(ByteCode::JUMPF, 1);
        m.insert(ByteCode::POP, 0);
        m.insert(ByteCode::ADD, 0);
        m.insert(ByteCode::LOADC, 1);
        m.insert(ByteCode::LOADV, 2);
        m.insert(ByteCode::STOREV, 2);
        m.insert(ByteCode::PUSHE, 1);
        m.insert(ByteCode::POPE, 0);
        m.insert(ByteCode::PUSHS, 0);
        m.insert(ByteCode::POPS, 0);
        m
    };
}

/// Metainsts are an intermediate representation of bytecode that will be further
/// simplified by the compiler at a later stage
#[derive(Debug, Clone)]
pub enum MetaInst {
    ByteCode(ByteCode),
    Number(i16),
    Label(String),
    Const(EstaData),
    Identifier(String),
    Declaration(String),
}

pub fn disassemble_u8(v: &Vec<u8>) -> Vec<MetaInst> {
    let mut idx = 0;
    let mut inst = Vec::new();
    while idx < v.len() {
        let bytecode = ByteCode::from(v[idx]);
        idx += 1;
        inst.push(MetaInst::ByteCode(bytecode));
        for _ in 0..(BYTECODE_ARITY.get(&bytecode).unwrap().clone()) {
            let upper = v[idx];
            idx += 1;
            let lower = v[idx];
            idx += 1;
            let number = i16::from_le_bytes([upper, lower]);
            inst.push(MetaInst::Number(number));
        }
    }

    inst
}
