use crate::{env::*, eval::Eval, eval_statement::EvalStatement};
use frontend::{ast::*, error::*};
use std::{collections::HashMap, rc::Rc};

mod env;
mod eval;
mod eval_statement;

pub struct Function {
    pub args_count: usize,
    pub call: Box<dyn Fn(Vec<f64>) -> f64>,
}

pub struct Interpreter {
    functions: HashMap<String, FunctionStatement>,
    root_env: SharedEnv,
    pub builtin_functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn eval(&mut self, stmts: Statement) -> Result<()> {
        stmts.eval(self, Rc::clone(&self.root_env))
    }

    pub fn add_function<F>(&mut self, name: &str, args_count: usize, f: F)
    where
        F: Fn(Vec<f64>) -> f64 + 'static,
    {
        self.builtin_functions.insert(
            name.into(),
            Function {
                args_count,
                call: Box::new(f),
            },
        );
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            builtin_functions: HashMap::new(),
            root_env: Env::new_shared(),
            functions: HashMap::new(),
        }
    }
}
