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
/// ## Frames Field
/// The frames section is a stack of variable bindings, one per every scope.
/// When a function is entered, all declared variables are initialized to Nil and then
/// as the process evolves, these may be updated or used.
#[derive(Debug)]
pub struct VirtualMachine {
    insts: Vec<u8>,             // An array of bytecode instructions
    frames: Vec<Vec<EstaData>>, // A stack of frames, one for each scope
    //    envs: Vec<Vec<EstaData>>,               // A stack of variable bindings, one for each scope
    consts: HashMap<String, Vec<EstaData>>, // A mapping for all constants specific to a function
    context: String,                        // The current executing function. Used to lookup consts
    // TODO: Eventually this can be merged into the function call opcode as a parameter
    context_alloc: HashMap<String, usize>, // A mapping between the function and the number of locals
    pc: usize,                             // Program counter. Indexes current instruction
}

impl VirtualMachine {
    pub fn new(prog: Program) -> VirtualMachine {
        assert!(!prog.insts.is_empty());
        let frames = vec![Vec::new()];
        VirtualMachine {
            insts: prog.insts,
            frames,
            consts: prog.consts,
            context: "GLOBAL".to_string(),
            context_alloc: prog.context_alloc,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        debug!("{}", self);
        debug!(
            "Inst: {:?}",
            disassemble_u8(&self.insts[self.pc..].to_vec())
        );

        //        // Alloc space in GLOBAL environment for variables
        //        let mem_size = self.context_alloc.get("GLOBAL").or(Some(&0)).unwrap();
        //        let mut global_envs = Vec::new();
        //        global_envs.resize(*mem_size, Default::default());
        //        self.push_envs(global_envs);

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
            ByteCode::LOADV => {
                let offset = self.frames.len() - 1 - self.read_inst_i16() as usize;
                let idx = self.read_inst_i16() as usize;
                let data = self.frames[offset][idx].clone();
                self.push_top(data);
            }
            ByteCode::STOREV => {
                let offset = self.frames.len() - 1 - self.read_inst_i16() as usize;
                let idx = self.read_inst_i16() as usize;
                let data = self.peek_top()?;
                self.frames[offset][idx] = data;
            }
            ByteCode::LOADC => {
                let idx = self.read_inst_i16() as usize;
                let consts = self
                    .consts
                    .get(&self.context)
                    .ok_or_else(|| "Const not found")?;
                self.push_top(consts[idx].clone())
            }
            ByteCode::ADD => {
                let rhs = self.pop_top()?;
                let lhs = self.pop_top()?;
                let result = EstaData::new_add(lhs, rhs)?;
                self.push_top(result);
            }
            ByteCode::PUSHF => {
                let local_count = self.read_inst_i16() as usize;
                let mut frame = Vec::new();
                frame.resize(local_count, Default::default());
                self.frames.push(frame);
            }
            ByteCode::POPF => {
                self.frames.pop();
            }
        }

        Ok(VMStatus::RUNNING)
    }

    fn peek_top(&mut self) -> Result<EstaData, &'static str> {
        let idx = self.frames.len() - 1;
        self.frames[idx]
            .last()
            .cloned()
            .ok_or_else(|| "Frame is empty")
    }

    fn pop_top(&mut self) -> Result<EstaData, &'static str> {
        let idx = self.frames.len() - 1;
        let top = self.peek_top();
        self.frames[idx].pop();
        top
    }

    fn push_top(&mut self, data: EstaData) {
        let idx = self.frames.len() - 1;
        self.frames[idx].push(data);
    }

    //    // Makes a new environment and pushes it to the environment stack
    //    fn push_frame(&mut self, frame: Vec<EstaData>) {
    //        self.envs.push(env);
    //    }

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
        let top_frame = &self.frames[self.frames.len() - 1];
        write!(
            f,
            "VM | PC: {} | Context: {} | Frame: {:?}",
            self.pc, self.context, top_frame
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VMStatus {
    HALTED,
    RUNNING,
    FATAL,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EstaData {
    data: EstaType,
}

impl EstaData {
    pub fn new_int(data: i32) -> EstaData {
        EstaData {
            data: EstaType::Num(data),
        }
    }
    pub fn new_add(lhs: EstaData, rhs: EstaData) -> Result<EstaData, &'static str> {
        match (lhs.data, rhs.data) {
            (EstaType::Num(lhs), EstaType::Num(rhs)) => Ok(EstaData::new_int(lhs + rhs)),
            _ => Err("Incompatible Types"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EstaType {
    Num(i32),
    Nil,
}

impl Default for EstaType {
    fn default() -> Self {
        EstaType::Nil
    }
}
