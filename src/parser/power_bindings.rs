use crate::token::Operator::{self, *};

pub struct PowerBinding;
impl PowerBinding {
    pub fn prefix(op: Operator) -> Option<u8> {
        match op {
            Subtraction | Addition => Some(5),
            _ => None,
        }
    }

    pub fn infix(op: Operator) -> Option<(u8, u8)> {
        match op {
            Subtraction | Addition => Some((1, 2)),
            Multiplication | Division => Some((3, 4)),
        }
    }
}
