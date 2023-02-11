use frontend::{ast::Id, error::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type SharedEnv = Rc<RefCell<Env>>;

pub struct Env {
    pub parent: Option<SharedEnv>,
    pub variables: HashMap<String, f64>,
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
            Some(dst) => {
                *dst = src;
                Ok(())
            }
            None => match &self.parent {
                Some(parent) => parent.borrow_mut().set_variable(id, src),
                None => Err(Error::new(ErrorKind::InvalidVariable(id.value), id.pos)),
            },
        }
    }

    pub fn add_variable(&mut self, id: Id, value: f64) {
        self.variables.insert(id.value, value);
    }
}
