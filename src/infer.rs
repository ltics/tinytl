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

#[allow(dead_code)]
pub fn instantiate(t: &Scheme) -> Type {
    let mut env: HashMap<char, Type> = HashMap::new();
    for var in t.all_vars().difference(&t.free_vars()) {
        unsafe {
            env.insert(*var, TVar(VAR as char));
            VAR += 1;
        }
    }
    replace_free_vars(t, &env)
}

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

#[allow(dead_code)]
pub fn unify(t1: &Type, t2: &Type) -> HashMap<char, Type> {
    match ((*t1).clone(), (*t2).clone()) {
        (TInt, TInt) => HashMap::new(),
        (TBool, TBool) => HashMap::new(),
        (TVar(n), t) => make_single_subrule(&n, &t),
        (t, TVar(n)) => make_single_subrule(&n, &t),
        (TArrow(ref tl1, ref tr1), TArrow(ref tl2, ref tr2)) => {
            let s1 = &unify(tl1, tl2);
            let s2 = &unify(&tr1.subst(s1), &tr2.subst(s1));
            compose(s2, s1)
        },
        _ => HashMap::new()
    }
}
