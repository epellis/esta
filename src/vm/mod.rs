pub mod bytecode;
mod serialize;

#[cfg(test)]
mod tests;

use self::bytecode::{ByteCode, Inst};
use crate::LOGGER;
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub enum StepCode {
    HALT,
    CONTINUE,
}

/// The Esta Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine {
    stack: Vec<i64>,
    heap: Vec<i64>,
    inst: Vec<Inst>,
    pc: usize,
    fp: usize,
}

impl VirtualMachine {
    pub fn new(inst: Vec<Inst>) -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            heap: Vec::new(),
            inst,
            pc: 0,
            fp: 1,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        while let StepCode::CONTINUE = self.step()? {
            info!(LOGGER, "{}", self.info());
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<StepCode, &'static str> {
        let ir = &self.inst[self.pc];
        self.pc += 1;

        match ir.inst {
            ByteCode::LOADC => self.push(ir.data.unwrap()),
            ByteCode::LOAD => {
                let addr: usize = self.pop()? as usize;
                self.push(self.stack[addr]);
            }
            ByteCode::LOADA => {
                let addr: usize = ir.data.unwrap() as usize;
                self.push(self.stack[addr]);
            }
            ByteCode::LOADRC => {
                let val = ir.data.unwrap() + self.fp as i64;
                self.push(val);
            }
            ByteCode::STORE => {
                let addr: usize = self.pop()? as usize;

                if self.stack.len() <= addr {
                    self.stack.resize(addr + 1, 0);
                }
                self.stack[addr] = *self.top()?;
            }
            ByteCode::POP => {
                self.pop()?;
            }
            ByteCode::NOP => {}
            ByteCode::MARK => {
                let fp = self.fp as i64;
                self.stack.push(fp);
            }
            ByteCode::CALL => {
                info!(LOGGER, "Old PC: {}", self.pc);
                info!(LOGGER, "Old FP: {}", self.fp);

                self.fp = self.stack.len();
                let tmp = self.pc as i64;
                self.pc = self.pop()? as usize;
                self.push(tmp);

                info!(LOGGER, "New PC: {}", self.pc);
                info!(LOGGER, "New FP: {}", self.fp);
            }
            ByteCode::ALLOC => {
                for _ in 0..ir.data.unwrap() {
                    self.push(0);
                }
            }
            ByteCode::SLIDE => {
                info!(LOGGER, "Old PC: {}", self.pc);
                let ret_value = *self.top()?;
                info!(LOGGER, "Sliding: {}", ret_value);
                for _ in 0..=ir.data.unwrap() {
                    self.pop()?;
                }
                self.push(ret_value);
                info!(LOGGER, "New top: {}", self.top()?);
            }
            ByteCode::RET => {
                info!(LOGGER, "Old PC: {}", self.pc);
                info!(LOGGER, "Old FP: {}", self.fp);
                let new_sp = self.fp as i64 - ir.data.unwrap();
                self.pc = self.stack[self.fp - 1] as usize;
                let new_fp = self.stack[self.fp - 2] as usize;

                while self.stack.len() > new_sp as usize {
                    self.pop()?;
                }

                self.fp = new_fp;
                info!(LOGGER, "Restored PC: {}", self.pc);
                info!(LOGGER, "Restored FP: {}", self.fp);
            }
            ByteCode::NEW => {
                let heap_top = self.heap.len() as usize;
                let length = self.pop()? as usize;
                self.heap.resize(heap_top + length, 0);
                self.push(heap_top as i64);
            }
            ByteCode::JUMP => {
                self.pc = ir.data.unwrap() as usize;
            }
            ByteCode::JUMPZ => {
                let new_pc = ir.data.unwrap() as usize;
                if self.pop()? == 0 {
                    self.pc = new_pc;
                }
            }
            ByteCode::HALT => {
                println!("Exited Successfully");
                return Ok(StepCode::HALT);
            }
            ByteCode::ADD => {
                let res = self.pop()? + self.pop()?;
                println!("{}", res);
                self.push(res);
            }
            ByteCode::SUB => {
                let lhs = self.pop()?;
                let rhs = self.pop()?;
                self.push(rhs - lhs);
            }
            ByteCode::MUL => {
                let res = self.pop()? * self.pop()?;
                self.push(res);
            }
            ByteCode::DIV => {
                let lhs = self.pop()?;
                let rhs = self.pop()?;
                self.push(rhs / lhs);
            }
            ByteCode::MOD => {
                let res = self.pop()? % self.pop()?;
                self.push(res);
            }
            ByteCode::AND => {
                let lhs = VirtualMachine::t_to_bool(self.pop()?);
                let rhs = VirtualMachine::t_to_bool(self.pop()?);
                self.push(VirtualMachine::bool_to_t(lhs && rhs));
            }
            ByteCode::OR => {
                let lhs = VirtualMachine::t_to_bool(self.pop()?);
                let rhs = VirtualMachine::t_to_bool(self.pop()?);
                self.push(VirtualMachine::bool_to_t(lhs || rhs));
            }
            ByteCode::EQ => {
                let res = self.pop()? == self.pop()?;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::NEQ => {
                let res = self.pop()? != self.pop()?;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::LE => {
                let (a, b) = (self.pop()?, self.pop()?);
                let res = b < a;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::LEQ => {
                let (a, b) = (self.pop()?, self.pop()?);
                let res = b <= a;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::GE => {
                let (a, b) = (self.pop()?, self.pop()?);
                let res = b > a;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::GEQ => {
                let (a, b) = (self.pop()?, self.pop()?);
                let res = b >= a;
                self.push(VirtualMachine::bool_to_t(res));
            }
            ByteCode::NEG => {
                let res = self.pop()?.checked_neg().unwrap();
                self.push(res);
            }
            ByteCode::NOT => {
                let res = !VirtualMachine::t_to_bool(self.pop()?);
                self.push(VirtualMachine::bool_to_t(res));
            }
        }
        Ok(StepCode::CONTINUE)
    }

    #[inline]
    fn push(&mut self, data: i64) {
        self.stack.push(data);
    }

    #[inline]
    fn top(&self) -> Result<&i64, &'static str> {
        self.stack.last().ok_or_else(|| "Empty stack")
    }

    #[inline]
    fn pop(&mut self) -> Result<i64, &'static str> {
        let top = *self.top()?;
        self.stack.pop();
        Ok(top)
    }

    pub fn bool_to_t(cond: bool) -> i64 {
        if cond {
            1
        } else {
            0
        }
    }

    pub fn t_to_bool(cond: i64) -> bool {
        cond == 1
    }

    pub fn info(&self) -> String {
        let ir = format!("{}", self.inst[self.pc].clone());
        format!(
            "{: >3} {: >2} {: <10} {:?}",
            &self.pc, &self.fp, ir, &self.stack
        )
    }
}
