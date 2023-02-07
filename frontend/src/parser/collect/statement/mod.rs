use crate::{
    ast::{AssignmentStatement, Call, DeclareStatement, FunctionStatement, Statement},
    error::*,
    parser::{Collect, TokenStream},
    token::TokenValue,
};

mod assignment_statement;
mod declare_statement;
mod function_statement;

impl Collect for Statement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let stmt = match token_stream.current().value {
            TokenValue::Fn => Self::Function(FunctionStatement::collect(token_stream)?),
            TokenValue::Let => Self::Declare(DeclareStatement::collect(token_stream)?),

            TokenValue::Id(_) => match token_stream.following()?.value {
                TokenValue::Assignment => {
                    Self::Assignment(AssignmentStatement::collect(token_stream)?)
                }
                TokenValue::OpeningParen => Self::Call(Call::collect(token_stream)?),

                _ => return Err(token_stream.unexpected_token()),
            },

            _ => return Err(token_stream.unexpected_token()),
        };

        token_stream.accept(&TokenValue::Semicolon)?;

        Ok(stmt)
    }
}
