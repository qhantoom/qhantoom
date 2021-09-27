use crate::back::codegen::interface::CodeGenerator;
use crate::front::parser::ast::{BinopKind, Expr, Item, Pkg, Stmt};

use std::ptr;
use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

#[derive(Clone)]
pub struct Jit {}

impl CodeGenerator for Jit {
  #[inline]
  unsafe fn codegen(&mut self, pkg: Pkg) {
    // Set up a context, module and builder in that context.
    let context = llvm_sys::core::LLVMContextCreate();
    let module = llvm_sys::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
    let builder = llvm_sys::core::LLVMCreateBuilderInContext(context);

    // In LLVM, you get your types from functions.
    let int_type = llvm_sys::core::LLVMInt64TypeInContext(context);
    let function_type = llvm_sys::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function = llvm_sys::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

    let entry_name = CString::new("entry").unwrap();
    let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
    llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

    // The juicy part: construct a `LLVMValue` from a Rust value:
    let int_value: u64 = "42".parse().unwrap();
    let int_value = llvm_sys::core::LLVMConstInt(int_type, int_value, 0);

    llvm_sys::core::LLVMBuildRet(builder, int_value);

    // Instead of dumping to stdout, let's write out the IR to `out.ll`
    let out_file = CString::new("out.ll").unwrap();
    llvm_sys::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

    llvm_sys::core::LLVMDisposeBuilder(builder);
    llvm_sys::core::LLVMDisposeModule(module);
    llvm_sys::core::LLVMContextDispose(context);
  }
}

impl Jit {
  #[inline]
  fn new() -> Self {
    Self {}
  }

  #[inline]
  fn codegen_item(&mut self, item: Box<Item>) {
    match item.kind() {
      _ => todo!(),
    }
  }

  #[inline]
  fn codegen_stmt(&mut self, stmt: Box<Stmt>) {
    match stmt.kind() {
      _ => todo!(),
    }
  }

  #[inline]
  fn codegen_expr(&mut self, expr: Box<Expr>) {
    match expr.kind() {
      _ => todo!(),
    }
  }

  #[inline]
  fn codegen_binop_expr(
    &mut self,
    lhs: Box<Expr>,
    op: BinopKind,
    rhs: Box<Expr>,
  ) {
    match (lhs.kind(), rhs.kind()) {
      _ => todo!(),
    }
  }
}
