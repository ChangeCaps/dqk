use crate::{Ident, Spanned};

#[derive(Clone, Debug, Spanned)]
pub enum Type {
    Ident(Ident),
}
