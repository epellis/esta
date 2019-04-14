use crate::ast;

lalrpop_mod!(grammar);

pub fn run(input: &str) {
    let stmts = grammar::StmtsParser::new().parse(input).unwrap();
    for stmt in stmts {
        println!("{}", stmt);
    }
}
