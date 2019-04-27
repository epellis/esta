pub mod backend;
pub mod frontend;
pub mod middleend;
pub mod vm;

#[macro_use]
extern crate lalrpop_util;

pub fn run(input: &str) -> Result<(), &'static str> {
    let stmts = frontend::run(input)?;
    backend::generate(stmts)?;
    //    let mut vm = vm::VirtualMachine::new();
    Ok(())
}
