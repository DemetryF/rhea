use lexer::Lexer;

use crate::token::TokenValue;

mod error;
mod lexer;
mod token;

fn main() {
    let code = "(){}+-*/2.2 0x_F id fn let";
    let mut lexer = Lexer::new(code);

    loop {
        match lexer.next() {
            Ok(token) => {
                println!("{}", token);
                if token.value == TokenValue::EOF {
                    break;
                }
            }
            Err(error) => println!("{}", error),
        }
    }
}
