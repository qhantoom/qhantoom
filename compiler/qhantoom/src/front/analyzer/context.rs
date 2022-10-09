use super::scope::ScopeMap;

use crate::front::parser::ast::{pbox, PBox, Program, Ty};

#[derive(Clone, Debug)]
pub struct Context<'a> {
  pub scope_map: ScopeMap,
  pub loops: u32,
  pub program: &'a Program,
  pub return_ty: PBox<Ty>,
}

impl<'a> Context<'a> {
  pub fn new(program: &'a Program) -> Self {
    let mut scope_map = ScopeMap::default();

    // TODO: tmp
    scope_map
      .set_fun("puts".to_string(), (Ty::VOID.into(), vec![Ty::STR.into()]))
      .unwrap();

    Self {
      scope_map,
      loops: 0,
      program,
      return_ty: pbox(Ty::VOID),
    }
  }
}
