use crate::vm::bytecode::{ByteCode, BYTECODE_ARRAY};

pub mod bytecode;
#[cfg(test)]
mod tests;

/// The Esta Virtual Machine
///
/// Inspired by the Python VM
#[derive(Debug)]
pub struct VirtualMachine {
    insts: Vec<u8>,        // An array of bytecode instructions
    frames: Vec<Vec<i32>>, // A stack of frames, one for each scope
    pc: usize,             // Program counter. Indexes current instruction
}

impl VirtualMachine {
    pub fn new(insts: Vec<u8>) -> VirtualMachine {
        assert!(!insts.is_empty());
        let frames = vec![Vec::new()];
        VirtualMachine {
            insts,
            frames,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        while VMStatus::RUNNING == self.step()? {}
        Ok(())
    }

    fn step(&mut self) -> Result<VMStatus, &'static str> {
        let inst = BYTECODE_ARRAY[self.insts[self.pc] as usize].clone();
        self.pc += 1;

        match inst {
            ByteCode::POP => {
                self.pop_top()?;
            }
            ByteCode::ADD => {
                let rhs = self.pop_top()?;
                let lhs = self.pop_top()?;
                self.push_top(rhs + lhs);
            }
            ByteCode::HALT => {
                return Ok(VMStatus::HALTED);
            }
            ByteCode::LOADC => {
                let d = self.read_inst_i16();
                self.push_top(d as i32)
            }
        }

        Ok(VMStatus::RUNNING)
    }

    fn pop_top(&mut self) -> Result<i32, &'static str> {
        let idx = self.frames.len() - 1;
        self.frames[idx].pop().ok_or_else(|| "Frame is empty")
    }

    fn push_top(&mut self, elem: i32) {
        let idx = self.frames.len() - 1;
        self.frames[idx].push(elem);
    }

    fn read_inst_i16(&mut self) -> i16 {
        let upper = self.insts[self.pc];
        self.pc += 1;
        let lower = self.insts[self.pc];
        self.pc += 1;
        i16::from_le_bytes([upper, lower])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VMStatus {
    HALTED,
    RUNNING,
    FATAL,
}
