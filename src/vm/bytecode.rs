pub enum ByteCode {
    LOADC, // Push a value to the stack
    LOAD,  // Push a value at address specified in top of address to stack
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

#[allow(dead_code)]
pub struct Inst<T> {
    pub inst: ByteCode,
    pub data: Option<T>,
}

#[allow(dead_code)]
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
