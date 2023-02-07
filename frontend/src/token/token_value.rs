use colored::Colorize;
use std::fmt::Display;

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Id(String),
    Number(f64),
    Operator(Operator),

    EOF,

    // special chars
    Comma,
    Semicolon,
    Assignment,
    OpeningParen,
    ClosingParen,

    // keywords
    Fn,
    Let,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id.white()),
            Self::Number(num) => write!(f, "{}", num.to_string().yellow()),
            Self::Operator(op) => write!(f, "{}", op.to_string().green()),
            Self::Comma => write!(f, "{}", ",".magenta()),
            Self::Semicolon => write!(f, "{}", ";".magenta()),
            Self::Assignment => write!(f, "{}", "=".magenta()),
            Self::OpeningParen => write!(f, "{}", "(".magenta()),
            Self::ClosingParen => write!(f, "{}", ")".magenta()),
            Self::Fn => write!(f, "{}", "fn".cyan()),
            Self::Let => write!(f, "{}", "let".cyan()),
            Self::EOF => write!(f, "{}", "EOF".black()),
        }
    }
}
