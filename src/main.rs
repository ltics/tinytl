#![feature(box_syntax, box_patterns)]

use std::fmt::*;

#[derive(Clone)]
pub enum Expr {
    EVar(&'static str),
    EAbs(&'static str, Box<Expr>),
    EApp(Box<Expr>, Box<Expr>),
    ELet(&'static str, Box<Expr>, Box<Expr>)
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

#[test]
fn display_spec() {
    assert_eq!(format!("{}", EVar("x")), "x");
    assert_eq!(format!("{}", EAbs("x", box EVar("x"))), "λ x → x");
    assert_eq!(format!("{}", EApp(box EVar("x"), box EVar("y"))), "x(y)");
    assert_eq!(format!("{}", ELet("x", box EVar("x"), box EApp(box EVar("x"), box EVar("y")))), "let x = x in x(y)");
}

fn main() {
}
