use std::fmt::*;
pub use self::Type::*;
pub use self::Scheme::*;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    TInt,
    TBool,
    TVar(&'static str),
    TArrow(Box<Type>, Box<Type>)
}

fn is_arrow(t: &Type) -> bool {
    match *t {
        TArrow(_, _) => true,
        _ => false
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            TInt => write!(f, "int"),
            TBool => write!(f, "bool"),
            TVar(ref name) => write!(f, "{}", name),
            TArrow(ref t1, ref t2) if is_arrow(t1) => write!(f, "({}) → {}", t1, t2),
            TArrow(ref t1, ref t2) => write!(f, "{} → {}", t1, t2)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum Scheme {
    Mono(Box<Type>),
    Poly(&'static str, Box<Scheme>)
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Mono(ref t) => write!(f, "{}", t),
            Poly(ref a, ref t) => write!(f, "∀{}. {}", a, t)
        }
    }
}

