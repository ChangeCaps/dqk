use dqk_macro::Spanned;

use crate::{
    Asterisk, CloseParen, Comma, EqualEqual, Ident, LitFloat, LitInt, LitStr, Minus, OpenParen,
    Period, Plus, Punctuated, Slash,
};

#[derive(Clone, Debug, Spanned)]
pub struct ParenExpr {
    pub open: OpenParen,
    pub expr: Box<Expr>,
    pub close: CloseParen,
}

#[derive(Clone, Debug, Spanned)]
pub enum LiteralExpr {
    Integer(LitInt),
    Float(LitFloat),
    String(LitStr),
}

#[derive(Clone, Debug, Spanned)]
pub struct MemberExpr {
    pub expr: Box<Expr>,
    pub period: Period,
    pub ident: Ident,
}

#[derive(Clone, Debug, Spanned)]
pub struct CallExpr {
    pub expr: Box<Expr>,
    pub args: Punctuated<Expr, Comma>,
}

#[derive(Clone, Debug, Spanned)]
pub enum BinOp {
    Add(Plus),
    Sub(Minus),
    Mul(Asterisk),
    Div(Slash),
    Eq(EqualEqual),
}

impl BinOp {
    pub fn precedence(&self) -> u32 {
        match self {
            Self::Mul(_) | Self::Div(_) => 12,
            Self::Add(_) | Self::Sub(_) => 11,
            Self::Eq(_) => 8,
        }
    }
}

#[derive(Clone, Debug, Spanned)]
pub struct BinOpExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

#[derive(Clone, Debug, Spanned)]
pub enum Expr {
    Paren(ParenExpr),
    Literal(LiteralExpr),
    Variable(Ident),
    Member(MemberExpr),
    Call(CallExpr),
    BinOp(BinOpExpr),
}
