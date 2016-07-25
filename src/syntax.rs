use std::fmt::*;
pub use self::Expr::*;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum Expr {
    EVar(&'static str),
    EAbs(&'static str, Box<Expr>),
    EApp(Box<Expr>, Box<Expr>),
    ELet(&'static str, Box<Expr>, Box<Expr>)
}

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

