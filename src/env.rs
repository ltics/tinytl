use types::*;

use std::collections::HashMap;
use std::collections::HashSet;

impl TypeVars<HashMap<&'static str, Scheme>> for HashMap<&'static str, Scheme> {
    fn all_vars(&self) -> HashSet<&'static str> {
        (*self).values().fold(HashSet::new(), |avs, t| {
            avs.union(&t.all_vars()).cloned().collect()
        })
    }

    fn free_vars(&self) -> HashSet<&'static str> {
        (*self).values().fold(HashSet::new(), |fvs, t| {
            fvs.union(&t.free_vars()).cloned().collect()
        })
    }

    fn subst(&self, s: &HashMap<&'static str, Type>) -> HashMap<&'static str, Scheme> {
        let mut new_env: HashMap<&'static str, Scheme> = HashMap::new();
        for (key, val) in (*self).iter() {
            new_env.insert(key, val.subst(s));
        }
        new_env
    }
}
