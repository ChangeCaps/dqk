use crate::{
    CloseBrace, CloseParen, ColonEqual, Comma, Eol, Equal, Expr, Ident, Ln, OpenBrace, OpenParen,
    Punctuated, SemiColon, Span, Spanned, SpannedOption, Tupled, Type, Where,
};

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

#[derive(Clone, Debug)]
pub struct Block {
    pub open: OpenBrace,
    pub stmts: Vec<Stmt>,
    pub close: CloseBrace,
}

impl Spanned for Block {
    fn span(&self) -> Span {
        self.open.span() | self.close.span()
    }
}

#[derive(Clone, Debug, Spanned)]
pub struct Argument {
    pub ident: Ident,
    pub semi_colon: SemiColon,
    pub ty: Type,
}

#[derive(Clone, Debug, Spanned)]
pub enum ListenerArgument {
    Expanded(Type),
    Args(Argument),
}

#[derive(Clone, Debug, Spanned)]
pub struct ListenerArguments {
    pub open: OpenParen,
    pub args: Punctuated<ListenerArgument, Comma>,
    pub close: CloseParen,
}

#[derive(Clone, Debug, Spanned)]
pub struct WhereClause {
    pub _where: Where,
    pub bounds: Punctuated<Expr, Comma>,
}

#[derive(Clone, Debug, Spanned)]
pub struct DefaultEvent {
    pub ident: Ident,
}

#[derive(Clone, Debug, Spanned)]
pub struct DefaultEvents {
    pub events: Tupled<Punctuated<DefaultEvent, Comma>>,
}

#[derive(Clone, Debug, Spanned)]
pub struct LnStmt {
    pub ln: Ln,
    pub ident: Ident,
    pub default_events: SpannedOption<DefaultEvents>,
    pub args: ListenerArguments,
    pub where_clause: SpannedOption<WhereClause>,
    pub block: Block,
}

#[derive(Clone, Debug, Spanned)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug, Spanned)]
pub enum Stmt {
    Ln(LnStmt),
    AssignNew(AssignNewStmt),
    Assign(AssignStmt),
    Expr(ExprStmt),
}
