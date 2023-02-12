use crate::{env::*, eval::Eval, eval_statement::EvalStatement};
use frontend::{ast::*, error::Error, parser::Parser};
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
    pub fn eval(&mut self, code: &str) -> Result<(), Vec<Error>> {
        let mut parser = match Parser::new(code) {
            Ok(parser) => parser,
            Err(error) => return Err(vec![error]),
        };

        let stmts = match parser.parse() {
            Ok(stmts) => stmts,
            Err(errors) => return Err(errors),
        };

        let mut errors = Vec::new();

        for stmt in stmts {
            match stmt.eval(self, Rc::clone(&self.root_env)) {
                Ok(()) => (),
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
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
