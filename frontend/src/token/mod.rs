use derive_more::Constructor;
use std::fmt::Display;

pub use self::{operator::Operator, pos::Pos, token_value::TokenValue};

pub mod operator;
pub mod pos;
pub mod token_value;

#[derive(Debug, Constructor, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { value, pos } = self;
        write!(f, "[{pos}] {value}")
    }
}
