use crate::token::Pos;
use derive_more::Constructor;

pub use expr::*;
pub use statement::*;

pub mod expr;
pub mod statement;

#[derive(Constructor, Debug)]
pub struct Id {
    pub value: String,
    pub pos: Pos,
}
