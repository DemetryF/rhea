use frontend::{ast::*, error::*, token::Operator};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type SharedEnv = Rc<RefCell<Env>>;

pub struct Env {
    pub parent: Option<SharedEnv>,

    variables: HashMap<String, f64>,
}

impl Env {
    pub fn new() -> Env {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn new_shared() -> SharedEnv {
        Self::make_shared(Env::new())
    }

    pub fn make_shared(env: Env) -> SharedEnv {
        Rc::new(RefCell::new(env))
    }

    pub fn new_with_parent(env: SharedEnv) -> SharedEnv {
        Self::make_shared(Self {
            parent: Some(env),
            ..Self::new()
        })
    }

    pub fn get_variable(&self, id: Id) -> Result<f64> {
        match self.variables.get(&id.value) {
            Some(value) => Ok(*value),
            None => match &self.parent {
                Some(parent) => parent.borrow().get_variable(id),
                None => Err(Error::new(ErrorKind::InvalidVariable(id.value), id.pos)),
            },
        }
    }

    pub fn set_variable(&mut self, id: Id, src: f64) -> Result<()> {
        match self.variables.get_mut(&id.value) {
            Some(dst) => Ok(*dst = src),
            None => match &self.parent {
                Some(parent) => parent.borrow_mut().set_variable(id, src),
                None => Err(Error::new(ErrorKind::InvalidVariable(id.value), id.pos)),
            },
        }
    }

    pub fn add_variable(&mut self, id: Id, value: f64) {
        self.variables.insert(id.value.clone(), value);
    }
}

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
    pub fn new() -> Self {
        Self {
            builtin_functions: HashMap::new(),
            root_env: Env::new_shared(),
            functions: HashMap::new(),
        }
    }

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

pub trait EvalStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()>;
}

pub trait Eval {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64>;
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

impl EvalStatement for DeclareStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        let value = match self.expr {
            Some(expr) => expr.eval(interpreter, Rc::clone(&env))?,
            None => 0.0,
        };

        env.borrow_mut()
            .variables
            .insert(self.id.value.clone(), value);

        Ok(())
    }
}

impl EvalStatement for FunctionStatement {
    fn eval(self, interpreter: &mut Interpreter, _env: SharedEnv) -> Result<()> {
        interpreter.functions.insert(self.id.value.clone(), self);
        Ok(())
    }
}

impl EvalStatement for AssignmentStatement {
    fn eval(self, interpreter: &mut Interpreter, env: SharedEnv) -> Result<()> {
        Rc::clone(&env)
            .borrow_mut()
            .set_variable(self.id, self.expr.eval(interpreter, env)?)?;

        Ok(())
    }
}

impl Eval for Expr {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        match self {
            Self::Infix(infix) => infix.eval(interpreter, env),
            Self::Prefix(prefix) => prefix.eval(interpreter, env),
            Self::Call(call) => call.eval(interpreter, env),
            Self::Atom(atom) => atom.eval(interpreter, env),
        }
    }
}

impl Eval for Infix {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        let lhs = self.lhs.eval(interpreter, Rc::clone(&env))?;
        let rhs = self.rhs.eval(interpreter, Rc::clone(&env))?;

        Ok(match self.op {
            Operator::Addition => lhs + rhs,
            Operator::Subtraction => lhs - rhs,
            Operator::Multiplication => lhs * rhs,
            Operator::Division => lhs / rhs,
        })
    }
}

impl Eval for Prefix {
    fn eval(self, interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        let rhs = self.rhs.eval(interpreter, env)?;

        Ok(match self.op {
            Operator::Addition => rhs,
            Operator::Subtraction => -rhs,
            _ => unreachable!(),
        })
    }
}

impl Eval for Atom {
    fn eval(self, _interpreter: &Interpreter, env: SharedEnv) -> Result<f64> {
        match self {
            Self::Literal(literal) => Ok(literal),
            Self::Id(id) => env.borrow().get_variable(id),
        }
    }
}

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

            let mut i = 0;
            for arg in self.args {
                let value = arg.eval(interpreter, Rc::clone(&f_env))?;
                let id = f.args.get(i).unwrap().to_owned();

                Rc::clone(&f_env)
                    .borrow_mut()
                    .add_variable(id.to_owned(), value);

                i += 1;
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
