#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(stainless)]

extern crate tinytl;
pub use tinytl::types::*;
pub use tinytl::infer::*;
pub use std::collections::HashMap;

describe! infer_spec {
    it "should unify" {
        let mono1 = &TArrow(Box::new(TVar('a')), Box::new(TInt));
        let mono2 = &TArrow(Box::new(TVar('b')), Box::new(TVar('b')));
        let mono3 = &TArrow(Box::new(TVar('a')), Box::new(TVar('b')));
        let mono4 = &TArrow(Box::new(TArrow(Box::new(TVar('b')), Box::new(TVar('c')))), Box::new(TVar('c')));
        let subrule1 = unify(mono1, mono2);
        let subrule2 = unify(mono3, mono4);
        assert_eq!(subrule1.get(&'a'), Some(&TInt));
        assert_eq!(subrule1.get(&'b'), Some(&TInt));
        assert_eq!(subrule2.get(&'a'), Some(&TArrow(Box::new(TVar('c')), Box::new(TVar('c')))));
        assert_eq!(subrule2.get(&'b'), Some(&TVar('c')));
    }
}
