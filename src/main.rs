extern crate tinytl;
pub use tinytl::syntax::*;
pub use tinytl::types::*;
pub use tinytl::infer::*;
pub use tinytl::env::*;
pub use std::collections::HashMap;

fn main() {
    println!("{}", generalize(&HashMap::new(), &infer(&get_assumptions(), &EAbs("a", Box::new(ELet("x", Box::new(EAbs("b", Box::new(ELet("y", Box::new(EAbs("c", Box::new(EApp(Box::new(EVar("a")), Box::new(EVar("zero")))))), Box::new(EApp(Box::new(EVar("y")), Box::new(EVar("one")))))))), Box::new(EApp(Box::new(EVar("x")), Box::new(EVar("one"))))))))));
}
