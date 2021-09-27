use crate::util::cstring::cstr;

use llvm_sys::core::{
  LLVMContextCreate, LLVMModuleCreateWithName,
  LLVMCreateBuilderInContext,
};

use llvm_sys::prelude::{
  LLVMContextRef, LLVMModuleRef, LLVMBuilderRef,
};

pub struct Context {
  name: String,
  context: LLVMContextRef,
  module: LLVMModuleRef,
  builder: LLVMBuilderRef,
}

impl Context {
  #[inline]
  pub unsafe fn new(name: &str) -> Self {
    let context = LLVMContextCreate();
    let module = LLVMModuleCreateWithName(cstr!(name));
    let builder = LLVMCreateBuilderInContext(context);

    Self {
      name: name.into(),
      context: context,
      module: module,
      builder: builder,
    }
  }
}
