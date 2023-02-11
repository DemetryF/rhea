mod atom;
mod call;
mod infix;
mod prefix;

use crate::{Interpreter, SharedEnv};
use frontend::{ast::Expr, error::Result};

pub trait Eval {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64>;
}

impl Eval for Expr {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        match self {
            Self::Infix(infix) => infix.eval(interpreter, env),
            Self::Prefix(prefix) => prefix.eval(interpreter, env),
            Self::Call(call) => call.eval(interpreter, env),
            Self::Atom(atom) => atom.eval(interpreter, env),
        }
    }
}
