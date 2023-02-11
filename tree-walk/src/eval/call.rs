use std::rc::Rc;

use frontend::{ast::Call, error::*};

use crate::{Env, Eval, Function, Interpreter, SharedEnv};

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
            return call_builtin(self, f, interpreter, env);
        }

        Err(Error::new(
            ErrorKind::InvalidFunction(self.id.value),
            self.id.pos,
        ))
    }
}

fn call_builtin(
    call: Call,
    f: &Function,
    interpreter: &Interpreter,
    env: SharedEnv,
) -> Result<f64> {
    let received = call.args.len();
    let expected = f.args_count;

    if received != expected {
        let kind = ErrorKind::InvalidArgsCount { expected, received };
        let pos = call.id.pos;

        return Err(Error::new(kind, pos));
    }

    let args = call
        .args
        .into_iter()
        .map(|x| x.eval(interpreter, Rc::clone(&env)))
        .collect::<std::result::Result<Vec<f64>, Error>>()?;

    Ok((f.call)(args))
}
