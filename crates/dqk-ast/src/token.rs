use serde::Serialize;

use crate::Span;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Symbol {
    Open(Delim),
    Close(Delim),
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
pub enum Delim {
    Paren,
    Brace,
    Bracket,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Keyword {
    Event,
}
