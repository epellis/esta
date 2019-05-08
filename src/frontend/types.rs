use crate::frontend::ast::*;
use crate::frontend::fold::*;
use crate::frontend::traversal::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EstaType {
    Number,
    Boolean,
    Nil,
    Compound(String),
}

/// Traverses the AST searching for struct declarations
pub struct TypeCollector;

impl TypeCollector {
    pub fn collect_types(body: &Stmt) -> Option<Vec<Stmt>> {
        TypeCollector::fold_stmt(&(), body)
    }
}

impl Fold for TypeCollector {
    type UpT = Vec<Stmt>;
    type DownT = ();

    fn reduce(children: Vec<Option<Self::UpT>>) -> Option<Self::UpT> {
        if children.len() > 0 {
            let mut new_children = Vec::new();
            for child in children {
                if let Some(child) = child {
                    new_children.extend(child)
                }
            }
            Some(new_children)
        } else {
            None
        }
    }

    fn fold_struct(down: &Self::DownT, id: &String, fields: &Vec<Identifier>) -> Option<Self::UpT> {
        Some(vec![Stmt::Struct(id.clone(), fields.clone())])
    }
}

pub struct TypeAssistant;

impl TypeAssistant {
    pub fn infer_types(body: &mut Stmt) {
        Self::traverse_stmt(body);
    }
}

// TODO: This is a bit complicated because most times a variable that has an
//  unknown type is not directly next to one that is. A potential solution
//  to this is to build a directed graph and use a BFS to find type
impl Traverse<Vec<String>> for TypeAssistant {
    fn traverse_stmt(s: &mut Stmt) -> Option<Vec<String>> {
        walk_stmt::<Vec<String>, TypeAssistant>(s)
    }
    fn traverse_expr(e: &mut Expr) -> Option<Vec<String>> {
        walk_expr::<Vec<String>, TypeAssistant>(e)
    }
}
