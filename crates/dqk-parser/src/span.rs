use std::{
    ops::{BitOr, BitOrAssign, Deref, DerefMut},
    path::Path,
};

use serde::Serialize;

pub use dqk_macro::Spanned;

use crate::{Parse, Parser, Result, TokenKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Span {
    file_path: Option<&'static Path>,
    start: usize,
    length: usize,
}

impl Span {
    pub const fn new(file_path: &'static Path, start: usize, length: usize) -> Self {
        Self {
            file_path: Some(file_path),
            start,
            length,
        }
    }

    pub const fn file_path(&self) -> Option<&'static Path> {
        self.file_path
    }

    pub const fn end(&self) -> usize {
        self.start + self.length
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl BitOr for Span {
    type Output = Span;

    fn bitor(self, rhs: Self) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.file_path(), rhs.file_path()) {
            assert_eq!(lhs as *const _, rhs as *const _);
        }

        let start = self.start.min(rhs.start);

        Self {
            file_path: self.file_path,
            start,
            length: self.end().max(rhs.end()) - start,
        }
    }
}

impl BitOrAssign for Span {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpannedOption<T> {
    value: Option<T>,
    span: Span,
}

impl<T> SpannedOption<T> {
    pub fn parse_if_next(parser: &mut Parser, kind: TokenKind) -> Result<Self>
    where
        T: Parse + Spanned,
    {
        let tok = parser.peek_token()?;

        if tok.kind() == kind {
            let value = parser.parse::<T>()?;

            Ok(Self {
                span: value.span(),
                value: Some(value),
            })
        } else {
            Ok(Self {
                value: None,
                span: parser.span(),
            })
        }
    }
}

impl<T> Spanned for SpannedOption<T> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Deref for SpannedOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SpannedOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {}
