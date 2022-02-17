use dqk_ast::{AssignNewStmt, AssignStmt, ExprStmt, Ident, Stmt, Symbol, TokenKind};
use dqk_error::{Error, Result};

use crate::{Parse, Parser};

impl Parse for AssignNewStmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            ident: parser.parse()?,
            equal: parser.parse()?,
            expr: parser.parse()?,
            eol: parser.parse()?,
        })
    }
}

impl Parse for AssignStmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            ident: parser.parse()?,
            equal: parser.parse()?,
            expr: parser.parse()?,
            eol: parser.parse()?,
        })
    }
}

impl Parse for ExprStmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            expr: parser.parse()?,
        })
    }
}

fn parse_ident_stmt(parser: &mut Parser) -> Result<Stmt> {
    let ident = parser.parse::<Ident>()?;

    let tok = parser.peek_token()?;

    match tok.kind() {
        TokenKind::Symbol(Symbol::ColonEqual) => Ok(Stmt::AssignNew(AssignNewStmt {
            ident,
            equal: parser.parse()?,
            expr: parser.parse()?,
            eol: parser.parse()?,
        })),
        TokenKind::Symbol(Symbol::Equal) => Ok(Stmt::Assign(AssignStmt {
            ident,
            equal: parser.parse()?,
            expr: parser.parse()?,
            eol: parser.parse()?,
        })),
        kind => {
            Err(Error::new("expected ['=', ':=']")
                .with_hint(format!("found {:?}", kind), tok.span()))
        }
    }
}

impl Parse for Stmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Ident(_) => Ok(parse_ident_stmt(parser)?),
            kind => Err(Error::new("expected statement")
                .with_hint(format!("found '{:?}'", kind), tok.span())),
        }
    }
}
