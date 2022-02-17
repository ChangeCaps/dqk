use dqk_macro::Spanned;

use crate::{LitFloat, LitInt, LitStr};

#[derive(Clone, Debug, Spanned)]
pub enum LiteralExpr {
    Integer(LitInt),
    Float(LitFloat),
    String(LitStr),
}

#[derive(Clone, Debug, Spanned)]
pub enum Expr {
    Literal(LiteralExpr),
}
