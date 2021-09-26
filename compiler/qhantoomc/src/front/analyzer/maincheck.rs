use crate::front::parser::ast::{Item, ItemKind, Pkg};

// traverse the AST and check the main function
#[inline]
pub fn check(pkg: &Pkg) {
  if !pkg.items.iter().any(|item| is_main(item)) {
    panic!("main function not found");
  }
}

// check if the item is main function 
#[inline]
pub fn is_main(item: &Item) -> bool {
  if let ItemKind::Fun(ref fun) = item.kind() {
    if fun.name() == "main" {
      if !fun.args.is_empty() {
        panic!("main function must have no arguments");
      }

      return true;
    }
  }

  false
}
