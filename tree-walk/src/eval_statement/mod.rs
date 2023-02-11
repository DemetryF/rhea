use crate::{Eval, Interpreter, SharedEnv};

use frontend::{ast::Statement, error::*};

mod assignment_statement;
mod declare_statement;
mod function_statement;

pub trait EvalStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()>;
}

impl EvalStatement for Statement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        match self {
            Self::Assignment(assignment) => assignment.eval(interpreter, env),
            Self::Function(function) => function.eval(interpreter, env),
            Self::Declare(declare) => declare.eval(interpreter, env),
            Self::Call(call) => call.eval(interpreter, env).map(|_| ()),
        }
    }
}
