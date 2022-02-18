use crate::{Parse, Parser, Result, Span, Spanned, TokenKind};

pub trait Termination {
    fn terminate(&self, kind: &TokenKind) -> bool;
}

impl Termination for TokenKind {
    fn terminate(&self, kind: &TokenKind) -> bool {
        self == kind
    }
}

impl<F: Fn(&TokenKind) -> bool> Termination for F {
    fn terminate(&self, kind: &TokenKind) -> bool {
        self(kind)
    }
}

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
    pub fn parse_terminated(parser: &mut Parser, termination: impl Termination) -> Result<Self>
    where
        I: Parse,
        P: Parse,
    {
        Self::parse_terminated_with(parser, I::parse, P::parse, termination)
    }

    // NOTE(changecaps): the function signature is fucked but this is the best way i could think of doing it.
    pub fn parse_terminated_with(
        parser: &mut Parser,
        mut parse_item: impl FnMut(&mut Parser) -> Result<I>,
        mut parse_punct: impl FnMut(&mut Parser) -> Result<P>,
        termination: impl Termination,
    ) -> Result<Self> {
        let span = parser.span();

        let mut items = Vec::new();
        let mut punct = Vec::new();

        loop {
            let tok = parser.peek_token()?;

            if termination.terminate(&tok.kind()) {
                break Ok(Self {
                    items,
                    punct,
                    span: span | parser.span(),
                });
            }

            items.push(parse_item(parser)?);

            let tok = parser.peek_token()?;

            if termination.terminate(&tok.kind()) {
                break Ok(Self {
                    items,
                    punct,
                    span: span | parser.span(),
                });
            }

            punct.push(parse_punct(parser)?);
        }
    }
}
