use crate::back::codegen::interface::CodeGenerator;
use crate::front::parser::ast::{BinopKind, Expr, Item, Pkg, Stmt};

// note: this is a bit of a hack, but it's the easiest way to get the cranelift jit
// i am following instructions from https://github.com/bytecodealliance/cranelift-jit-demo

pub struct Jit {}

impl CodeGenerator for Jit {
  unsafe fn codegen(&mut self, pkg: &Pkg) {}
}

impl Jit {
  #[inline]
  pub fn new() -> Self {
    Self {}
  }
}
