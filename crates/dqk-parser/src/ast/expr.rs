use dqk_macro::Spanned;

use crate::{EqualEqual, LitFloat, LitInt, LitStr};

#[derive(Clone, Debug, Spanned)]
pub enum LiteralExpr {
    Integer(LitInt),
    Float(LitFloat),
    String(LitStr),
}

#[derive(Clone, Debug, Spanned)]
pub enum BinOp {
    Eq(EqualEqual),
}

#[derive(Clone, Debug, Spanned)]
pub enum Expr {
    Literal(LiteralExpr),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}
