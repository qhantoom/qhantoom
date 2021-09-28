use super::cranelift::Jit as CraneliftJit;
use super::interface::CodeGenerator;
use super::llvm::Jit as LLVMJit;

use crate::front::parser::ast::Pkg;
use crate::front::parser::parse;

// code generation to machine code using llvm backend
#[inline]
pub unsafe fn codegen_with_llvm(code: &str) {
  let ast = parse(code).unwrap();
  let mut codegen = Codegen::new(box LLVMJit::new("example_module"));

  codegen.codegen(ast);
}

// code generation to machine code using cranelift backend
#[inline]
pub unsafe fn codegen_with_cranelift(code: &str) {
  let ast = parse(code).unwrap();
  let mut codegen = Codegen::new(box CraneliftJit {});

  codegen.codegen(ast);
}

pub struct Codegen<T: ?Sized>
where
  T: CodeGenerator,
{
  jit: Box<T>,
}

impl<T> Codegen<T>
where
  T: CodeGenerator,
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
