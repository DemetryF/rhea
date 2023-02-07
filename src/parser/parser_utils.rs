use crate::{
    error::*,
    parser::TokenStream,
    token::{Operator, TokenValue},
};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> Result<String> {
        let token = token_stream.skip()?;
        match token.value {
            TokenValue::Id(value) => Ok(value),
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken(token.value),
                token.pos,
            )),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Result<Operator> {
        let token = token_stream.skip()?;
        match token.value {
            TokenValue::Operator(op) => Ok(op),
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken(token.value),
                token.pos,
            )),
        }
    }
}