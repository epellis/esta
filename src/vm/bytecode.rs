use crate::frontend::ast::Opcode;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ByteCode {
    LOADC, // Push a value to the stack
    LOAD,  // Push a value at address specified in top of address to stack
    STORE, // Overwrite a value at address specified in top of stack
    POP,   // Pop the top element off the stack
    NEW,   // Allocate space on the heap for an object the size of top of stack
    JUMP,  // Change the PC to a new value
    JUMPZ, // Change the PC to a new value if the top of stack is zero
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

#[derive(Debug, Clone)]
pub struct Inst {
    pub inst: ByteCode,
    pub data: Option<i64>,
    pub label: Option<String>,
}

impl Inst {
    pub fn new_data(inst: ByteCode, data: i64) -> Inst {
        Inst {
            inst,
            data: Some(data),
            label: None,
        }
    }
    pub fn new_inst(inst: ByteCode) -> Inst {
        Inst {
            inst,
            data: None,
            label: None,
        }
    }
    pub fn new_jump(inst: ByteCode, label: String) -> Inst {
        Inst {
            inst,
            data: None,
            label: Some(label),
        }
    }
    pub fn update_lbl(&self, label: &str, offset: i64) -> Inst {
        if let Some(name) = &self.label {
            if name == label {
                return Inst::new_data(self.inst.clone(), offset);
            }
        }
        self.clone()
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.data, &self.label) {
            (None, Some(l)) => write!(f, "{:?} {}", self.inst, l),
            (Some(d), None) => write!(f, "{:?} {}", self.inst, d),
            _ => write!(f, "{:?}", self.inst),
        }
        //        match &self.data {
        //            Some(data) => write!(f, "{:?} {:?}", self.inst, data),
        //            None => write!(f, "{:?}", self.inst),
        //        }
    }
}
