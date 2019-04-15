use crate::ast::{Expr, Literal, Stmt};

pub fn infer(stmts: Stmt) -> Result<Stmt, &'static str> {
    infer_stmt(&stmts).unwrap();
    Ok(stmts)
}

fn infer_stmt(stmt: &Stmt) -> Result<(), &'static str> {
    match stmt {
        Stmt::Declaration(id, rhs) => {
            // TODO: Add expression id's type to global table
        }
        Stmt::Assignment(lhs, rhs) => {
            combine_type(lhs, rhs); // TODO: Assert types match
        }
        Stmt::FunDecl(id, params, body) => {
            // TODO: Assign each param a type
            // TODO: Add function's return type to global table
            infer_stmt(body)?;
        }
        Stmt::Block(stmts) => {
            for stmt in stmts {
                infer_stmt(stmt)?;
            }
        }
        Stmt::If(test, result, alternate) => {
            infer_expr(test);
            infer_stmt(result)?;
            infer_stmt(alternate)?;
        }
        Stmt::While(test, body) => {
            infer_expr(test);
            infer_stmt(body)?;
        }
        Stmt::For(init, test, increment, body) => {
            if let Some(init) = init {
                infer_stmt(init)?;
            }
            if let Some(test) = test {
                infer_expr(test);
            }
            if let Some(increment) = increment {
                infer_stmt(increment)?;
            }
            infer_stmt(body)?;
        }
        Stmt::ImpureCall(funcall) => {
            infer_expr(funcall);
        }
        Stmt::Return(_) => {} // TODO: Assert return value matches return type of fun
        Stmt::Break => {}
        Stmt::Continue => {}
    }
    Ok(())
}

// TODO: Memoize results and store in ast internal nodes
fn infer_expr(expr: &Expr) -> &'static str {
    match expr {
        Expr::Literal(Literal::Number(_)) => "Number",
        Expr::Literal(Literal::Boolean(_)) => "Boolean",
        Expr::Literal(Literal::String(_)) => "String",
        Expr::Literal(Literal::Nil) => "Nil",
        Expr::UnaryOp(_, expr) => infer_expr(expr),
        Expr::BinaryOp(lhs, _, rhs) => combine_type(lhs, rhs),
        Expr::FunCall(_, _) => "Function", // TODO: Lookup function return in global table
        Expr::Identifier(_) => "Identifier", // TODO: Lookup identifier in global table
    }
}

fn combine_type(lhs: &Expr, rhs: &Expr) -> &'static str {
    match (infer_expr(lhs), infer_expr(rhs)) {
        (lhs, rhs) if lhs == rhs => lhs,
        _ => "Unknown",
    }
}
