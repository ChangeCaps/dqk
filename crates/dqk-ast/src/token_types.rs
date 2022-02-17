use serde::Serialize;

use crate::{Float, FromToken, Integer, Span, Spanned, Symbol, Token, TokenKind};

macro_rules! value_tokens {
    ($($pat:pat => $ident:ident($value:ident: $ty:ty, $name:literal)),* $(,)?) => {
		$(
			#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
			pub struct $ident {
				value: $ty,
				span: Span,
			}

			impl $ident {
				pub const fn new(value: $ty, span: Span) -> Self {
					Self { value, span }
				}
			}

			impl Spanned for $ident {
				fn span(&self) -> Span {
					self.span
				}
			}

			impl FromToken for $ident {
				const NAME: &'static str = $name;

				fn from_token(token: Token) -> Option<Self> {
					match token.kind() {
						$pat => Some(Self::new($value, token.span())),
						_ => None,
					}
				}
			}
		)*
    };
}

value_tokens! {
    TokenKind::Ident(value) => Ident(value: &'static str, "{identifier}"),
    TokenKind::String(value) => LitStr(value: &'static str, "{string}"),
    TokenKind::Comment(value) => Comment(value: &'static str, "{comment}"),
    TokenKind::Integer(value) => LitInt(value: Integer, "{integer}"),
    TokenKind::Float(value) => LitFloat(value: Float, "{float}"),
}

macro_rules! symbol_tokens {
    ($($ident:ident),* $(,)?) => {
		$(
			#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
			pub struct $ident {
				span: Span,
			}

			impl Spanned for $ident {
				fn span(&self) -> Span {
					self.span
				}
			}

			impl FromToken for $ident {
				const NAME: &'static str = stringify!($ident);

				fn from_token(token: Token) -> Option<Self> {
					match token.kind() {
						TokenKind::Symbol(Symbol::$ident) => Some(Self {
							span: token.span(),
						}),
						_ => None,
					}
				}
			}
		)*
	};
}

symbol_tokens! {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Eol {
    span: Span,
}

impl Spanned for Eol {
    fn span(&self) -> Span {
        self.span
    }
}

impl FromToken for Eol {
    const NAME: &'static str = "end of line";

    fn from_token(token: Token) -> Option<Self> {
        match token.kind() {
            TokenKind::Eol => Some(Self { span: token.span() }),
            TokenKind::Eof => Some(Self { span: token.span() }),
            _ => None,
        }
    }
}
