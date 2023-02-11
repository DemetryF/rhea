use crate::{Eval, Interpreter, SharedEnv};

use frontend::{ast::Prefix, error::*, token::Operator};

impl Eval for Prefix {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        let rhs = self.rhs.eval(interpreter, env)?;

        Ok(match self.op {
            Operator::Addition => rhs,
            Operator::Subtraction => -rhs,
            _ => unreachable!(),
        })
    }
}
