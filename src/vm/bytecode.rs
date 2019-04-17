pub enum ByteCode {
    PUSH, // Push a value to the top of the stack
    POP,  // Pop a value off the top of the stack
    ADD,  // Add the top two values and push it to the stack
    SUB,  // Subtract the top two values and push it to the stack
    MUL,  // Multiply the top two values and push it to the stack
    DIV,  // Divide the top two values and push it to the stack
    AND,  // And the top two values and push it to the stack
    OR,   // Or the top two values and push it to the stack
    HALT, // Stop the VM from executing
}

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
