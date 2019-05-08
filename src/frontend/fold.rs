use crate::frontend::ast::*;

/// Fold Trait
///
/// This trait traverses over an abstract syntax tree and returns a single value.
/// Typically this is used to either collect data on the abstract syntax tree
/// or to create a new one with changes.
///
/// The idea is that the user supplies functions that handle what happens to the
/// data at return points where there are multiple options
pub trait Fold {
    type UpT; // This value is passed down before the traversal
    type DownT; // This value is passed up after the traversal

    fn reduce(children: Vec<Option<Self::UpT>>) -> Option<Self::UpT> {
        None
    }

    fn fold_block(down: &Self::DownT, body: &Vec<Box<Stmt>>, is_scope: &bool) -> Option<Self::UpT> {
        let children = body.iter().map(|b| Self::fold_stmt(down, b)).collect();
        Self::reduce(children)
    }

    fn fold_if(
        down: &Self::DownT,
        test: &Box<Expr>,
        body: &Box<Stmt>,
        alter: &Box<Stmt>,
    ) -> Option<Self::UpT> {
        let children = vec![
            Self::fold_expr(down, test),
            Self::fold_stmt(down, body),
            Self::fold_stmt(down, alter),
        ];
        Self::reduce(children)
    }

    fn fold_while(down: &Self::DownT, test: &Box<Expr>, body: &Box<Stmt>) -> Option<Self::UpT> {
        let children = vec![Self::fold_expr(down, test), Self::fold_stmt(down, body)];
        Self::reduce(children)
    }

    fn fold_return(down: &Self::DownT, value: &Option<Box<Expr>>) -> Option<Self::UpT> {
        if let Some(expr) = value {
            Self::fold_expr(down, expr)
        } else {
            None
        }
    }

    fn fold_declaration(down: &Self::DownT, id: &Identifier) -> Option<Self::UpT> {
        None
    }

    fn fold_fundecl(
        down: &Self::DownT,
        id: &Identifier,
        params: &Vec<Identifier>,
        body: &Box<Stmt>,
    ) -> Option<Self::UpT> {
        Self::fold_stmt(down, body)
    }

    fn fold_assignment(down: &Self::DownT, lhs: &Box<Expr>, rhs: &Box<Expr>) -> Option<Self::UpT> {
        let children = [rhs, lhs]
            .iter()
            .map(|e| Self::fold_expr(down, e))
            .collect();
        Self::reduce(children)
    }

    fn fold_struct(down: &Self::DownT, id: &String, fields: &Vec<Identifier>) -> Option<Self::UpT> {
        None
    }

    fn fold_id(down: &Self::DownT, id: &Identifier) -> Option<Self::UpT> {
        None
    }

    fn fold_literal(down: &Self::DownT, lit: &Literal) -> Option<Self::UpT> {
        None
    }

    fn fold_binary(
        down: &Self::DownT,
        lhs: &Box<Expr>,
        op: &Opcode,
        rhs: &Box<Expr>,
    ) -> Option<Self::UpT> {
        let children = [lhs, rhs]
            .iter()
            .map(|e| Self::fold_expr(down, e))
            .collect();
        Self::reduce(children)
    }

    fn fold_unary(down: &Self::DownT, op: &Opcode, rhs: &Box<Expr>) -> Option<Self::UpT> {
        Self::fold_expr(down, rhs)
    }

    fn fold_funcall(down: &Self::DownT, id: &String, args: &Vec<Expr>) -> Option<Self::UpT> {
        let children = args.iter().map(|e| Self::fold_expr(down, e)).collect();
        Self::reduce(children)
    }

    fn fold_stmt(down: &Self::DownT, s: &Stmt) -> Option<Self::UpT> {
        match s {
            Stmt::Block(body, is_scope) => Self::fold_block(down, body, is_scope),
            Stmt::If(test, body, alter) => Self::fold_if(down, test, body, alter),
            Stmt::While(test, body) => Self::fold_while(down, test, body),
            Stmt::Return(value) => Self::fold_return(down, value),
            Stmt::Declaration(id) => Self::fold_declaration(down, id),
            Stmt::FunDecl(id, params, body) => Self::fold_fundecl(down, id, params, body),
            Stmt::Assignment(lhs, rhs) => Self::fold_assignment(down, lhs, rhs),
            Stmt::Struct(id, fields) => Self::fold_struct(down, id, fields),
        }
    }
    fn fold_expr(down: &Self::DownT, e: &Expr) -> Option<Self::UpT> {
        match e {
            Expr::Id(id) => Self::fold_id(down, id),
            Expr::Literal(lit) => Self::fold_literal(down, lit),
            Expr::BinaryOp(lhs, op, rhs) => Self::fold_binary(down, lhs, op, rhs),
            Expr::UnaryOp(op, rhs) => Self::fold_unary(down, op, rhs),
            Expr::FunCall(id, args) => Self::fold_funcall(down, id, args),
        }
    }
}
