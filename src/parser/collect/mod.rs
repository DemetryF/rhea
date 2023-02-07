use super::TokenStream;
use crate::error::*;

mod expr;
mod statement;

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self>;
}
