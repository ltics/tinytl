use std::collections::HashMap;
use types::*;

static mut VAR: u8 = 97;

#[allow(dead_code)]
pub fn replace_free_vars(scheme: &Scheme, rule: &HashMap<char, Type>) -> Type {
    match *scheme {
        Mono(ref t) => t.subst(rule),
        Poly(_, ref t) => replace_free_vars(t, rule)
    }
}

#[allow(dead_code)]
pub fn occurs(tname: &char, t: &Type) -> bool {
    t.free_vars().contains(tname)
}

#[allow(dead_code)]
pub fn generalize(env: &HashMap<&'static str, Scheme>, t: &Type) -> Scheme {
    t.free_vars().difference(&env.free_vars()).fold(Mono(Box::new((*t).clone())), |scheme, fv| {
        Poly(*fv, Box::new(scheme))
    })
}

pub fn update(env: &HashMap<char, Type>, a: &char) -> HashMap<char, Type> {
    unsafe {
        let mut new_env = (*env).clone();
        new_env.insert(*a, TVar(VAR as char));
        VAR += 1;
        new_env
    }
}

/*#[allow(dead_code)]
pub fn instantiate(scheme: &Scheme) -> Type {
    let t = (*scheme).clone();
    let bound_vars = (t.all_vars()).clone().difference(&t.free_vars());
    TInt
}*/

#[allow(dead_code)]
pub fn make_single_subrule(tname: &char, t: &Type) -> HashMap<char, Type> {
    match (*tname, (*t).clone()) {
        (a, TVar(name)) if a == name => HashMap::new(),
        (a, ref t) if occurs(&a, t) => panic!("occurs check fails"),
        (a, t) => {
            let mut m: HashMap<char, Type> = HashMap::new();
            m.insert(a, t);
            m
        }
    }
}

#[allow(dead_code)]
pub fn assoc_env(tname: &'static str, scheme: &Scheme, env: &HashMap<&'static str, Scheme>) -> HashMap<&'static str, Scheme> {
    let mut new_env = (*env).clone();
    new_env.remove(tname);
    new_env.insert(tname, (*scheme).clone());
    new_env
}
