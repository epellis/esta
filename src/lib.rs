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
extern crate slog;
extern crate sloggers;

use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

pub fn run(input: &str) -> Result<(), &'static str> {
    //    info!(LOGGER, "Hello World!");
    let stmts = frontend::run(input)?;
    let inst = backend::generate(stmts)?;
    //    for (j, i) in inst.iter().enumerate() {
    //        println!("{: >3} {}", j, i);
    //    }
    //    for i in inst.iter() {
    //        println!("{}", i);
    //    }
    let mut vm = vm::VirtualMachine::new(inst);
    vm.run()?;
    Ok(())
}

lazy_static! {
    pub static ref LOGGER: slog::Logger = {
        let mut builder = TerminalLoggerBuilder::new();
        builder.level(Severity::Debug);
        builder.destination(Destination::Stdout);

        builder.build().unwrap()
    };
}
