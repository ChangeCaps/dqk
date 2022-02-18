use serde::Serialize;

use crate::{Error, Parse, Parser, Result, Span, Spanned};

pub trait FromToken: Sized {
    const NAME: &'static str;

    fn from_token(token: Token) -> Option<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub const fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum TokenKind {
    Ident(&'static str),
    String(&'static str),
    Comment(&'static str),
    Keyword(Keyword),
    Integer(Integer),
    Float(Float),
    Symbol(Symbol),
    Eol,
    Eof,
}

impl TokenKind {
    pub fn is_eol(&self) -> bool {
        match self {
            Self::Eol | Self::Eof => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Integer {
    value: i64,
    kind: IntegerKind,
}

impl Integer {
    pub const fn new(value: i64, kind: IntegerKind) -> Self {
        Self { value, kind }
    }

    pub const fn value(&self) -> i64 {
        self.value
    }

    pub const fn kind(&self) -> IntegerKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Hex,
}

impl IntegerKind {
    pub const fn radix(&self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Decimal => 10,
            Self::Hex => 16,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Float {
    value: f64,
}

impl Float {
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> f64 {
        self.value
    }
}

macro_rules! sub_token {
    (
        pub enum $ident:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
        pub enum $ident {
            $($variant),*
        }

        $(
            #[derive(Clone, Copy, PartialEq, Eq, Serialize)]
            pub struct $variant {
                span: Span,
            }

            impl ::std::fmt::Debug for $variant {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(stringify!($variant))
                }
            }

            impl Spanned for $variant {
                fn span(&self) -> Span {
                    self.span
                }
            }

            impl Parse for $variant {
                fn parse(parser: &mut Parser) -> Result<Self> {
                    Ok(Self {
                        span: parser.expect(TokenKind::$ident($ident::$variant))?,
                    })
                }
            }
        )*
    };
}

sub_token! {
    pub enum Keyword {
        Event,
        Query,
        Where,
        Ln,
        Fn,
        If,
        For,
        Else,
        Return,
    }
}

sub_token! {
    pub enum Symbol {
        OpenParen,
        OpenBrace,
        OpenBracket,
        CloseParen,
        CloseBrace,
        CloseBracket,
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
}

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

			impl Parse for $ident {
                fn parse(parser: &mut Parser) -> Result<Self> {
                    let tok = parser.next_token()?;

					match tok.kind() {
						$pat => Ok(Self::new($value, tok.span())),
						_ => Err(
                            Error::new(format!("expected '{}'", $name))
                                .with_hint(format!("found '{:?}'", tok.kind()), tok.span())
                        ),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Eol {
    span: Span,
}

impl Spanned for Eol {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Eol {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            span: parser.expect_eol()?,
        })
    }
}
