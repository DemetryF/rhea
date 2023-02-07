use crate::{
    ast::{DeclareStatement, Expr},
    error::*,
    parser::{Collect, ParserUtils, TokenStream},
    token::TokenValue,
};

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::Let)?;
        let id = ParserUtils::id(token_stream)?;
        let expr = Self::collect_init_expr(token_stream)?;

        Ok(DeclareStatement::new(id, expr))
    }
}

impl DeclareStatement {
    fn collect_init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>> {
        match token_stream.try_consume(&TokenValue::Assignment)? {
            None => Ok(None),
            Some(_) => Ok(Some(Expr::collect(token_stream)?)),
        }
    }
}
