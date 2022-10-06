mod mainchecker;
mod namechecker;
mod typechecker;

use crate::front::parser::ast::Program;

pub fn analyze(program: &Program) -> Result<(), String> {
  mainchecker::check(program);
  namechecker::check(program);
  typechecker::check(program);

  Ok(())
}
