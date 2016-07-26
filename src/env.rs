use types::*;

use std::collections::HashMap;
use std::collections::HashSet;

impl TypeVars<HashMap<&'static str, Scheme>> for HashMap<&'static str, Scheme> {
    fn all_vars(&self) -> HashSet<char> {
        (*self).values().fold(HashSet::new(), |avs, t| {
            avs.union(&t.all_vars()).cloned().collect()
        })
    }

    fn free_vars(&self) -> HashSet<char> {
        (*self).values().fold(HashSet::new(), |fvs, t| {
            fvs.union(&t.free_vars()).cloned().collect()
        })
    }

    fn subst(&self, s: &HashMap<char, Type>) -> HashMap<&'static str, Scheme> {
        let mut new_env: HashMap<&'static str, Scheme> = HashMap::new();
        for (key, val) in (*self).iter() {
            new_env.insert(key, val.subst(s));
        }
        new_env
    }
}

pub fn get_assumptions() -> HashMap<&'static str, Scheme> {
    let mut assumptions: HashMap<&'static str, Scheme> = HashMap::new();
    assumptions.insert("zero", Mono(Box::new(TInt)));
    assumptions.insert("one", Mono(Box::new(TInt)));
    assumptions.insert("true", Mono(Box::new(TBool)));
    assumptions.insert("false", Mono(Box::new(TBool)));
    assumptions.insert("not", Mono(Box::new(TArrow(Box::new(TBool), Box::new(TBool)))));
    assumptions.insert("and", Mono(Box::new(TArrow(Box::new(TBool), Box::new(TArrow(Box::new(TBool), Box::new(TBool)))))));
    assumptions.insert("add", Mono(Box::new(TArrow(Box::new(TInt), Box::new(TArrow(Box::new(TInt), Box::new(TInt)))))));
    assumptions.insert("id", Poly('a', Box::new(Mono(Box::new(TArrow(Box::new(TVar('a')), Box::new(TVar('a'))))))));
    assumptions.insert("eq", Poly('a', Box::new(Mono(Box::new(TArrow(Box::new(TVar('a')), Box::new(TArrow(Box::new(TVar('a')), Box::new(TBool)))))))));
    assumptions.insert("compose", Poly('a', Box::new(Poly('b', Box::new(Poly('c', Box::new(Mono(Box::new(TArrow(Box::new(TArrow(Box::new(TVar('b')), Box::new(TVar('c')))), Box::new(TArrow(Box::new(TArrow(Box::new(TVar('a')), Box::new(TVar('b')))), Box::new(TArrow(Box::new(TVar('a')), Box::new(TVar('c'))))))))))))))));
    assumptions.insert("choose", Poly('a', Box::new(Mono(Box::new(TArrow(Box::new(TVar('a')), Box::new(TArrow(Box::new(TVar('a')), Box::new(TVar('a'))))))))));
    assumptions.insert("wrong", Poly('a', Box::new(Mono(Box::new(TVar('a'))))));
    assumptions
}
