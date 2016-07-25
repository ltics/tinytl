#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(stainless)]

extern crate tinytl;
pub use tinytl::syntax::*;

describe! syntax_display_spec {
    it "display syntax" {
        assert_eq!(format!("{}", EVar("x")), "x");
        assert_eq!(format!("{}", EAbs("x", box EVar("x"))), "λ x → x");
        assert_eq!(format!("{}", EApp(box EVar("x"), box EVar("y"))), "x(y)");
        assert_eq!(format!("{}", ELet("x", box EVar("x"), box EApp(box EVar("x"), box EVar("y")))), "let x = x in x(y)");
    }
}
