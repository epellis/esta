use crate::backend::program::*;
use crate::vm::bytecode::*;
use std::collections::HashMap;
use std::fmt;

pub mod bytecode;
#[cfg(test)]
mod tests;

/// # The Esta Virtual Machine
///
/// Inspired by the Python VM
///
/// ## Constants (Consts) Field
/// This section carries data constants specific to a certain function.
/// For example, if a function foo calculates 2 + 2, then 2 is a constant
/// in the foo section.
///
/// When a VM is initialized, a mapping between the context and a vec of consts
/// must be passed to the newly created struct.
///
/// ## Context Allocation Field
/// This section carries a mapping from the context to the number of local variables
/// and is used as a LUT to allocate the correct number of local variables upon a
/// function invocation.
///
/// ## Env Field
/// The frames section is a stack of variable bindings, one per every scope.
/// When a function is entered, all declared variables are initialized to Nil and then
/// as the process evolves, these may be updated or used.
///
/// ## Stack Field
/// The stack section is a stack of EstaData, which is used to hold intermediate
/// values during computations.
#[derive(Debug)]
pub struct VirtualMachine {
    insts: Vec<u8>,            // An array of bytecode instructions
    stack: Vec<Vec<EstaData>>, // A stack of frames, one for each function
    env: Vec<Vec<EstaData>>,   // A stack of variable bindings, one for each scope
    consts: Vec<EstaData>,     // All constants used in the program
    context: String,           // The current executing function. Used to lookup consts
    pc: usize,                 // Program counter. Indexes current instruction
}

impl VirtualMachine {
    pub fn new(prog: Program) -> VirtualMachine {
        assert!(!prog.insts.is_empty());
        let stack = vec![Vec::new()];
        let env = vec![Vec::new()];
        VirtualMachine {
            insts: prog.insts,
            stack,
            env,
            consts: prog.consts,
            context: "GLOBAL".to_string(),
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        debug!("{}", self);
        debug!("Inst: {:?}", disassemble_u8(&self.insts));
        debug!("Raw Inst: {:?}", &self.insts);
        debug!("Consts: {:?}", self.consts);

        while VMStatus::RUNNING == self.step()? {
            debug!("{}", self);
            debug!(
                "Inst: {:?}",
                disassemble_u8(&self.insts[self.pc..].to_vec())
            );
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<VMStatus, &'static str> {
        let inst = BYTECODE_ARRAY[self.insts[self.pc] as usize].clone();
        self.pc += 1;

        match inst {
            ByteCode::POP => {
                self.pop_top()?;
            }
            ByteCode::HALT => {
                return Ok(VMStatus::HALTED);
            }
            ByteCode::JUMP => {
                self.pc = self.read_inst_i16() as usize;
            }
            ByteCode::JUMPF => {
                let pc = self.read_inst_i16() as usize;
                if !self.pop_top()?.eval_bool()? {
                    self.pc = pc
                }
            }
            ByteCode::LOADV => {
                let offset = self.env.len() - 1 - self.read_inst_i16() as usize;
                let idx = self.read_inst_i16() as usize;
                let data = self.env[offset][idx].clone();
                self.push_top(data);
            }
            ByteCode::STOREV => {
                let offset = self.env.len() - 1 - self.read_inst_i16() as usize;
                let idx = self.read_inst_i16() as usize;
                let data = self.peek_top()?;
                self.env[offset][idx] = data;
            }
            ByteCode::LOADC => {
                let idx = self.read_inst_i16() as usize;
                self.push_top(self.consts[idx].clone())
            }
            ByteCode::ADD => {
                let rhs = self.pop_top()?;
                let lhs = self.pop_top()?;
                let result = EstaData::new_add(lhs, rhs)?;
                self.push_top(result);
            }
            ByteCode::PUSHE => {
                let local_count = self.read_inst_i16() as usize;
                let mut frame = Vec::new();
                frame.resize(local_count, Default::default());
                self.env.push(frame);
            }
            ByteCode::POPE => {
                self.env.pop();
            }
            ByteCode::PUSHS => {
                self.stack.push(Vec::new());
            }
            ByteCode::POPS => {
                self.stack.pop();
            }
        }

        Ok(VMStatus::RUNNING)
    }

    fn peek_top(&mut self) -> Result<EstaData, &'static str> {
        let idx = self.stack.len() - 1;
        self.stack[idx]
            .last()
            .cloned()
            .ok_or_else(|| "Frame is empty")
    }

    fn pop_top(&mut self) -> Result<EstaData, &'static str> {
        let idx = self.stack.len() - 1;
        let top = self.peek_top();
        self.stack[idx].pop();
        top
    }

    fn push_top(&mut self, data: EstaData) {
        let idx = self.stack.len() - 1;
        self.stack[idx].push(data);
    }

    fn read_inst_i16(&mut self) -> i16 {
        let upper = self.insts[self.pc];
        self.pc += 1;
        let lower = self.insts[self.pc];
        self.pc += 1;
        i16::from_le_bytes([upper, lower])
    }
}

impl fmt::Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let top_stack = &self.stack[self.stack.len() - 1];
        write!(
            f,
            "VM | PC: {} | Context: {} | Stack: {:?}",
            self.pc, self.context, top_stack
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VMStatus {
    HALTED,
    RUNNING,
    FATAL,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct EstaData {
    data: EstaType,
}

impl EstaData {
    pub fn new_int(data: i32) -> EstaData {
        EstaData {
            data: EstaType::Num(data),
        }
    }
    pub fn new_bool(data: bool) -> EstaData {
        EstaData {
            data: EstaType::Bool(data),
        }
    }
    pub fn new_add(lhs: EstaData, rhs: EstaData) -> Result<EstaData, &'static str> {
        match (lhs.data, rhs.data) {
            (EstaType::Num(lhs), EstaType::Num(rhs)) => Ok(EstaData::new_int(lhs + rhs)),
            _ => Err("Incompatible Types"),
        }
    }
    pub fn eval_bool(self) -> Result<bool, &'static str> {
        if let EstaType::Bool(b) = self.data {
            Ok(b)
        } else {
            Err("Self is not a boolean type")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EstaType {
    Num(i32),
    Bool(bool),
    Nil,
}

impl Default for EstaType {
    fn default() -> Self {
        EstaType::Nil
    }
}
