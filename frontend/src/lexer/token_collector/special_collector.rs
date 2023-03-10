use crate::{
    lexer::{CodeStream, TokenCollector},
    token::TokenValue,
};

pub struct SpecialCollector;
impl TokenCollector for SpecialCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = match code_stream.current() {
            ',' => TokenValue::Comma,
            ';' => TokenValue::Semicolon,
            '=' => TokenValue::Assignment,
            '(' => TokenValue::OpeningParen,
            ')' => TokenValue::ClosingParen,

            _ => return None,
        };

        code_stream.accept();

        Some(value)
    }
}
