use super::ast::*;

/// Visitor Pattern
///
/// The Visitor Pattern allows for a default behaviour to be defined when the
/// AST is walked. This means that when new walks of the AST are done, the
/// specific walk only needs to modify functions specific to itself.
pub trait Visitor {
    fn visit_stmt(&mut self, s: &Stmt) {
        walk_stmt(self, s);
    }
    fn visit_expr(&mut self, e: &ExprNode) {
        walk_expr(self, e)
    }
}

pub fn walk_stmt<V: ?Sized + Visitor>(v: &mut V, s: &Stmt) {
    match s {
        Stmt::While(test, body) => {
            v.visit_expr(test);
            v.visit_stmt(body);
        }
        Stmt::If(test, body, alt) => {
            v.visit_expr(test);
            v.visit_stmt(body);
            v.visit_stmt(alt);
        }
        Stmt::Return(value) => {
            if let Some(value) = value {
                v.visit_expr(value);
            }
        }
        Stmt::Declaration(id, rhs) => {
            v.visit_expr(rhs);
        }
        Stmt::FunDecl(id, params, ret, body) => {
            // TODO: Should we walk params?
            v.visit_stmt(body);
        }
        Stmt::Assignment(lhs, rhs) => {
            v.visit_expr(lhs);
            v.visit_expr(rhs);
        }
        _ => {} // TODO: Synthesize For Loop into While Loop
    }
}

pub fn walk_expr<V: ?Sized + Visitor>(v: &mut V, e: &ExprNode) {
    match &*e.expr {
        Expr::Identifier(id) => {}
        Expr::Literal(literal) => {}
        Expr::BinaryOp(lhs, op, rhs) => {
            v.visit_expr(lhs);
            v.visit_expr(rhs);
        }
        Expr::UnaryOp(op, rhs) => {
            v.visit_expr(rhs);
        }
        Expr::FunCall(id, params) => {
            // TODO: Should we walk params?
        }
    }
}
