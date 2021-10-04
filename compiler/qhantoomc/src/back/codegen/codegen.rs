use super::cranelift::Jit as CraneliftJit;
use super::interface::CodeGenerator;
use super::llvm::Jit as LLVMJit;

use crate::front::parser::ast::Pkg;
use crate::front::parser::parse;

// code generation to machine code using llvm backend
#[inline]
pub unsafe fn codegen_with_llvm(ast: &Pkg) {
  let jit = LLVMJit::new("example_module");
  let mut codegen = Codegen::new(jit);

  codegen.generate(ast);
}

// code generation to machine code using cranelift backend
#[inline]
pub unsafe fn codegen_with_cranelift(ast: &Pkg) {
  let jit = CraneliftJit::new();
  let mut codegen = Codegen::new(jit);

  codegen.generate(ast);
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
  pub fn new(jit: T) -> Self {
    Self { jit: box jit }
  }

  #[inline]
  pub unsafe fn generate(&mut self, pkg: &Pkg) {
    self.jit.generate(pkg);
  }
}
