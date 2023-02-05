use crate::token::Operator;

pub enum Statement {
    Assignment(AssignmentStatement),
    Function(FunctionStatement),
    Declare(DeclareStatement),
    Call(Call),
}

pub struct AssignmentStatement {
    pub id: String,
    pub expr: Expr,
}

pub struct FunctionStatement {
    pub id: String,
    pub args: Vec<String>,
}

pub struct DeclareStatement {
    pub id: String,
    pub expr: Option<Expr>,
}

pub struct Call {
    pub id: String,
    pub args: Vec<Expr>,
}

pub enum Expr {
    Infix(Infix),
    Prefix(Prefix),
    Call(Call),
    Atom(Atom),
}

pub struct Infix {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

pub struct Prefix {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

pub enum Atom {
    Literal(f64),
    Id(String),
}
