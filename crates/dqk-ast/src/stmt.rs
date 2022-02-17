use dqk_macro::Spanned;

use crate::{ColonEqual, Eol, Equal, Expr, Ident};

#[derive(Clone, Debug, Spanned)]
pub struct AssignNewStmt {
    pub ident: Ident,
    pub equal: ColonEqual,
    pub expr: Expr,
    pub eol: Eol,
}

#[derive(Clone, Debug, Spanned)]
pub struct AssignStmt {
    pub ident: Ident,
    pub equal: Equal,
    pub expr: Expr,
    pub eol: Eol,
}

#[derive(Clone, Debug, Spanned)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug, Spanned)]
pub enum Stmt {
    AssignNew(AssignNewStmt),
    Assign(AssignStmt),
    Expr(ExprStmt),
}
