use std::fmt::*;
use std::collections::HashMap;
use std::collections::HashSet;
pub use self::Type::*;
pub use self::Scheme::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    TInt,
    TBool,
    TVar(char),
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scheme {
    Mono(Box<Type>),
    Poly(char, Box<Scheme>)
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Mono(ref t) => write!(f, "{}", t),
            Poly(ref a, ref t) => write!(f, "∀{}. {}", a, t)
        }
    }
}

//pub type Subrule<'srl> = HashMap<char, &'srl Type>;

#[allow(dead_code)]
pub fn compose(s2: &HashMap<char, Type>, s1: &HashMap<char, Type>) -> HashMap<char, Type> {
    let mut s3: HashMap<char, Type> = HashMap::new();
    //apply (s2 ∘ s1) = apply s2 ∘ apply s1, so s1 should take precedence over any bindings for the same type variable.
    for (key, val) in s2.iter() {
        s3.insert(*key, val.clone());
    }
    for (key, val) in s1.iter() {
        s3.insert(*key, val.subst(s2));
    }
    s3
}

pub trait TypeVars<A> {
    fn all_vars(&self) -> HashSet<char>;
    fn free_vars(&self) -> HashSet<char>;
    fn subst(&self, &HashMap<char, Type>) -> A;
}

impl TypeVars<Type> for Type {
    fn all_vars(&self) -> HashSet<char> {
        match *self {
            TVar(ref a) => vec!(*a).into_iter().collect(),
            TArrow(ref t1, ref t2) => t1.all_vars().union(&t2.all_vars()).cloned().collect(),
            _ => HashSet::new()
        }
    }

    fn free_vars(&self) -> HashSet<char> {
        self.all_vars()
    }

    fn subst(&self, s: &HashMap<char, Type>) -> Type {
        match *self {
            TVar(ref n) => match s.get(n) {
                Some(t) => (*t).clone(),
                None => TVar(*n)
            },
            TArrow(ref t1, ref t2) => TArrow(Box::new(t1.subst(s)), Box::new(t2.subst(s))),
            _ => self.clone()
        }
    }
}

impl TypeVars<Scheme> for Scheme {
    fn all_vars(&self) -> HashSet<char> {
        match *self {
            Mono(ref t) => t.all_vars(),
            Poly(ref a, ref t) => {
                let mut av = t.all_vars();
                av.insert(*a);
                av
            } 
        }
    }

    fn free_vars(&self) -> HashSet<char> {
        match *self {
            Mono(ref t) => t.free_vars(),
            Poly(ref a, ref t) => {
                let mut fv = t.free_vars();
                fv.remove(a);
                fv
            }
        }
    }

    fn subst(&self, s: &HashMap<char, Type>) -> Scheme {
        match *self {
            Mono(ref t) => Mono(Box::new(t.subst(s))),
            Poly(ref a, ref t) => {
                let mut rule = s.clone();
                rule.remove(a);
                Poly(*a, Box::new(t.subst(&rule)))
            }
        }
    }
}
