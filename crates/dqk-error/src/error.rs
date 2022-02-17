use std::panic::Location;

use dqk_ast::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    hints: Vec<ErrorHint>,
    msg: String,
    location: &'static Location<'static>,
}

impl Error {
    #[track_caller]
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            hints: Vec::new(),
            msg: msg.into(),
            location: Location::caller(),
        }
    }

    pub fn with_hint(mut self, msg: impl Into<String>, span: Span) -> Self {
        self.hints.push(ErrorHint::new(msg, span));
        self
    }

    pub fn hints(&self) -> &[ErrorHint] {
        &self.hints
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

#[derive(Clone, Debug)]
pub struct ErrorHint {
    span: Span,
    msg: String,
}

impl ErrorHint {
    pub fn new(msg: impl Into<String>, span: Span) -> Self {
        Self {
            span,
            msg: msg.into(),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}
