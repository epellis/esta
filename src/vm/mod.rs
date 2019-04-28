pub mod bytecode;

extern crate num_traits;

use self::bytecode::{ByteCode, Inst};
use num_traits::{cast::ToPrimitive, CheckedNeg, Num, One, Zero};
use std::cmp::PartialOrd;
use std::fmt::Debug;

/// The Esta Virtual Machine
pub struct VirtualMachine<T> {
    stack: Vec<T>,
    mem: Vec<T>,
    inst: Vec<Inst<T>>,
    data: T,
    pc: usize,
}

impl<T: Num + Clone + PartialOrd + CheckedNeg + ToPrimitive + Debug> VirtualMachine<T> {
    pub fn new(inst: Vec<Inst<T>>) -> VirtualMachine<T> {
        VirtualMachine {
            stack: Vec::new(),
            mem: Vec::new(),
            inst,
            data: Zero::zero(),
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let ir = &self.inst[self.pc];
            self.pc += 1;

            println!("{} {}\t{:?}\t{:?}", self.pc, ir, &self.stack, &self.mem);

            match ir.inst {
                ByteCode::LOADC => self.push(ir.data.clone().unwrap()),
                ByteCode::LOAD => {
                    let addr: usize = self.pop()?.to_usize().unwrap();
                    self.push(self.mem[addr].clone());
                }
                ByteCode::STORE => {
                    let addr: usize = self.pop()?.to_usize().unwrap();
                    self.mem.resize(addr + 1, Zero::zero());
                    self.mem[addr] = self.top()?.clone();
                }
                ByteCode::POP => {
                    self.pop()?;
                }
                ByteCode::JUMP => {
                    self.pc = ir.data.clone().unwrap().to_usize().unwrap();
                }
                ByteCode::JUMPZ => {
                    let new_pc = ir.data.clone().unwrap().to_usize().unwrap();
                    if self.pop()? == Zero::zero() {
                        self.pc = new_pc;
                    }
                }
                ByteCode::HALT => {
                    println!("");
                    return Ok(());
                }
                ByteCode::ADD => {
                    let res = self.pop()? + self.pop()?;
                    self.push(res);
                }
                ByteCode::SUB => {
                    let res = self.pop()? - self.pop()?;
                    self.push(res);
                }
                ByteCode::MUL => {
                    let res = self.pop()? * self.pop()?;
                    self.push(res);
                }
                ByteCode::DIV => {
                    let res = self.pop()? / self.pop()?;
                    self.push(res);
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
                    let res = self.pop()? < self.pop()?;
                    self.push(VirtualMachine::bool_to_t(res));
                }
                ByteCode::LEQ => {
                    let res = self.pop()? <= self.pop()?;
                    self.push(VirtualMachine::bool_to_t(res));
                }
                ByteCode::GE => {
                    let res = self.pop()? < self.pop()?;
                    self.push(VirtualMachine::bool_to_t(res));
                }
                ByteCode::GEQ => {
                    let res = self.pop()? <= self.pop()?;
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
        }
    }

    #[inline]
    fn push(&mut self, data: T) {
        self.stack.push(data);
    }

    #[inline]
    fn top(&mut self) -> Result<&T, &'static str> {
        self.stack.last().ok_or_else(|| "Empty stack")
    }

    #[inline]
    fn pop(&mut self) -> Result<T, &'static str> {
        self.data = self.top()?.clone();
        self.stack.pop();
        Ok(self.data.clone())
    }

    pub fn bool_to_t(cond: bool) -> T {
        match cond {
            true => One::one(),
            false => Zero::zero(),
        }
    }

    pub fn t_to_bool(cond: T) -> bool {
        cond == One::one()
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::bytecode::*;
    use crate::vm::*;

    #[test]
    fn test_halt() {
        let instructions: Vec<Inst<i64>> = vec![Inst::new_inst(ByteCode::HALT)];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
    }

    #[test]
    fn test_loadc() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[0].to_vec(), &vm.stack);
    }

    #[test]
    fn test_load() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::STORE),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::LOAD),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[2, 2].to_vec(), &vm.stack);
        assert_eq!(&[2].to_vec(), &vm.mem);
    }

    #[test]
    fn test_store() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::STORE),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[2].to_vec(), &vm.stack);
    }

    #[test]
    fn test_pop() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::POP),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[2].to_vec(), &vm.stack);
    }

    #[test]
    fn test_jump() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::JUMP, 3),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_jumpz() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_data(ByteCode::JUMPZ, 4),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_add() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_inst(ByteCode::ADD),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[4].to_vec(), &vm.stack);
    }

    #[test]
    fn test_sub() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_inst(ByteCode::SUB),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[0].to_vec(), &vm.stack);
    }

    #[test]
    fn test_mul() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_inst(ByteCode::MUL),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[4].to_vec(), &vm.stack);
    }

    #[test]
    fn test_div() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_inst(ByteCode::DIV),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_mod() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_data(ByteCode::LOADC, 2),
            Inst::new_inst(ByteCode::MOD),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[0].to_vec(), &vm.stack);
    }

    #[test]
    fn test_and() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_inst(ByteCode::AND),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_or() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::OR),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_eq() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::EQ),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[0].to_vec(), &vm.stack);
    }

    #[test]
    fn test_neq() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::NEQ),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_le() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_data(ByteCode::LOADC, 0),
            Inst::new_inst(ByteCode::LE),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_neg() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_inst(ByteCode::NEG),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[-1].to_vec(), &vm.stack);
    }

    #[test]
    fn test_not() {
        let instructions: Vec<Inst<i64>> = vec![
            Inst::new_data(ByteCode::LOADC, 1),
            Inst::new_inst(ByteCode::NOT),
            Inst::new_inst(ByteCode::HALT),
        ];
        let mut vm: VirtualMachine<i64> = VirtualMachine::new(instructions);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(&[0].to_vec(), &vm.stack);
    }
}
