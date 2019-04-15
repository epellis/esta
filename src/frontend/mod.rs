mod infer_types;
mod scope;

use crate::ast::Stmt;

lalrpop_mod!(grammar);

pub fn run(input: &str) -> Result<(), &'static str> {
    // TODO: Use LALRPOP for error analysis
    let stmts = grammar::StmtsParser::new()
        .parse(input)
        .map_err(|_| "Parsing Error")?;
    let stmts = Stmt::Block(stmts);
    let stmts = scope::scope(stmts)?;
    let result = infer_types::infer(stmts)?;
    Ok(())
}

//pub fn infer(stmts: Vec<Box<Stmt>>) -> Result<Stmt, &'static str> {
//    let stmts = Stmt::Block(stmts);
//    infer_stmt(&stmts).unwrap();
//    Ok(stmts)
//}
//for stmt in &stmts {
//println!("{}", stmt);
//}
