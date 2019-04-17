mod bytecode;

use self::bytecode::{ByteCode, Inst};
use std::ops::Add;

/// The Esta Virtual Machine
pub struct VirtualMachine {
    stack: Vec<u64>,
    inst: Vec<Inst<u64>>,
    data: u64,
    pc: usize,
}

impl VirtualMachine {
    pub fn new(inst: Vec<Inst<u64>>) -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            inst,
            data: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let curr = &self.inst[self.pc];
            self.pc += 1;
            match curr.inst {
                ByteCode::PUSH => self.push(&curr.data.unwrap().clone()),
                ByteCode::POP => {
                    self.pop()?;
                }
                // TODO: For floating point ops, replace u64 with Enum { u64, f64 ...}
                ByteCode::ADD => {
                    let res = self.pop()? + self.pop()?;
                    self.push(&res);
                }
                ByteCode::SUB => {
                    let res = self.pop()? - self.pop()?;
                    self.push(&res);
                }
                ByteCode::MUL => {
                    let res = self.pop()? * self.pop()?;
                    self.push(&res);
                }
                ByteCode::DIV => {
                    let res = self.pop()? / self.pop()?;
                    self.push(&res);
                }
                ByteCode::AND => {
                    let before = self.stack.len();
                    let res = self.pop()? == 1 && self.pop()? == 1;
                    self.shrink_stack(before - 2)?;
                    self.push(&VirtualMachine::map_bool(res));
                }
                ByteCode::OR => {
                    let before = self.stack.len();
                    let res = self.pop()? == 1 || self.pop()? == 1;
                    self.shrink_stack(before - 2)?;
                    self.push(&VirtualMachine::map_bool(res));
                }
                ByteCode::HALT => return Ok(()),
            }
        }
    }

    #[inline]
    fn push(&mut self, data: &u64) {
        self.stack.push(data.clone());
    }

    #[inline]
    fn top(&mut self) -> Result<&u64, &'static str> {
        self.stack.last().ok_or_else(|| "Empty stack")
    }

    #[inline]
    fn pop(&mut self) -> Result<u64, &'static str> {
        self.data = self.top()?.clone();
        self.stack.pop();
        Ok(self.data)
    }

    #[inline]
    fn shrink_stack(&mut self, target_len: usize) -> Result<(), &'static str> {
        while self.stack.len() > target_len {
            self.pop()?;
        }
        Ok(())
    }

    pub fn debug_stack(&self) -> &Vec<u64> {
        &self.stack
    }

    pub fn map_bool(cond: bool) -> u64 {
        match cond {
            true => 1,
            false => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::bytecode::*;
    use crate::vm::*;

    #[test]
    fn test_init() {
        let mut instructions: Vec<Inst<u64>> = vec![Inst::new_inst(ByteCode::HALT)];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
    }

    #[test]
    fn test_add() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_inst(ByteCode::ADD),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![2]);
    }

    #[test]
    fn test_sub() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_inst(ByteCode::SUB),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![0]);
    }

    #[test]
    fn test_mul() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_inst(ByteCode::MUL),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![1]);
    }

    #[test]
    fn test_div() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_inst(ByteCode::DIV),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![1]);
    }

    #[test]
    fn test_and() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 0),
            Inst::new_inst(ByteCode::AND),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![0]);
    }

    #[test]
    fn test_or() {
        let mut instructions: Vec<Inst<u64>> = vec![
            Inst::new_data(ByteCode::PUSH, 1),
            Inst::new_data(ByteCode::PUSH, 0),
            Inst::new_inst(ByteCode::OR),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(*vm.debug_stack(), vec![1]);
    }
}