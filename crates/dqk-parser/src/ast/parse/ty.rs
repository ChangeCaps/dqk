use crate::{Error, Parse, Parser, Result, TokenKind, Type};

impl Parse for Type {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let tok = parser.peek_token()?;

        match tok.kind() {
            TokenKind::Ident(_) => Ok(Type::Ident(parser.parse()?)),
            kind => {
                Err(Error::new("expected {type}")
                    .with_hint(format!("found '{:?}'", kind), tok.span()))
            }
        }
    }
}
