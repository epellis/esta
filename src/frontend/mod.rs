pub mod ast;
pub mod fold;
pub mod traversal;
mod types;

#[cfg(test)]
mod tests;

use self::ast::Stmt;
use self::types::TypeAssistant;
use crate::frontend::types::TypeCollector;

lalrpop_mod!(grammar);

pub fn run(input: &str) -> Result<Stmt, &'static str> {
    // TODO: Use LALRPOP for error analysis
    // TODO: Write a custom lexer for comments
    let stmts = grammar::StmtsParser::new()
        .parse(input)
        .map_err(|_| "Parsing Error")?;
    let mut stmts = Stmt::Block(stmts, false);
    TypeAssistant::infer_types(&mut stmts);
    let structs = TypeCollector::collect_types(&stmts);
    println!("Structs: {:?}", structs);
    // TODO: Discover all variables in a given scope
    // TODO: Discover and check function arity
    Ok(stmts)
}
