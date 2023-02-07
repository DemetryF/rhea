use crate::{
    ast::{Expr, Prefix},
    error::*,
    parser::{Collect, ParserUtils, PowerBinding, TokenStream},
};

impl Collect for Prefix {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let op = ParserUtils::op(token_stream)?;
        let r_bp = match PowerBinding::prefix(op) {
            Some(r_bp) => r_bp,
            None => todo!(),
        };
        let rhs = Expr::expr_bp(token_stream, r_bp)?;

        Ok(Self::new(op, Box::new(rhs)))
    }
}
