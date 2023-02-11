use crate::{
    ast::Id,
    error::*,
    parser::TokenStream,
    token::{Operator, Token, TokenValue},
};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> Result<Id> {
        let Token { value, pos } = token_stream.skip()?;

        match value {
            TokenValue::Id(value) => Ok(Id::new(value, pos)),
            _ => Err(Error::new(ErrorKind::UnexpectedToken(value), pos)),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Result<Operator> {
        let Token { value, pos } = token_stream.skip()?;

        match value {
            TokenValue::Operator(op) => Ok(op),
            _ => Err(Error::new(ErrorKind::UnexpectedToken(value), pos)),
        }
    }
}
