use derive_more::Constructor;

use crate::ast::{Call, Expr, Id};

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Function(FunctionStatement),
    Declare(DeclareStatement),
    Call(Call),
}

#[derive(Debug, Constructor)]
pub struct AssignmentStatement {
    pub id: Id,
    pub expr: Expr,
}

#[derive(Debug, Constructor, Clone)]
pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<Id>,
    pub body: Expr,
}

#[derive(Debug, Constructor)]
pub struct DeclareStatement {
    pub id: Id,
    pub expr: Option<Expr>,
}
