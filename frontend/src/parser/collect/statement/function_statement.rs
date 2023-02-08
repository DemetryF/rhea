use crate::{
    ast::{Expr, FunctionStatement, Id},
    error::*,
    parser::{Collect, ParserUtils, TokenStream},
    token::TokenValue,
};

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::Fn)?;
        let id = ParserUtils::id(token_stream)?;
        let args = Self::collect_args(token_stream)?;
        token_stream.accept(&TokenValue::Assignment)?;
        let body = Expr::collect(token_stream)?;

        Ok(FunctionStatement::new(id, args, body))
    }
}

impl FunctionStatement {
    fn collect_args(token_stream: &mut TokenStream) -> Result<Vec<Id>> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen)?;
        if token_stream
            .try_consume(&TokenValue::ClosingParen)?
            .is_some()
        {
            return Ok(args);
        }

        args.push(ParserUtils::id(token_stream)?);
        while token_stream.try_consume(&TokenValue::Comma)?.is_some() {
            args.push(ParserUtils::id(token_stream)?);
        }

        token_stream.accept(&TokenValue::ClosingParen)?;

        Ok(args)
    }
}
