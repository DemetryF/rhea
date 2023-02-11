use std::rc::Rc;

use frontend::{ast::Call, error::*};

use crate::{Env, Eval, Interpreter, SharedEnv};

impl Eval for Call {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        if let Some(f) = interpreter.functions.get(&self.id.value).cloned() {
            {
                let received = self.args.len();
                let expected = f.args.len();

                if received != expected {
                    return Err(Error::new(
                        ErrorKind::InvalidArgsCount { expected, received },
                        self.id.pos,
                    ));
                }
            }

            let f_env = Env::new_with_parent(env);

            for (i, arg) in self.args.into_iter().enumerate() {
                let value = arg.eval(interpreter, Rc::clone(&f_env))?;
                let id = f.args.get(i).unwrap().to_owned();

                Rc::clone(&f_env)
                    .borrow_mut()
                    .add_variable(id.to_owned(), value);
            }

            return f.body.eval(interpreter, f_env);
        }

        if let Some(f) = interpreter.builtin_functions.get(&self.id.value) {
            {
                let received = self.args.len();
                let expected = f.args_count;

                if received != expected {
                    return Err(Error::new(
                        ErrorKind::InvalidArgsCount { expected, received },
                        self.id.pos,
                    ));
                }
            }

            return Ok((f.call)(
                self.args
                    .into_iter()
                    .map(|x| x.eval(interpreter, Rc::clone(&env)))
                    .collect::<std::result::Result<Vec<f64>, Error>>()?,
            ));
        }

        Err(Error::new(
            ErrorKind::InvalidFunction(self.id.value),
            self.id.pos,
        ))
    }
}
