use crate::front::parser::ast::Pkg;

pub trait CodeGenerator {
  unsafe fn generate(&mut self, pkg: &Pkg);
}
