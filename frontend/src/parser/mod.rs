mod collect;
mod parser_utils;
mod power_bindings;
mod token_stream;

use self::{
    collect::Collect, parser_utils::ParserUtils, power_bindings::PowerBinding,
    token_stream::TokenStream,
};
use crate::{ast::Statement, error::Error};

pub struct Parser<'code> {
    pub token_stream: TokenStream<'code>,
}

impl<'code> Parser<'code> {
    pub fn new(code: &'code str) -> Result<Self, Error> {
        Ok(Self {
            token_stream: TokenStream::new(code)?,
        })
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();

        while !self.token_stream.is_end() {
            match Statement::collect(&mut self.token_stream) {
                Ok(stmt) => stmts.push(stmt),
                Err(error) => {
                    errors.push(error);
                    self.token_stream.recovery();
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(stmts)
        }
    }
}
