use super::scope::ScopeMap;

use crate::front::parser::ast::{pbox, Program, Ty};

#[derive(Debug)]
pub struct Context<'a> {
  pub scope_map: ScopeMap,
  pub loops: u32,
  pub program: &'a Program,
  pub return_ty: Box<Ty>,
}

impl<'a> Context<'a> {
  pub fn new(program: &'a Program) -> Self {
    let mut scope_map = ScopeMap::default();

    // TODO: tmp
    scope_map
      .set_fun("puts".to_string(), (pbox(Ty::VOID), vec![pbox(Ty::STR)]))
      .unwrap();

    Self {
      scope_map,
      loops: 0,
      program,
      return_ty: Box::new(Ty::VOID),
    }
  }
}