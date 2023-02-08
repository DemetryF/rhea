use derive_more::Constructor;

use crate::{ast::Id, token::Operator};

#[derive(Debug, Constructor)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Infix(Infix),
    Prefix(Prefix),
    Call(Call),
    Atom(Atom),
}

#[derive(Debug, Constructor)]
pub struct Infix {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Constructor)]
pub struct Prefix {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub enum Atom {
    Literal(f64),
    Id(Id),
}
