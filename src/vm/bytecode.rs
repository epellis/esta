pub enum ByteCode {
    PUSH, // Push a value to the top of the stack
    POP,  // Pop a value off the top of the stack
    ADD,  // Add the top two values and push it to the stack
    SUB,  // Subtract the top two values and push it to the stack
    MUL,  // Multiply the top two values and push it to the stack
    DIV,  // Divide the top two values and push it to the stack
    AND,  // And the top two values and push it to the stack
    OR,   // Or the top two values and push it to the stack
    EQ,   // Compare the top two values and push it to the stack
    NEQ,  // Compare the top two values and push it to the stack
    GT,   // Compare the top two values and push it to the stack
    GTE,  // Compare the top two values and push it to the stack
    LT,   // Compare the top two values and push it to the stack
    LTE,  // Compare the top two values and push it to the stack
    STR,  // Pop the top of the stack and store it in the provided index
    LD,   // Load an item from the provided index and push it to the stack
    BRT,  // Branch to the instruction specified if the top of the stack is 1
    JMP,  // Jump to the instruction specified
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
