use crate::frontend::ast::*;

/// Traverse Trait
///
/// This trait is used to perform arbitrary pre and post order traversals of a
/// the abstract syntax tree. This trait provides abstract functions to traverse
/// the children of any given node, and the implementor is free do perform
/// arbitrary operations at each statement/expression if necessary.
///
/// To preform a pre-order traversal, call walk_* after the logic.
/// To preform a pre-order traversal, call walk_* before the logic.
pub trait Traverse<T> {
    // TODO: Currently these traversals carry no information downwards, so they
    //  wouldn't work for a "top-down" algorithm
    // TODO: Make them return Option<T> No this won't work because they have no way to "fold"
    // Or... you could make an Option<Vec<T>>
    // By the way, you can make dynamic types return a defined trait. Aka,
    // foo<T>() -> Self::ReturnValue
    fn traverse_stmt(s: &mut Stmt) -> Option<T>;
    fn traverse_expr(e: &mut Expr) -> Option<T>;
}

pub fn walk_stmt<T: Clone, U: Traverse<T>>(s: &mut Stmt) -> Option<T> {
    //    let stream = match s {
    //        Stmt::Block(body) => body
    //            .iter()
    //            .cloned()
    //            .map(|mut x| U::traverse_stmt(&mut *x))
    //            .collect(),
    //        Stmt::FlatBlock(body) => body
    //            .iter()
    //            .cloned()
    //            .map(|mut x| U::traverse_stmt(&mut *x))
    //            .collect(),
    //        Stmt::If(test, body, alter) => vec![
    //            U::traverse_expr(test),
    //            U::traverse_stmt(body),
    //            U::traverse_stmt(alter),
    //        ],
    //        Stmt::While(test, body) => vec![U::traverse_expr(test), U::traverse_stmt(body)],
    //        Stmt::Return(expr) => expr
    //            .iter()
    //            .cloned()
    //            .map(|mut x| U::traverse_expr(&mut *x))
    //            .collect(),
    //        Stmt::Declaration(_) => None,
    //        Stmt::FunDecl(_, _, body) => vec![U::traverse_stmt(body)],
    //        Stmt::Assignment(lhs, rhs) => vec![U::traverse_expr(lhs), U::traverse_expr(rhs)],
    //        Stmt::Struct(_, _) => None,
    //    }
    //    let stream = Vec::new();
    //    stream.iter().cloned().flatten().collect()
    None
}

pub fn walk_expr<T: Clone, U: Traverse<T>>(e: &mut Expr) -> Option<T> {
    //    let stream = match e {
    //        Expr::Id(_) => None,
    //        Expr::Literal(_) => None,
    //        Expr::BinaryOp(lhs, _, rhs) => vec![U::traverse_expr(lhs), U::traverse_expr(rhs)],
    //        Expr::UnaryOp(_, rhs) => vec![U::traverse_expr(rhs)],
    //        Expr::FunCall(_, args) => args
    //            .iter()
    //            .cloned()
    //            .map(|mut x| U::traverse_expr(&mut x))
    //            .collect(),
    //    };
    //    stream.iter().cloned().flatten().collect()
    None
}
