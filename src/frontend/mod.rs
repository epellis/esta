mod scope;

use crate::ast::Stmt;

lalrpop_mod!(grammar);

pub fn run(input: &str) -> Result<(), &'static str> {
    // TODO: Use LALRPOP for error analysis
    // TODO: Write a custom lexer for comments
    let stmts = grammar::StmtsParser::new()
        .parse(input)
        .map_err(|_| "Parsing Error")?;
    let stmts = Stmt::Block(stmts);
    let stmts = scope::scope(stmts)?;
    println!("{}", stmts);
    Ok(())
}
