use crate::back::codegen::interface::CodeGenerator;
use crate::front::parser::ast::{BinopKind, Expr, Item, Pkg, Stmt};

#[derive(Clone)]
pub struct Jit;

impl CodeGenerator for Jit {
  unsafe fn codegen(&mut self, pkg: Pkg) {}
}
