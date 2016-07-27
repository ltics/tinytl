#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(stainless)]

extern crate tinytl;
pub use tinytl::syntax::*;
pub use tinytl::types::*;
pub use tinytl::infer::*;
pub use tinytl::env::*;
pub use std::collections::HashMap;

pub fn run_infer_spec(expr: &Expr, expect: &'static str) {
    assert_eq!(format!("{}", generalize(&HashMap::new(), &infer(&get_assumptions(), expr))), expect);
    reset_var();
}

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

    it "should infer" {
        run_infer_spec(&EVar("id"), "∀a. a → a");
        run_infer_spec(&EApp(Box::new(EVar("id")), Box::new(EApp(Box::new(EVar("id")), Box::new(EVar("one"))))), "int");
        run_infer_spec(&EApp(Box::new(EApp(Box::new(EVar("eq")), Box::new(EVar("false")))), Box::new(EVar("true"))), "bool");
        run_infer_spec(&EVar("compose"), "∀a. ∀b. ∀c. (b → c) → (a → b) → a → c");
        run_infer_spec(&EApp(Box::new(EVar("compose")), Box::new(EVar("not"))), "∀a. (a → bool) → a → bool");
        run_infer_spec(&EApp(Box::new(EApp(Box::new(EVar("compose")), Box::new(EVar("not")))), Box::new(EApp(Box::new(EVar("eq")), Box::new(EVar("one"))))), "int → bool");
        run_infer_spec(&EApp(Box::new(EVar("compose")), Box::new(EApp(Box::new(EVar("add")), Box::new(EVar("one"))))), "∀a. (a → int) → a → int");
        run_infer_spec(&EApp(Box::new(EApp(Box::new(EApp(Box::new(EVar("compose")), Box::new(EVar("eq")))), Box::new(EVar("add")))), Box::new(EVar("one"))), "(int → int) → bool");
        run_infer_spec(&EApp(Box::new(EVar("compose")), Box::new(EVar("compose"))), "∀a. ∀d. ∀e. ∀f. (a → e → f) → a → (d → e) → d → f");
        run_infer_spec(&EAbs("a", Box::new(ELet("x", Box::new(EAbs("b", Box::new(ELet("y", Box::new(EAbs("c", Box::new(EApp(Box::new(EVar("a")), Box::new(EVar("zero")))))), Box::new(EApp(Box::new(EVar("y")), Box::new(EVar("one")))))))), Box::new(EApp(Box::new(EVar("x")), Box::new(EVar("one"))))))), "∀h. (int → h) → h");
        run_infer_spec(&EApp(Box::new(EApp(Box::new(EVar("choose")), Box::new(EAbs("a", Box::new(EAbs("b", Box::new(EVar("a")))))))), Box::new(EAbs("a", Box::new(EAbs("b", Box::new(EVar("b"))))))), "∀f. f → f → f");
        run_infer_spec(&EAbs("x", Box::new(EAbs("y", Box::new(ELet("x", Box::new(EApp(Box::new(EVar("x")), Box::new(EVar("y")))), Box::new(EAbs("x", Box::new(EApp(Box::new(EVar("y")), Box::new(EVar("x"))))))))))), "∀c. ∀d. ∀e. ((d → e) → c) → (d → e) → d → e");
    }
}
