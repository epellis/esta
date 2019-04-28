use crate::frontend::ast::Opcode;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ByteCode {
    LOADC, // Push a value to the stack
    LOAD,  // Push a value at address specified in top of address to stack
    STORE, // Overwrite a value at address specified in top of stack
    HALT,  // Stop the VM from executing
    ADD,   // Add the top two items on the stack and push the result
    SUB,   // Subtract the top two items on the stack and push the result
    MUL,   // Multiply the top two items on the stack and push the result
    DIV,   // Divide the top two items on the stack and push the result
    MOD,   // Modulo's the top two items on the stack and push the result
    AND,   // Logical AND's the top two items on the stack and push the result
    OR,    // Logical OR's the top two items on the stack and push the result
    EQ,    // Checks if the top two items on the stack equal each other and push result
    NEQ,   // Checks if the top two items on the stack do not equal each other and push result
    LE,    // Checks if the first item on the stack is less than the second item and push result
    LEQ, // Checks if the first item on the stack is less than or equal the second item and push result
    GE,  // Checks if the first item on the stack is greater than the second item and push result
    GEQ, // Checks if the first item on the stack is greater than or equal the second item and push result
    NEG, // Replace the top of the stack with it's negative
    NOT, // Replace the top of the stack with it's opposite
}

lazy_static! {
    pub static ref OP_TO_BYTE: HashMap<Opcode, ByteCode> = {
        let mut m = HashMap::new();
        m.insert(Opcode::Add, ByteCode::ADD);
        m.insert(Opcode::Sub, ByteCode::SUB);
        m.insert(Opcode::Mul, ByteCode::MUL);
        m.insert(Opcode::Div, ByteCode::DIV);
        m.insert(Opcode::Greater, ByteCode::GE);
        m.insert(Opcode::GreaterEqual, ByteCode::GEQ);
        m.insert(Opcode::Lesser, ByteCode::LE);
        m.insert(Opcode::LesserEqual, ByteCode::LEQ);
        m.insert(Opcode::EqualEqual, ByteCode::EQ);
        m.insert(Opcode::BangEqual, ByteCode::NEQ);
        m.insert(Opcode::And, ByteCode::AND);
        m.insert(Opcode::Or, ByteCode::OR);
        m.insert(Opcode::Not, ByteCode::NOT);
        m
    };
}

#[derive(Debug)]
pub struct Inst<T> {
    pub inst: ByteCode,
    pub data: Option<T>,
}

impl<T> Inst<T> {
    pub fn new_data(inst: ByteCode, data: T) -> Inst<T> {
        Inst {
            inst,
            data: Some(data),
        }
    }
    pub fn new_inst(inst: ByteCode) -> Inst<T> {
        Inst { inst, data: None }
    }
}
