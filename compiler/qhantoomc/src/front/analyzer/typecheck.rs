use crate::front::parser::ast::Program;

// traverse the AST and check the main function
#[inline]
pub fn check(_program: &Program) {
  print!("typecheck: no\n");
}
