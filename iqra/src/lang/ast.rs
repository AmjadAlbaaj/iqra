use crate::lang::token::TokenKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Var(String),
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Unary { op: TokenKind, expr: Box<Expr> },
    List(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
    Assign { name: String, value: Expr },
    Print(Expr),
    Block(Vec<Stmt>),
    If { cond: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Expr, body: Box<Stmt> },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Option<Expr>),
}
