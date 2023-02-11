use frontend::{ast::Atom, error::*};

use crate::{Eval, Interpreter, SharedEnv};

impl Eval for Atom {
    fn eval(self, _interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        match self {
            Self::Literal(literal) => Ok(literal),
            Self::Id(id) => env.borrow().get_variable(id),
        }
    }
}
