pub mod backend;
pub mod frontend;
pub mod middleend;
pub mod util;
pub mod vm;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate log;
extern crate env_logger;

pub fn run(input: &str) -> Result<(), &'static str> {
    debug!("Hello World");

    let stmts = frontend::run(input)?;
    let inst = backend::generate(stmts)?;
    for (j, i) in inst.iter().enumerate() {
        debug!("{: >3} {}", j, i);
    }
    let mut vm = vm::VirtualMachine::new(inst);
    vm.run()?;
    Ok(())
}
