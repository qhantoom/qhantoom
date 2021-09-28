use super::context::Context;

use crate::back::codegen::interface::CodeGenerator;

use crate::front::parser::ast::{
  BinopKind, Expr, ExprKind, Item, Pkg, Stmt, StmtKind, UnopKind,
};

use crate::util::cstring::cstr;

use std::ffi::CString;
use std::ptr;

use llvm_sys::core::*;

use llvm_sys::prelude::{
  LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMValueRef,
};

// note: this is a bit of a hack, but it's the easiest way to get the LLVM
// i am following instruction from https://blog.ulysse.io/post/a-toy-front-end-for-llvm-written-in-rust

pub struct Jit {
  context: Context,
}

impl CodeGenerator for Jit {
  #[inline]
  unsafe fn codegen(&mut self, pkg: Pkg) {
    // in LLVM, you get your types from functions.
    let int_type = llvm_sys::core::LLVMInt64TypeInContext(self.context.current);
    let function_type =
      llvm_sys::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function = llvm_sys::core::LLVMAddFunction(
      self.context.module,
      cstr!("main"),
      function_type,
    );

    let entry_name = cstr!("entry");
    let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(
      self.context.current,
      function,
      entry_name,
    );
    llvm_sys::core::LLVMPositionBuilderAtEnd(self.context.builder, bb);

    // the juicy part: construct a `LLVMValue` from a Rust value:
    let int_value: u64 = "42".parse().unwrap();
    let int_value = llvm_sys::core::LLVMConstInt(int_type, int_value, 0);

    llvm_sys::core::LLVMBuildRet(self.context.builder, int_value);

    // write out the IR to `out.ll`
    let out_file = cstr!("out.ll");
    llvm_sys::core::LLVMPrintModuleToFile(
      self.context.module,
      out_file,
      ptr::null_mut(),
    );
  }
}

impl Jit {
  #[inline]
  pub unsafe fn new(mod_name: &str) -> Self {
    Self {
      context: Context::new(mod_name),
    }
  }

  #[inline]
  unsafe fn codegen_item(&mut self, item: Box<Item>) {
    match item.kind() {
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn codegen_stmt(&mut self, stmt: Box<Stmt>) {
    match stmt.kind() {
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn codegen_expr(&mut self, expr: &Box<Expr>) -> LLVMValueRef {
    match expr.kind() {
      ExprKind::Int(ref expr) => self.codegen_int(*expr),
      ExprKind::Float(ref expr) => self.codegen_float(*expr),
      ExprKind::Bool(ref expr) => self.codegen_bool(*expr),
      ExprKind::Char(ref expr) => self.codegen_char(*expr),
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn codegen_bool(&mut self, expr: bool) -> LLVMValueRef {
    let bool_type = llvm_sys::core::LLVMInt1TypeInContext(self.context.current);
    LLVMConstInt(bool_type, expr as u64, 0)
  }

  #[inline]
  unsafe fn codegen_int(&mut self, expr: i32) -> LLVMValueRef {
    let int_type = llvm_sys::core::LLVMInt64TypeInContext(self.context.current);
    LLVMConstInt(int_type, expr as u64, 0)
  }

  #[inline]
  unsafe fn codegen_float(&mut self, expr: f32) -> LLVMValueRef {
    let float_type =
      llvm_sys::core::LLVMDoubleTypeInContext(self.context.current);
    LLVMConstReal(float_type, expr as f64)
  }

  #[inline]
  unsafe fn codegen_char(&mut self, expr: char) -> LLVMValueRef {
    let char_type =
      llvm_sys::core::LLVMInt32TypeInContext(self.context.current);
    LLVMConstInt(char_type, expr as u64, 0)
  }
}
