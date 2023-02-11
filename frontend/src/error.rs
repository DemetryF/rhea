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
    InvalidVariable(String),
    InvalidFunction(String),
    InvalidArgsCount { expected: usize, received: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} at {}", self.kind, self.pos)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "unexpected char '{char}'"),
            Self::UnexpectedToken(value) => write!(f, "unexpected token '{value}'"),
            Self::InvalidVariable(var) => write!(f, "invalid variable '{var}'"),
            Self::InvalidFunction(func) => write!(f, "invalid function '{func}'"),
            Self::InvalidArgsCount { expected, received } => {
                write!(f, "expected {expected} args, received {received}")
            }
        }
    }
}
