use derive_more::Constructor;
use std::fmt::Display;

use crate::token::{Pos, TokenValue};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Constructor, Debug)]
pub struct Error {
    kind: ErrorKind,
    pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenValue),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "unexpected char '{}'", char),
            Self::UnexpectedToken(value) => write!(f, "unexpected token '{}'", value),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} at {}", self.kind, self.pos)
    }
}
