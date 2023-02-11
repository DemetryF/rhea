use std::rc::Rc;

use frontend::{ast::AssignmentStatement, error::*};

use crate::{Eval, EvalStatement, Interpreter, SharedEnv};

impl EvalStatement for AssignmentStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        let id = self.id;
        let value = self.expr.eval(interpreter, Rc::clone(&env))?;

        env.borrow_mut().set_variable(id, value)?;

        Ok(())
    }
}
