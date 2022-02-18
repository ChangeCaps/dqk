use crate::{Parse, Parser, Result, Span, Spanned, TokenKind};

#[derive(Clone, Debug)]
pub struct Punctuated<I, P> {
    items: Vec<I>,
    punct: Vec<P>,
    span: Span,
}

impl<I, P> Spanned for Punctuated<I, P> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<I, P> Punctuated<I, P> {
    pub fn parse_terminated(parser: &mut Parser, termination: TokenKind) -> Result<Self>
    where
        I: Parse,
        P: Parse,
    {
        let span = parser.span();

        let mut items = Vec::new();
        let mut punct = Vec::new();

        loop {
            let tok = parser.peek_token()?;

            if tok.kind() == termination {
                break Ok(Self {
                    items,
                    punct,
                    span: span | parser.span(),
                });
            }

            items.push(parser.parse()?);

            let tok = parser.peek_token()?;

            if tok.kind() == termination {
                break Ok(Self {
                    items,
                    punct,
                    span: span | parser.span(),
                });
            }

            punct.push(parser.parse()?);
        }
    }
}
