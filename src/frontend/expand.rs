use super::ast::*;
use crate::util::transform::{transform_stmt, Transform};

pub struct Expand {}

impl Expand {
    pub fn expand(stmt: Stmt) -> Result<Stmt, &'static str> {
        //        let mut ctx = ();
        //        let stmt = transform_stmt::<(), Expand>(&mut ctx, stmt);
        Ok(stmt)
    }
}

impl Transform<()> for Expand {
    fn route_block(ctx: &mut (), body: Vec<Box<Stmt>>) -> Stmt {
        Stmt::Block(body)
    }
}
