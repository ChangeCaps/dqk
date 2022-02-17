use dqk_ast::FromToken;

use dqk_error::{Error, Result};

use crate::Parser;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Box::new(parser.parse()?))
    }
}

macro_rules! impl_parse {
    ($($ty:ident),* $(,)?) => {
        $(
            impl Parse for dqk_ast::$ty {
                fn parse(parser: &mut Parser) -> Result<Self> {
                    let token = parser.next_token()?;

                    if let Some(this) = dqk_ast::$ty::from_token(token) {
                        Ok(this)
                    } else {
                        let found = format!("found '{:?}'", token);
                        Err(Error::new(format!("expected '{}'", dqk_ast::$ty::NAME))
                            .with_hint(found, parser.span()))
                    }
                }
            }
        )*
    };
}

impl_parse! {
    Ident,
    LitStr,
    Comment,
    LitInt,
    LitFloat,

    // symbols
    ColonEqual,
    Colon,
    SemiColon,
    Comma,
    Period,
    Plus,
    Minus,
    Asterisk,
    Slash,
    EqualEqual,
    Equal,
    GtEqual,
    Gt,
    LtEqual,
    Lt,

    // eol
    Eol,
}
