use std::rc::Rc;

use frontend::{ast::Infix, error::*, token::Operator};

use crate::{Eval, Interpreter, SharedEnv};

impl Eval for Infix {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        let lhs = self.lhs.eval(interpreter, Rc::clone(&env))?;
        let rhs = self.rhs.eval(interpreter, Rc::clone(&env))?;

        Ok(match self.op {
            Operator::Addition => lhs + rhs,
            Operator::Subtraction => lhs - rhs,
            Operator::Multiplication => lhs * rhs,
            Operator::Division => lhs / rhs,
        })
    }
}
