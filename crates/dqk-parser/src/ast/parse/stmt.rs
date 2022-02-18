use crate::{
    AssignNewStmt, AssignStmt, Block, CloseParen, DefaultEvent, DefaultEvents, Error, Expr,
    ExprStmt, Ident, Keyword, ListenerArgument, ListenerArguments, LnStmt, Parse, Parser,
    Punctuated, Result, SpannedOption, Stmt, Symbol, TokenKind, Tupled, WhereClause,
};

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

impl Parse for DefaultEvent {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            ident: parser.parse()?,
        })
    }
}

impl Parse for DefaultEvents {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            events: Tupled::parse_with(parser, |parser| {
                Punctuated::parse_terminated(parser, TokenKind::Symbol(Symbol::Gt))
            })?,
        })
    }
}

impl Parse for WhereClause {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let _where = parser.parse()?;
        parser.skip_eol()?;

        let bounds = Punctuated::parse_terminated_with(
            parser,
            Expr::parse,
            |parser| {
                let punct = parser.parse()?;
                parser.skip_eol()?;
                Ok(punct)
            },
            TokenKind::Symbol(Symbol::OpenBrace),
        )?;
        parser.skip_eol()?;

        Ok(Self { _where, bounds })
    }
}

impl Parse for ListenerArgument {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Ident(_) => Ok(Self::Expanded(parser.parse()?)),
            kind => Err(Error::new("expected 'listener argument'")
                .with_hint(format!("found '{:?}'", kind), tok.span())),
        }
    }
}

impl Parse for ListenerArguments {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            open: parser.parse()?,
            args: Punctuated::parse_terminated(parser, TokenKind::Symbol(Symbol::CloseParen))?,
            close: parser.parse()?,
        })
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let open = parser.parse()?;

        let mut stmts = Vec::new();

        loop {
            parser.skip_eol()?;

            let tok = parser.peek_token()?;

            if tok.kind() == TokenKind::Symbol(Symbol::CloseBrace) {
                break;
            }

            stmts.push(parser.parse()?);
        }

        Ok(Self {
            open,
            stmts,
            close: parser.parse()?,
        })
    }
}

impl Parse for LnStmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let ln = parser.parse()?;
        let ident = parser.parse()?;
        let default_events = SpannedOption::parse_if_next(parser, TokenKind::Symbol(Symbol::Lt))?;
        let args = parser.parse()?;

        parser.skip_eol()?;

        let where_clause =
            SpannedOption::parse_if_next(parser, TokenKind::Keyword(Keyword::Where))?;

        parser.skip_eol()?;

        let block = parser.parse()?;

        Ok(Self {
            ln,
            ident,
            default_events,
            args,
            where_clause,
            block,
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
        _ => Ok(Stmt::Expr(ExprStmt {
            expr: Expr::parse_with_ident(parser, ident)?,
        })),
    }
}

impl Parse for Stmt {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Keyword(Keyword::Ln) => Ok(Self::Ln(parser.parse()?)),
            TokenKind::Ident(_) => Ok(parse_ident_stmt(parser)?),
            kind if kind.starts_expr() => Ok(Self::Expr(parser.parse()?)),
            kind => Err(Error::new("expected statement")
                .with_hint(format!("found '{:?}'", kind), tok.span())),
        }
    }
}
