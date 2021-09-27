use crate::front::parser::ast::Pkg;

pub trait CodeGenerator {
  unsafe fn codegen(&mut self, pkg: Pkg);
}
