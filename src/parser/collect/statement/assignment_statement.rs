use crate::{
    ast::{AssignmentStatement, Expr},
    error::*,
    parser::{Collect, ParserUtils, TokenStream},
    token::TokenValue,
};

impl Collect for AssignmentStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let id = ParserUtils::id(token_stream)?;
        token_stream.accept(&TokenValue::Assignment)?;
        let expr = Expr::collect(token_stream)?;

        Ok(AssignmentStatement::new(id, expr))
    }
}
