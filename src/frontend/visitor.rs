//use super::ast::*;
//
////pub type VisitResult<T> = ::std::result::Result<T, Box<::std::error::Error>>;
//
///// Visitor Pattern
///// Inspired by RustC MIR Project
/////
///// The Visitor Pattern allows for a default behaviour to be defined when the
///// AST is walked. This means that when new walks of the AST are done, the
///// specific walk only needs to modify functions specific to itself.
//pub trait Visitor<T> {
//    fn visit_stmt(&mut self, s: &Stmt);
//    fn visit_expr(&mut self, e: &ExprNode);
//}
//
//// TODO: WalkStmt/Expr need to have error handling/passing abilities
//pub fn walk_stmt<T, V: ?Sized + Visitor<T>>(v: &mut V, s: &Stmt) {
//    match s {
//        Stmt::Block(stmts) => {
//            for stmt in stmts {
//                v.visit_stmt(stmt);
//            }
//        }
//        Stmt::FlatBlock(stmts) => {
//            for stmt in stmts {
//                v.visit_stmt(stmt);
//            }
//        }
//        Stmt::While(test, body) => {
//            v.visit_expr(test);
//            v.visit_stmt(body);
//        }
//        Stmt::If(test, body, alt) => {
//            v.visit_expr(test);
//            v.visit_stmt(body);
//            v.visit_stmt(alt);
//        }
//        Stmt::Return(value) => {
//            if let Some(value) = value {
//                v.visit_expr(value);
//            }
//        }
//        Stmt::Declaration(id) => {}
//        Stmt::FunDecl(id, params, ret, body) => {
//            for param in params {
//                v.visit_expr(param);
//            }
//            v.visit_stmt(body);
//        }
//        Stmt::Assignment(lhs, rhs) => {
//            v.visit_expr(lhs);
//            v.visit_expr(rhs);
//        }
//    }
//}
//
//pub fn walk_expr<T, V: ?Sized + Visitor<T>>(v: &mut V, e: &ExprNode) {
//    match &*e.expr {
//        Expr::Identifier(id) => {}
//        Expr::Literal(literal) => {}
//        Expr::BinaryOp(lhs, op, rhs) => {
//            v.visit_expr(lhs);
//            v.visit_expr(rhs);
//        }
//        Expr::UnaryOp(op, rhs) => {
//            v.visit_expr(rhs);
//        }
//        Expr::FunCall(id, params) => {
//            for param in params {
//                v.visit_expr(param);
//            }
//        }
//    }
//}
