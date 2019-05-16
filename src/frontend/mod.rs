pub mod ast;
// pub mod fold;
// pub mod types;

#[cfg(test)]
mod tests;

use self::ast::Stmt;
// use self::types::TypeAssistant;
// use crate::frontend::types::TypeCollector;

lalrpop_mod!(grammar);

pub fn run(input: &str) -> Result<Stmt, &'static str> {
    // TODO: Use LALRPOP for error analysis
    // TODO: Write a custom lexer for comments
    let stmts = grammar::StmtsParser::new()
        .parse(input)
        .map_err(|_| "Parsing Error")?;
    let stmts = Stmt::Block(stmts, false);
    Ok(stmts)
}
