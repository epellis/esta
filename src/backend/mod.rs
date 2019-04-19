use crate::frontend::ast::{Expr, ExprNode, Stmt};

pub fn generate(stmts: Stmt) -> Result<(), &'static str> {
    println!("Generating code...");
    Ok(())
}

fn gen_expr(expr: &Expr) {
    use crate::frontend::ast::Expr::*;
    use crate::frontend::ast::Opcode::*;

    // TODO: Add a static mapping from Opcodes to Bytecodes?

    //    match expr {
    //        BinaryOp(lhs, op, rhs) => {}
    //    }
}
