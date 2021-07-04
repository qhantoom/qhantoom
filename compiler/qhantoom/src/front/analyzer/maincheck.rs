use crate::front::parser::ast::{Capsule, Item, ItemKind};
use crate::util::session::session;
use crate::util::span::SPAN_ZERO;

const MAIN: &str = "main";

#[inline]
pub fn check(capsule: &Box<Capsule>) {
  let mut mainchecker = Mainchecker::new();
  mainchecker.run(capsule);
}

pub struct Mainchecker;

impl Mainchecker {
  #[inline]
  pub fn new() -> Self {
    Self {}
  }

  #[inline]
  fn run(&mut self, capsule: &Box<Capsule>) {
    if !capsule.module.items().iter().any(|item| self.is_main(item)) {
      session().abort(&format!("\nmain function not found\n"), SPAN_ZERO);
    }
  }

  #[inline]
  fn is_main(&mut self, item: &Box<Item>) -> bool {
    if let ItemKind::Fun(ref fun) = item.kind() {
      if &fun.name() == MAIN {
        if fun.args.len() != 0 {
          session().abort(
            &format!("\nno arguments needed for `{}` function\n", MAIN),
            item.span,
          );
        }

        return true;
      }
    }

    false
  }
}
