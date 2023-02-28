use crate::expression::Expr;

pub enum Stmt {
    Var(VarDecStmt),
    Print(PrintStmt),
    Expr(ExprStmt),
    Block(Block),
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

pub struct VarDecStmt {
    pub var_name: String,
    pub initializer: Option<Expr>,
}

impl VarDecStmt {
    pub fn new(var_name: String, initializer: Option<Expr>) -> Self {
        Self {
            var_name,
            initializer,
        }
    }
}

pub struct PrintStmt {
    pub expr: Expr,
}

impl PrintStmt {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
pub struct ExprStmt {
    pub expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
