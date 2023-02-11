use frontend::{ast::FunctionStatement, error::*};

use crate::{EvalStatement, Interpreter, SharedEnv};

impl EvalStatement for FunctionStatement {
    fn eval(self, interpreter: &mut Interpreter, _env: SharedEnv) -> Result<()> {
        let id = self.id.value.clone();

        interpreter.functions.insert(id, self);

        Ok(())
    }
}
