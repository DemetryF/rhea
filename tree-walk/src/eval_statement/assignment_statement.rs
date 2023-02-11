use std::rc::Rc;

use frontend::{ast::AssignmentStatement, error::*};

use crate::{Eval, EvalStatement, Interpreter, SharedEnv};

impl EvalStatement for AssignmentStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        Rc::clone(&env)
            .borrow_mut()
            .set_variable(self.id, self.expr.eval(interpreter, env)?)?;

        Ok(())
    }
}
