use frontend::{ast::FunctionStatement, error::*};

use crate::{EvalStatement, Interpreter, SharedEnv};

impl EvalStatement for FunctionStatement {
    fn eval(self, interpreter: &mut Interpreter, _env: SharedEnv) -> Result<()> {
        interpreter.functions.insert(self.id.value.clone(), self);
        Ok(())
    }
}
