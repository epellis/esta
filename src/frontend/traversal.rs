use crate::ast::*;

/// Traverse Trait
///
/// This trait is used to perform arbitrary pre and post order traversals of a
/// the abstract syntax tree. This trait provides abstract functions to traverse
/// the children of any given node, and the implementor is free do perform
/// arbitrary operations at each statement/expression if necessary.
///
/// To preform a pre-order traversal, call walk_* after the logic.
/// To preform a pre-order traversal, call walk_* before the logic.
trait Traverse<T> {
    fn traverse_stmt(&mut self, s: &Stmt) -> Vec<T>;
    fn traverse_expr(&mut self, s: &Expr) -> Vec<T>;
}
