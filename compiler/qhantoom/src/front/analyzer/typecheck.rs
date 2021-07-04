use crate::front::parser::ast::{
  Capsule, FunDecl, Ident, Item, ItemKind, Ty, TyKind,
};

// use crate::util::session::session;
use crate::util::span::SPAN_ZERO;

use std::collections::HashMap;

#[inline]
pub fn check(capsule: &Capsule) {
  let mut typecheck = Typechecker::new(capsule);
  typecheck.run();
}

pub struct Typechecker<'a> {
  capsule: &'a Capsule,
  functions: HashMap<Ident, (Ty, Vec<Ty>)>,
  structs: HashMap<Ident, Option<HashMap<Ident, Ty>>>,
  vars: HashMap<Ident, Ty>,
  loops: u32,
  ret: Ty,
}

impl<'a> Typechecker<'a> {
  #[inline]
  pub fn new(capsule: &'a Capsule) -> Self {
    Self {
      capsule,
      functions: HashMap::new(),
      structs: HashMap::new(),
      vars: HashMap::new(),
      loops: 0,
      ret: Ty::new(TyKind::Void, SPAN_ZERO),
    }
  }

  #[inline]
  pub fn run(&mut self) {
    for item in self.capsule.module.items() {
      self.check_item(item)
    }
  }

  #[inline]
  pub fn check_item(&mut self, item: &Box<Item>) {
    match item.kind() {
      ItemKind::Fun(ref fun) => self.check_fun_item(fun),
      _ => unreachable!(),
    }
  }

  #[inline]
  pub fn check_fun_item(&mut self, _fun: &Box<FunDecl>) {}
}
