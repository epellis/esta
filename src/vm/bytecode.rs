use crate::frontend::ast::Opcode;
use std::collections::HashMap;
use std::fmt;

use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Clone, EnumIter, Display)]
pub enum ByteCode {
    LOADC,  // Push a value to the stack
    LOAD,   // Push a value at address specified in top of address to stack
    LOADA,  // LOADC followed by LOAD. For variable addresses
    LOADRC, // Push data plus the current frame pointer to the stack
    STORE,  // Overwrite a value at address specified in top of stack
    POP,    // Pop the top element off the stack
    NOP,    // No Operation Done
    MARK,   // Save context for function call setup
    CALL,   // Switch context between two functions
    ALLOC,  // Extend the stack by a given amount
    SLIDE,  // Move the return value from the top of stack to top of FP
    RET,    // Return control to the caller
    NEW,    // Allocate space on the heap for an object the size of top of stack
    JUMP,   // Change the PC to a new value
    JUMPZ,  // Change the PC to a new value if the top of stack is zero
    HALT,   // Stop the VM from executing
    ADD,    // Add the top two items on the stack and push the result
    SUB,    // Subtract the top two items on the stack and push the result
    MUL,    // Multiply the top two items on the stack and push the result
    DIV,    // Divide the top two items on the stack and push the result
    MOD,    // Modulo's the top two items on the stack and push the result
    AND,    // Logical AND's the top two items on the stack and push the result
    OR,     // Logical OR's the top two items on the stack and push the result
    EQ,     // Checks if the top two items on the stack equal each other and push result
    NEQ,    // Checks if the top two items on the stack do not equal each other and push result
    LE,     // Checks if the first item on the stack is less than the second item and push result
    LEQ, // Checks if the first item on the stack is less than or equal the second item and push result
    GE,  // Checks if the first item on the stack is greater than the second item and push result
    GEQ, // Checks if the first item on the stack is greater than or equal the second item and push result
    NEG, // Replace the top of the stack with it's negative
    NOT, // Replace the top of the stack with it's opposite
}

lazy_static! {
    pub static ref BIN_OP_TO_BYTE: HashMap<Opcode, ByteCode> = {
        let mut m = HashMap::new();
        m.insert(Opcode::Add, ByteCode::ADD);
        m.insert(Opcode::Sub, ByteCode::SUB);
        m.insert(Opcode::Mul, ByteCode::MUL);
        m.insert(Opcode::Div, ByteCode::DIV);
        m.insert(Opcode::Mod, ByteCode::MOD);
        m.insert(Opcode::Greater, ByteCode::GE);
        m.insert(Opcode::GreaterEqual, ByteCode::GEQ);
        m.insert(Opcode::Lesser, ByteCode::LE);
        m.insert(Opcode::LesserEqual, ByteCode::LEQ);
        m.insert(Opcode::EqualEqual, ByteCode::EQ);
        m.insert(Opcode::BangEqual, ByteCode::NEQ);
        m.insert(Opcode::And, ByteCode::AND);
        m.insert(Opcode::Or, ByteCode::OR);
        m
    };
    pub static ref UN_OP_TO_BYTE: HashMap<Opcode, ByteCode> = {
        let mut m = HashMap::new();
        m.insert(Opcode::Not, ByteCode::NOT);
        m.insert(Opcode::Sub, ByteCode::NEG);
        m
    };
    pub static ref RAW_TO_BYTE: HashMap<String, ByteCode> = {
        let mut m = HashMap::new();
        for bytecode in ByteCode::iter() {
            let str_rep = format!("{}", bytecode);
            m.insert(str_rep, bytecode);
        }
        m
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inst {
    pub inst: ByteCode,
    pub data: Option<i64>,
}

impl Inst {
    pub fn new_data(inst: ByteCode, data: i64) -> Inst {
        Inst {
            inst,
            data: Some(data),
        }
    }
    pub fn new_inst(inst: ByteCode) -> Inst {
        Inst { inst, data: None }
    }
}

#[derive(Debug, Clone)]
pub enum MetaAsm {
    Inst(MetaInst),
    Lbl(String),
}

#[derive(Debug, Clone)]
pub struct MetaInst {
    pub inst: ByteCode,
    pub var: MetaVar,
}

impl MetaInst {
    pub fn new_data(inst: ByteCode, data: i64) -> MetaInst {
        let var = MetaVar::Data(data);
        MetaInst { inst, var }
    }
    pub fn new_label(inst: ByteCode, label: String) -> MetaInst {
        let var = MetaVar::Label(label);
        MetaInst { inst, var }
    }
    pub fn new_local_alloc(inst: ByteCode, label: String) -> MetaInst {
        let var = MetaVar::LocalAlloc(label);
        MetaInst { inst, var }
    }
    pub fn new_inst(inst: ByteCode) -> MetaInst {
        let var = MetaVar::None;
        MetaInst { inst, var }
    }
    pub fn new_nop() -> MetaInst {
        let inst = ByteCode::NOP;
        let var = MetaVar::None;
        MetaInst { inst, var }
    }
}

#[derive(Debug, Clone)]
pub enum MetaVar {
    Data(i64),
    Label(String),
    LocalAlloc(String),
    None,
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.data {
            Some(data) => write!(f, "{:?} {:?}", self.inst, data),
            None => write!(f, "{:?}", self.inst),
        }
    }
}
