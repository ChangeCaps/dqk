use std::{
    ops::{BitOr, BitOrAssign},
    path::Path,
};

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Span {
    file_path: &'static Path,
    start: usize,
    length: usize,
}

impl Span {
    pub fn new(file_path: &'static Path, start: usize, length: usize) -> Self {
        Self {
            file_path,
            start,
            length,
        }
    }

    pub fn file_path(&self) -> &'static Path {
        self.file_path
    }

    pub fn end(&self) -> usize {
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
        assert_eq!(self.file_path() as *const _, rhs.file_path() as *const _);

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

#[cfg(test)]
mod tests {}
