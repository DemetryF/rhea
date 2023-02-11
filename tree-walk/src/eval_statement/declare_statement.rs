use std::rc::Rc;

use frontend::{ast::DeclareStatement, error::*};

use crate::{Eval, EvalStatement, Interpreter, SharedEnv};

impl EvalStatement for DeclareStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        let id = self.id.value.clone();

        let value = match self.expr {
            Some(expr) => expr.eval(interpreter, Rc::clone(&env))?,
            None => 0.0,
        };

        env.borrow_mut().variables.insert(id, value);

        Ok(())
    }
}
