use crate::frontend::ast::*;
use crate::util::fold::*;

/// Traverses the AST searching for struct declarations
pub struct TypeCollector;

impl TypeCollector {
    pub fn collect_types(body: &Stmt) -> Option<Vec<EstaStruct>> {
        if let Some(s) = TypeCollector::fold_stmt(&(), body) {
            let s = s
                .into_iter()
                .enumerate()
                .map(move |(i, s)| EstaStruct { tag: i, ..s })
                .collect();
            Some(s)
        } else {
            None
        }
    }
}

impl Fold for TypeCollector {
    type UpT = Vec<EstaStruct>;
    type DownT = ();

    fn reduce(children: Vec<Option<Self::UpT>>) -> Option<Self::UpT> {
        if children.len() > 0 {
            Some(children.into_iter().flatten().flatten().collect())
        } else {
            None
        }
    }

    fn fold_struct(_: &Self::DownT, id: &String, fields: &Vec<Identifier>) -> Option<Self::UpT> {
        Some(vec![EstaStruct::new(Stmt::Struct(
            id.clone(),
            fields.clone(),
        ))])
    }
}
