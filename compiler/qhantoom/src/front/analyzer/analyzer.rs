use super::checker::{maincheck, semacheck, typecheck};

use crate::front::parser::ast::Program;

pub struct Analyzer<'a> {
  program: &'a Program,
}

impl<'a> Analyzer<'a> {
  pub fn new(program: &'a Program) -> Self {
    Self { program }
  }

  pub fn analyze(&mut self) -> bool {
    let mainchecked = maincheck::analyze(&self.program);
    let semachecked = semacheck::analyze(&self.program);
    let typechecked = typecheck::analyze(&self.program);

    mainchecked && semachecked && typechecked
  }
}
