use derive_more::Constructor;

use crate::ast::{Call, Expr};

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Function(FunctionStatement),
    Declare(DeclareStatement),
    Call(Call),
}

#[derive(Debug, Constructor)]
pub struct AssignmentStatement {
    pub id: String,
    pub expr: Expr,
}

#[derive(Debug, Constructor)]
pub struct FunctionStatement {
    pub id: String,
    pub args: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, Constructor)]
pub struct DeclareStatement {
    pub id: String,
    pub expr: Option<Expr>,
}
