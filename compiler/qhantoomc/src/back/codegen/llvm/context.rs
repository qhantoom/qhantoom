use crate::util::cstring::cstr;

use llvm_sys::core::{
  LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
  LLVMDisposeBuilder, LLVMDisposeModule, LLVMModuleCreateWithName,
};

use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef};

pub struct Context {
  pub name: String,
  pub current: LLVMContextRef,
  pub module: LLVMModuleRef,
  pub builder: LLVMBuilderRef,
}

impl Drop for Context {
  // dispose llvm builder, module and context
  #[inline]
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeBuilder(self.builder);
      LLVMDisposeModule(self.module);
      LLVMContextDispose(self.current);
    }
  }
}

impl Context {
  // create a new context
  #[inline]
  pub unsafe fn new(name: &str) -> Self {
    let context = LLVMContextCreate();
    let module = LLVMModuleCreateWithName(cstr!(name));
    let builder = LLVMCreateBuilderInContext(context);

    Self {
      name: name.into(),
      current: context,
      module: module,
      builder: builder,
    }
  }
}
