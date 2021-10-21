use crate::front::parser::ast::Program;

// traverse the AST and check the main function
#[inline]
pub fn analyze(_program: &Program) {
  print!("\nmaincheck: no\n");
}
