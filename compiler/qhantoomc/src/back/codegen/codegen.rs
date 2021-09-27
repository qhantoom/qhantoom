use super::cranelift::Jit as CraneliftJit;
use super::interface::CodeGenerator;
use super::llvm::Jit as LLVMJit;

use crate::front::parser::parse;
use crate::front::parser::ast::Pkg;

// code generation to machine code using llvm backend
#[inline]
pub unsafe fn codegen_with_llvm(code: &str) {
  let ast = parse(code).unwrap();
  let mut codegen = Codegen::new(box LLVMJit {});

  codegen.codegen(ast);
}

// code generation to machine code using cranelift backend
#[inline]
pub fn codegen_with_cranelift(code: &str) {
  let mut codegen = Codegen::new(box CraneliftJit {});
}

pub struct Codegen<T: ?Sized>
where
  T: CodeGenerator + Clone,
{
  jit: Box<T>,
}

impl<T> Codegen<T>
where
  T: CodeGenerator + Clone,
{
  #[inline]
  pub fn new(jit: Box<T>) -> Self {
    Self { jit: jit }
  }

  #[inline]
  pub unsafe fn codegen(&mut self, pkg: Pkg) {
    self.jit.codegen(pkg);
  }
}
