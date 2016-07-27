use std::collections::HashMap;
use syntax::*;
use types::*;

static mut VAR: u8 = 97;

fn get_new_var() -> Type {
    unsafe {
        let fresh = TVar(VAR as char);
        VAR += 1;
        fresh
    }
}

fn replace_free_vars(scheme: &Scheme, rule: &HashMap<char, Type>) -> Type {
    match *scheme {
        Mono(ref t) => t.subst(rule),
        Poly(_, ref t) => replace_free_vars(t, rule)
    }
}

fn occurs(tname: &char, t: &Type) -> bool {
    t.free_vars().contains(tname)
}

fn generalize(env: &HashMap<&'static str, Scheme>, t: &Type) -> Scheme {
    t.free_vars().difference(&env.free_vars()).fold(Mono(Box::new((*t).clone())), |scheme, fv| {
        Poly(*fv, Box::new(scheme))
    })
}

fn instantiate(t: &Scheme) -> Type {
    let mut env: HashMap<char, Type> = HashMap::new();
    for var in t.all_vars().difference(&t.free_vars()) {
        unsafe {
            env.insert(*var, TVar(VAR as char));
            VAR += 1;
        }
    }
    replace_free_vars(t, &env)
}

fn make_single_subrule(tname: &char, t: &Type) -> HashMap<char, Type> {
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

fn assoc_env(tname: &'static str, scheme: &Scheme, env: &HashMap<&'static str, Scheme>) -> HashMap<&'static str, Scheme> {
    let mut new_env = (*env).clone();
    new_env.remove(tname);
    new_env.insert(tname, (*scheme).clone());
    new_env
}

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

pub fn algw(env: &HashMap<&'static str, Scheme>, expr: &Expr) -> (HashMap<char, Type>, Type) {
    match *expr {
        EVar(ref name) => match env.get(name) {
            Some(t) => (HashMap::new(), instantiate(t)),
            None => panic!("unbound variable: {}", name)
        },
        EAbs(ref name, ref expr) => {
            let fresh = get_new_var();
            let new_env = assoc_env(name, &Mono(Box::new(fresh.clone())), env);
            let (subrule, mono) = algw(&new_env, expr);
            (HashMap::new(), TArrow(Box::new(fresh.clone().subst(&subrule)), Box::new(mono)))
        }
        EApp(ref e1, ref e2) => {
            let (s1, m1) = algw(env, e1);
            let (s2, m2) = algw(env, e2);
            let fresh = get_new_var();
            let s3 = unify(&m1.subst(&s2), &TArrow(Box::new(m2), Box::new(fresh.clone())));
            (compose(&s3, &compose(&s2, &s1)), fresh.clone().subst(&s3))
        }
        ELet(ref name, ref value, ref body) => {
            let (s1, value_mono) = algw(env, value);
            let env1 = env.subst(&s1);
            let g = generalize(&env1, &value_mono);
            let env2 = assoc_env(name, &g, &env1);
            let (s2, body_mono) = algw(&env2, body);
            (compose(&s2, &s1), body_mono)
        }
    }
}
