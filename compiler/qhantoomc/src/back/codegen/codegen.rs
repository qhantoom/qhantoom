use super::cranelift::Jit as CraneliftJit;
use super::interface::CodeGenerator;
use super::llvm::Jit as LLVMJit;

use crate::front::parser::ast::Pkg;
use crate::front::parser::parse;

// code generation to machine code using llvm backend
#[inline]
pub unsafe fn codegen_with_llvm(code: &str) {
  let ast = parse(code).unwrap();
  let jit = LLVMJit::new("example_module");
  let mut codegen = Codegen::new(jit);

  codegen.codegen(ast);
}

// code generation to machine code using cranelift backend
#[inline]
pub unsafe fn codegen_with_cranelift(code: &str) {
  let ast = parse(code).unwrap();
  let jit = CraneliftJit::new();
  let mut codegen = Codegen::new(jit);

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
  pub fn new(jit: T) -> Self {
    Self { jit: box jit }
  }

  #[inline]
  pub unsafe fn codegen(&mut self, pkg: Pkg) {
    self.jit.codegen(pkg);
  }
}
