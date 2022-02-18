use crate::{Error, FromToken, Parser, Result, SpannedOption, Symbol, TokenKind, Tupled};

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Box::new(parser.parse()?))
    }
}
