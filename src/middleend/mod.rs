mod types;

use crate::frontend::ast::*;
use crate::middleend::types::*;

pub struct MetaData {
    pub structs: Vec<EstaStruct>,
}

impl MetaData {
    pub fn new() -> MetaData {
        let structs = Vec::new();
        MetaData { structs }
    }
}

pub fn run(stmts: Stmt) -> Result<(Stmt, MetaData), &'static str> {
    let structs = TypeCollector::collect_types(&stmts);
    println!("Structs: {:?}", structs);
    Ok((stmts, MetaData::new()))
}

// TODO: Discover all variables in a given scope
// TODO: Discover and check function arity
