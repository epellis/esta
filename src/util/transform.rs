use crate::frontend::ast::*;

// TODO: This trait will not work until each

/// This is a common design pattern used to perform an operation on a
/// treelike structure.
/// If the route is not implemented, the identify will be returned.
pub trait Transform<C> {
    fn route_block(ctx: &mut C, body: Vec<Box<Stmt>>) -> Stmt {
        Stmt::Block(body)
    }
    fn route_if(mut ctx: &mut C, test: ExprNode, body: Box<Stmt>, alter: Box<Stmt>) -> Stmt {
        Stmt::If(test, body, alter)
    }
    fn route_while(mut ctx: &mut C, test: ExprNode, body: Box<Stmt>) -> Stmt {
        Stmt::While(test, body)
    }
    fn route_return(ctx: &mut C, value: Option<ExprNode>) -> Stmt {
        Stmt::Return(value)
    }
    fn route_declaration(mut ctx: &mut C, id: String) -> Stmt {
        Stmt::Declaration(id)
    }
    fn route_fun(ctx: &mut C, id: String, args: Vec<ExprNode>, ret: Type, body: Box<Stmt>) -> Stmt {
        Stmt::FunDecl(id, args, ret, body)
    }
    fn route_assignment(ctx: &mut C, lhs: ExprNode, rhs: ExprNode) -> Stmt {
        Stmt::Assignment(lhs, rhs)
    }
    fn route_identifier(ctx: &mut C, id: String) -> ExprNode {
        ExprNode::new_untyped(Expr::Identifier(id))
    }
    fn route_literal(ctx: &mut C, lit: Literal) -> ExprNode {
        ExprNode::new_untyped(Expr::Literal(lit))
    }
    fn route_binary(ctx: &mut C, lhs: ExprNode, op: Opcode, rhs: ExprNode) -> ExprNode {
        ExprNode::new_untyped(Expr::BinaryOp(lhs, op, rhs))
    }
    fn route_unary(ctx: &mut C, op: Opcode, rhs: ExprNode) -> ExprNode {
        ExprNode::new_untyped(Expr::UnaryOp(op, rhs))
    }
    fn route_funcall(ctx: &mut C, id: String, args: Vec<ExprNode>) -> ExprNode {
        ExprNode::new_untyped(Expr::FunCall(id, args))
    }
}

pub fn transform_stmt<C, D: Transform<C>>(ctx: &mut C, s: Stmt) -> Stmt {
    match s {
        Stmt::Block(body) => D::route_block(ctx, body),
        Stmt::FlatBlock(body) => D::route_block(ctx, body),
        Stmt::If(test, body, alter) => D::route_if(ctx, test, body, alter),
        Stmt::While(test, body) => D::route_while(ctx, test, body),
        Stmt::Return(value) => D::route_return(ctx, value),
        Stmt::Declaration(id) => D::route_declaration(ctx, id),
        Stmt::FunDecl(id, args, ret, body) => D::route_fun(ctx, id, args, ret, body),
        Stmt::Assignment(lhs, rhs) => D::route_assignment(ctx, lhs, rhs),
    }
}

fn transform_expr<C, D: Transform<C>>(ctx: &mut C, e: ExprNode) -> ExprNode {
    match *e.expr {
        Expr::Identifier(id) => D::route_identifier(ctx, id),
        Expr::Literal(lit) => D::route_literal(ctx, lit),
        Expr::BinaryOp(lhs, op, rhs) => D::route_binary(ctx, lhs, op, rhs),
        Expr::UnaryOp(op, rhs) => D::route_unary(ctx, op, rhs),
        Expr::FunCall(id, args) => D::route_funcall(ctx, id, args),
    }
}
