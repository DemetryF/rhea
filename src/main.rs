use token::*;

mod token;

fn main() {
    println!("{}", Token::new(TokenValue::Number(9.2), Pos::new(1, 2, 0)));
    println!("{}", Token::new(TokenValue::Fn, Pos::new(1, 2, 0)));
    println!("{}", Token::new(TokenValue::Comma, Pos::new(1, 2, 0)));
    println!("{}", Token::new(TokenValue::Semicolon, Pos::new(1, 2, 0)));
    println!(
        "{}",
        Token::new(TokenValue::Operator(Operator::Addition), Pos::new(1, 2, 0))
    );
}
