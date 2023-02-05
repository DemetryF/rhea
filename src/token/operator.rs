use std::fmt::Display;
use strum_macros::EnumIter;

macro_rules! operators {
    [ $($Case:ident = $CaseValue:literal,)* ] => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
        pub enum Operator {
            $($Case,)*
        }

        impl From<Operator> for &str {
            fn from(op: Operator) -> Self {
                match op {
                    $( Operator::$Case => $CaseValue, )*
                }
            }
        }

        impl Display for Operator {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", <&str>::from(*self))
            }
        }
    };
}

#[rustfmt::skip]
operators![
    Addition = "+",
    Subtraction = "-",
    Multiplication = "*",
    Division = "/",
];
