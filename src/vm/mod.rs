use crate::vm::bytecode::*;
use std::collections::HashMap;
use std::fmt;

pub mod bytecode;
#[cfg(test)]
mod tests;

/// The Esta Virtual Machine
///
/// Inspired by the Python VM
#[derive(Debug)]
pub struct VirtualMachine {
    insts: Vec<u8>,                         // An array of bytecode instructions
    frames: Vec<Vec<EstaData>>,             // A stack of frames, one for each scope
    envs: Vec<Vec<EstaData>>,               // A stack of variable bindings, one for each scope
    consts: HashMap<String, Vec<EstaData>>, // A mapping for all constants specific to a function
    context: String,                        // The current executing function. Used to lookup consts
    context_alloc: HashMap<String, usize>, // A mapping between the function and the number of locals
    pc: usize,                             // Program counter. Indexes current instruction
}

impl VirtualMachine {
    pub fn new(
        insts: Vec<u8>,
        consts: HashMap<String, Vec<EstaData>>,
        context_alloc: HashMap<String, usize>,
    ) -> VirtualMachine {
        assert!(!insts.is_empty());
        let frames = vec![Vec::new()];
        let envs = vec![Vec::new()];
        VirtualMachine {
            insts,
            frames,
            envs,
            consts,
            context: "GLOBAL".to_string(),
            context_alloc,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        debug!("{}", self);
        debug!(
            "Inst: {:?}",
            disassemble_u8(&self.insts[self.pc..].to_vec())
        );
        // TODO: Setup global stage
        while VMStatus::RUNNING == self.step()? {
            debug!("{}", self);
            debug!(
                "Inst: {:?}",
                disassemble_u8(&self.insts[self.pc..].to_vec())
            );
        }
        Ok(())
    }

    fn step(&mut self) -> Result<VMStatus, &'static str> {
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
                let offset = self.envs.len() - 1 - self.read_inst_i16() as usize;
                let idx = self.read_inst_i16() as usize;
                let data = self.envs[offset][idx].clone();
                self.push_top(data);
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
        }

        Ok(VMStatus::RUNNING)
    }

    fn pop_top(&mut self) -> Result<EstaData, &'static str> {
        let idx = self.frames.len() - 1;
        self.frames[idx].pop().ok_or_else(|| "Frame is empty")
    }

    fn push_top(&mut self, data: EstaData) {
        let idx = self.frames.len() - 1;
        self.frames[idx].push(data);
    }

    // Makes a
    fn push_envs(&mut self, ctx: Vec<EstaData>) {}

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

#[derive(Debug, Clone, PartialEq)]
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
}
