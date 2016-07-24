#![feature(box_syntax, box_patterns)]

use std::fmt::*;

#[derive(Clone)]
pub enum Expr {
    EVar(String),
    EAbs(String, Box<Expr>),
    EApp(Box<Expr>, Box<Expr>),
    ELet(String, Box<Expr>, Box<Expr>)
}

use self::Expr::*;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            EVar(ref name) => write!(f, "{}", name),
            EAbs(ref name, ref expr) => write!(f, "λ {} → {}", name, expr),
            EApp(ref e1, ref e2) => write!(f, "{}({})", e1, e2),
            ELet(ref name, ref value, ref body) => write!(f, "let {} = {} in {}", name, value, body)
        }
    }
}

fn main() {
    println!("{}", EVar(String::from("x")));
    println!("{}", EAbs(String::from("x"), box EVar(String::from("x"))));
    println!("{}", EApp(box EVar(String::from("x")), box EVar(String::from("y"))));
}
