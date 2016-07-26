use std::collections::HashMap;
use types::*;

#[allow(dead_code)]
pub fn replace_free_vars(scheme: &Scheme, rule: &HashMap<&'static str, Type>) -> Type {
    match *scheme {
        Mono(ref t) => t.subst(rule),
        Poly(_, ref t) => replace_free_vars(t, rule)
    }
}

#[allow(dead_code)]
pub fn occurs(tname: &'static str, t: &Type) -> bool {
    t.free_vars().contains(tname)
}

#[allow(dead_code)]
pub fn generalize(env: &HashMap<&'static str, Scheme>, t: &Type) -> Scheme {
    t.free_vars().difference(&env.free_vars()).fold(Mono(Box::new((*t).clone())), |scheme, fv| {
        Poly(fv, Box::new(scheme))
    })
}

#[allow(dead_code)]
pub fn make_single_subrule(tname: &'static str, t: &Type) -> HashMap<&'static str, Type> {
    match (tname, (*t).clone()) {
        (a, TVar(name)) if a == name => HashMap::new(),
        (a, ref t) if occurs(a, t) => panic!("occurs check fails"),
        (a, t) => {
            let mut m: HashMap<&'static str, Type> = HashMap::new();
            m.insert(a, t);
            m
        }
    }
}
