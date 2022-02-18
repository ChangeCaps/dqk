use dqk_macro::Spanned;

use crate::{Gt, Lt, Parse, Parser, Result};

#[derive(Clone, Debug, Spanned)]
pub struct Tupled<T> {
    pub open: Lt,
    pub contents: T,
    pub close: Gt,
}

impl<T> Tupled<T> {
    pub fn parse_with(
        parser: &mut Parser,
        f: impl FnOnce(&mut Parser) -> Result<T>,
    ) -> Result<Self> {
        Ok(Self {
            open: parser.parse()?,
            contents: f(parser)?,
            close: parser.parse()?,
        })
    }
}

impl<T: Parse> Parse for Tupled<T> {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            open: parser.parse()?,
            contents: parser.parse()?,
            close: parser.parse()?,
        })
    }
}
