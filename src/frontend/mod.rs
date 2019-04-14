use crate::ast;

lalrpop_mod!(grammar);

pub fn run(input: &str) {
    let expr = grammar::ExprParser::new().parse(input).unwrap();
    println!("{}", expr);
}
