#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(stainless)]

extern crate tinytl;
pub use tinytl::types::*;

describe! types_display_spec {
    it "display types" {
        assert_eq!(format!("{}", Mono(box TArrow(box TInt, box TBool))), "int → bool");
        assert_eq!(format!("{}", Poly('x', box Mono(box TArrow(box TArrow(box TInt, box TBool), box TVar('x'))))), "∀x. (int → bool) → x");
    }
}
