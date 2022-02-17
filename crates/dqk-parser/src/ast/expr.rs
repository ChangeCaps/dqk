use dqk_ast::{Expr, LitFloat, LitInt, LitStr, LiteralExpr, TokenKind};
use dqk_error::{Error, Result};

use crate::{Parse, Parser};

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

impl Parse for Expr {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::String(_) => {
                Ok(Self::Literal(parser.parse()?))
            }
            kind => Err(Error::new("expected expression")
                .with_hint(format!("found '{:?}'", kind), tok.span())),
        }
    }
}
