use crate::{
    BinOp, BinOpExpr, CallExpr, Error, Expr, Ident, LitFloat, LitInt, LitStr, LiteralExpr,
    MemberExpr, ParenExpr, Parse, Parser, Punctuated, Result, Symbol, TokenKind,
};

impl TokenKind {
    pub fn starts_expr(&self) -> bool {
        match self {
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::Ident(_)
            | TokenKind::Symbol(Symbol::OpenParen) => true,
            _ => false,
        }
    }
}

impl Parse for ParenExpr {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            open: parser.parse()?,
            expr: parser.parse()?,
            close: parser.parse()?,
        })
    }
}

impl Parse for LiteralExpr {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.next_token()?;

        match tok.kind() {
            TokenKind::Integer(value) => Ok(Self::Integer(LitInt::new(value, tok.span()))),
            TokenKind::Float(value) => Ok(Self::Float(LitFloat::new(value, tok.span()))),
            TokenKind::String(value) => Ok(Self::String(LitStr::new(value, tok.span()))),
            kind => {
                Err(Error::new("expected literal")
                    .with_hint(format!("found '{:?}'", kind), tok.span()))
            }
        }
    }
}

fn parse_term(parser: &mut Parser, ident: Option<Ident>) -> Result<Expr> {
    if let Some(ident) = ident {
        return Ok(Expr::Variable(ident));
    }

    let tok = parser.peek_token()?;

    match tok.kind() {
        TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::String(_) => {
            Ok(Expr::Literal(parser.parse()?))
        }
        TokenKind::Ident(_) => Ok(Expr::Variable(parser.parse()?)),
        TokenKind::Symbol(Symbol::OpenParen) => Ok(Expr::Paren(parser.parse()?)),
        _ => Err(Error::new("expected expression").with_fount_hint(tok)),
    }
}

fn parse_member(parser: &mut Parser, ident: Option<Ident>) -> Result<Expr> {
    let mut expr = parse_term(parser, ident)?;

    while parser.peek_token()?.kind() == TokenKind::Symbol(Symbol::Period) {
        expr = Expr::Member(MemberExpr {
            expr: Box::new(expr),
            period: parser.parse()?,
            ident: parser.parse()?,
        });
    }

    Ok(expr)
}

fn parse_call(parser: &mut Parser, ident: Option<Ident>) -> Result<Expr> {
    let expr = parse_member(parser, ident)?;

    let tok = parser.peek_token()?;

    match tok.kind() {
        kind if kind.starts_expr() => Ok(Expr::Call(CallExpr {
            expr: Box::new(expr),
            args: Punctuated::parse_terminated(parser, |kind: &TokenKind| {
                !kind.starts_expr() && *kind != TokenKind::Symbol(Symbol::Comma)
            })?,
        })),
        _ => Ok(expr),
    }
}

impl BinOp {
    pub fn try_parse(parser: &mut Parser) -> Result<Option<Self>> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Symbol(Symbol::Plus) => Ok(Some(Self::Add(parser.parse()?))),
            TokenKind::Symbol(Symbol::Minus) => Ok(Some(Self::Sub(parser.parse()?))),
            TokenKind::Symbol(Symbol::Asterisk) => Ok(Some(Self::Mul(parser.parse()?))),
            TokenKind::Symbol(Symbol::Slash) => Ok(Some(Self::Div(parser.parse()?))),
            TokenKind::Symbol(Symbol::EqualEqual) => Ok(Some(Self::Eq(parser.parse()?))),
            _ => Ok(None),
        }
    }
}

fn parse_bin_op(parser: &mut Parser, ident: Option<Ident>) -> Result<Expr> {
    let mut lhs = parse_call(parser, ident)?;

    while let Some(op) = BinOp::try_parse(parser)? {
        match lhs {
            Expr::BinOp(BinOpExpr {
                lhs: lhs_lhs,
                op: lhs_op,
                rhs: lhs_rhs,
            }) if op.precedence() > lhs_op.precedence() => {
                lhs = Expr::BinOp(BinOpExpr {
                    lhs: lhs_lhs,
                    op: lhs_op,
                    rhs: Box::new(Expr::BinOp(BinOpExpr {
                        lhs: lhs_rhs,
                        op,
                        rhs: Box::new(parse_call(parser, None)?),
                    })),
                });
            }
            _ => {
                lhs = Expr::BinOp(BinOpExpr {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(parse_call(parser, None)?),
                })
            }
        }
    }

    Ok(lhs)
}

impl Parse for Expr {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_bin_op(parser, None)
    }
}

impl Expr {
    pub fn parse_with_ident(parser: &mut Parser, ident: Ident) -> Result<Self> {
        parse_bin_op(parser, Some(ident))
    }
}
