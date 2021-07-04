use super::initcheck;
use super::maincheck;
use super::typecheck;

use crate::front::parser::ast::Capsule;

use crate::front::parser::{
  parse_capsule_from_file, parse_capsule_from_source,
};

use std::path::Path;

#[inline]
pub fn analyze(capsule: &Box<Capsule>) {
  let mut analyzer = Analyzer::new();
  analyzer.analyze(&capsule);
}

#[inline]
pub fn analyze_capsule_from_file(path: &Path) -> Result<(), String> {
  let capsule = parse_capsule_from_file(path)?;
  let mut analyzer = Analyzer::new();
  analyzer.analyze(&capsule);

  Ok(())
}

#[inline]
pub fn analyze_capsule_from_source(src: &str) -> Result<(), String> {
  let capsule = parse_capsule_from_source(src)?;
  let mut analyzer = Analyzer::new();
  analyzer.analyze(&capsule);

  Ok(())
}

pub struct Analyzer;

impl Analyzer {
  #[inline]
  pub const fn new() -> Self {
    Self {}
  }

  #[inline]
  pub fn analyze(&mut self, capsule: &Box<Capsule>) {
    typecheck::check(capsule);
    maincheck::check(capsule);
    initcheck::check(capsule);
  }
}
