use std::collections::HashMap;

use super::context::Context;

use crate::back::codegen::interface::CodeGenerator;

use crate::front::parser::ast::{
  BinopKind, Block, Expr, ExprKind, FunDecl, Item, ItemKind, Pkg, Stmt,
  StmtKind, UnopKind,
};

use crate::util::cstring::cstr;

use std::ffi::CString;
use std::ptr;

use llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction;
use llvm_sys::analysis::LLVMVerifyFunction;

use llvm_sys::core::{
  LLVMAddFunction, LLVMAddIncoming, LLVMAppendBasicBlock,
  LLVMAppendBasicBlockInContext, LLVMBuildAdd, LLVMBuildBr, LLVMBuildCondBr,
  LLVMBuildICmp, LLVMBuildMul, LLVMBuildNeg, LLVMBuildNot, LLVMBuildPhi,
  LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSub, LLVMBuildUDiv, LLVMConstInt,
  LLVMConstReal, LLVMCountBasicBlocks, LLVMDeleteFunction,
  LLVMDoubleTypeInContext, LLVMFunctionType, LLVMGetInsertBlock,
  LLVMGetNamedFunction, LLVMGetParam, LLVMInt1TypeInContext,
  LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMPositionBuilderAtEnd,
  LLVMPrintModuleToFile,
};

use llvm_sys::LLVMIntPredicate::LLVMIntNE;

use llvm_sys::prelude::{
  LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMValueRef,
};

// note: this is a bit of a hack, but it's the easiest way to get the LLVM jit
// i am following instructions from https://blog.ulysse.io/post/a-toy-front-end-for-llvm-written-in-rust

pub struct Jit {
  context: Context,
  functions: HashMap<String, LLVMValueRef>,
  main_function: LLVMValueRef,
  variables: HashMap<String, LLVMValueRef>,
}

impl CodeGenerator for Jit {
  #[inline]
  unsafe fn generate(&mut self, pkg: &Pkg) {
    let int_type = LLVMInt64TypeInContext(self.context.current);
    let function_type = LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);

    self.main_function =
      LLVMAddFunction(self.context.module, cstr!("main"), function_type);

    let entry_name = cstr!("entry");
    let block = LLVMAppendBasicBlockInContext(
      self.context.current,
      self.main_function,
      entry_name,
    );

    LLVMPositionBuilderAtEnd(self.context.builder, block);

    let int_type = LLVMInt64TypeInContext(self.context.current);
    let zero = LLVMConstInt(int_type, 42, 0);

    let mut return_value = zero;

    // for item in pkg.items {
    //   return_value = self.generate_item(&item);
    // }

    LLVMBuildRet(self.context.builder, return_value);

    self.print_ir_to_file();
  }
}

impl Jit {
  #[inline]
  pub unsafe fn new(mod_name: &str) -> Self {
    Self {
      context: Context::new(mod_name),
      functions: HashMap::new(),
      main_function: ptr::null_mut(),
      variables: HashMap::new(),
    }
  }

  #[inline]
  unsafe fn generate_item(&mut self, item: &Box<Item>) -> LLVMValueRef {
    match item.kind() {
      ItemKind::Fun(ref fun) => self.generate_fun_decl_item(fun),
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn generate_fun_decl_item(&mut self, fun: &FunDecl) -> LLVMValueRef {
    let name = cstr!(&fun.name());
    let mut f = LLVMGetNamedFunction(self.context.module, name);
    let empty = LLVMCountBasicBlocks(f) == 0;

    if !empty {
      panic!("function already defined");
    }

    let name = cstr!("entry");
    let bb = LLVMAppendBasicBlock(f, name);

    LLVMPositionBuilderAtEnd(self.context.builder, bb);

    let len = fun.args.len();

    for i in 0..len {
      let param = LLVMGetParam(f, i as u32);
      self.variables.insert(fun.args[i].to_string(), param);
    }

    let mut ret_value = ptr::null_mut();

    for stmt in &fun.block.stmts {
      ret_value = self.generate_stmt(stmt);
    }

    LLVMBuildRet(self.context.builder, ret_value);

    let ok = LLVMVerifyFunction(f, LLVMPrintMessageAction);

    if ok != 0 {
      LLVMDeleteFunction(f);
      panic!("function failed to verify");
    }

    f
  }

  #[inline]
  unsafe fn generate_stmt(&mut self, stmt: &Box<Stmt>) -> LLVMValueRef {
    match stmt.kind() {
      StmtKind::Return(ref expr) => self.generate_return_stmt(expr),
      StmtKind::Expr(ref expr) => self.generate_expr(expr),
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn generate_return_stmt(
    &mut self,
    expr: &Option<Box<Expr>>,
  ) -> LLVMValueRef {
    if let Some(ref e) = expr {
      let value = self.generate_expr(e);
      return LLVMBuildRet(self.context.builder, value);
    }

    LLVMBuildRetVoid(self.context.builder)
  }

  #[inline]
  unsafe fn generate_expr(&mut self, expr: &Box<Expr>) -> LLVMValueRef {
    match expr.kind() {
      ExprKind::Bool(ref expr) => self.generate_bool_expr(*expr),
      ExprKind::Int(ref expr) => self.generate_int_expr(*expr),
      ExprKind::Float(ref expr) => self.generate_float_expr(*expr),
      ExprKind::Char(ref expr) => self.generate_char_expr(*expr),
      ExprKind::Binop {
        ref lhs,
        ref op,
        ref rhs,
      } => self.generate_binop_expr(lhs, op, rhs),
      ExprKind::Unop { ref op, ref rhs } => self.generate_unop_expr(op, rhs),
      ExprKind::Ident(ref expr) => self.generate_ident_expr(expr),
      ExprKind::Assign { ref lhs, ref rhs } => {
        self.generate_assign_expr(lhs, rhs)
      }
      ExprKind::If {
        ref condition,
        ref consequence,
        ref alternative,
      } => self.generate_if_expr(condition, consequence, alternative),
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn generate_bool_expr(&mut self, expr: bool) -> LLVMValueRef {
    let bool_type = LLVMInt1TypeInContext(self.context.current);
    LLVMConstInt(bool_type, expr as u64, 0)
  }

  #[inline]
  unsafe fn generate_int_expr(&mut self, expr: i32) -> LLVMValueRef {
    let int_type = LLVMInt64TypeInContext(self.context.current);
    LLVMConstInt(int_type, expr as u64, 0)
  }

  #[inline]
  unsafe fn generate_float_expr(&mut self, expr: f32) -> LLVMValueRef {
    let float_type = LLVMDoubleTypeInContext(self.context.current);
    LLVMConstReal(float_type, expr as f64)
  }

  #[inline]
  unsafe fn generate_char_expr(&mut self, expr: char) -> LLVMValueRef {
    let char_type = LLVMInt32TypeInContext(self.context.current);
    LLVMConstInt(char_type, expr as u64, 0)
  }

  #[inline]
  unsafe fn generate_binop_expr(
    &mut self,
    lhs: &Box<Expr>,
    op: &BinopKind,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    match op {
      BinopKind::Add => self.generate_binop_add(lhs, rhs),
      BinopKind::Sub => self.generate_binop_sub(lhs, rhs),
      BinopKind::Mul => self.generate_binop_mul(lhs, rhs),
      BinopKind::Div => self.generate_binop_div(lhs, rhs),
      _ => todo!(),
    }
  }

  #[inline]
  unsafe fn generate_binop_add(
    &mut self,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    let lhs = self.generate_expr(lhs);
    let rhs = self.generate_expr(rhs);

    let name = cstr!("addtmp");
    LLVMBuildAdd(self.context.builder, lhs, rhs, name)
  }

  #[inline]
  unsafe fn generate_binop_sub(
    &mut self,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    let lhs = self.generate_expr(lhs);
    let rhs = self.generate_expr(rhs);

    let name = cstr!("subtmp");
    LLVMBuildSub(self.context.builder, lhs, rhs, name)
  }

  #[inline]
  unsafe fn generate_binop_mul(
    &mut self,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    let lhs = self.generate_expr(lhs);
    let rhs = self.generate_expr(rhs);

    let name = cstr!("multmp");
    LLVMBuildMul(self.context.builder, lhs, rhs, name)
  }

  #[inline]
  unsafe fn generate_binop_div(
    &mut self,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    let lhs = self.generate_expr(lhs);
    let rhs = self.generate_expr(rhs);

    let name = cstr!("divtmp");
    LLVMBuildUDiv(self.context.builder, lhs, rhs, name)
  }

  #[inline]
  unsafe fn generate_unop_expr(
    &mut self,
    op: &UnopKind,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    match op {
      UnopKind::Neg => self.generate_neg_unop_expr(rhs),
      UnopKind::Not => self.generate_not_unop_expr(rhs),
    }
  }

  #[inline]
  unsafe fn generate_neg_unop_expr(&mut self, rhs: &Box<Expr>) -> LLVMValueRef {
    let value = self.generate_expr(rhs);
    LLVMBuildNeg(self.context.builder, value, cstr!("neg"))
  }

  #[inline]
  unsafe fn generate_not_unop_expr(&mut self, rhs: &Box<Expr>) -> LLVMValueRef {
    let value = self.generate_expr(rhs);
    LLVMBuildNot(self.context.builder, value, cstr!("not"))
  }

  #[inline]
  unsafe fn generate_ident_expr(&mut self, expr: &String) -> LLVMValueRef {
    todo!()
  }

  #[inline]
  unsafe fn generate_assign_expr(
    &mut self,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    todo!()
  }

  #[inline]
  unsafe fn generate_if_expr(
    &mut self,
    condition: &Box<Expr>,
    consequence: &Box<Block>,
    alternative: &Option<Box<Block>>,
  ) -> LLVMValueRef {
    let condition_value = self.generate_expr(condition);
    let int_type = LLVMInt64TypeInContext(self.context.current);
    let zero = LLVMConstInt(int_type, 0, 0);

    let name = cstr!("is_nonzero");
    let is_nonzero = LLVMBuildICmp(
      self.context.builder,
      LLVMIntNE,
      condition_value,
      zero,
      name,
    );

    let entry_name = cstr!("entry");

    let consequence_block = LLVMAppendBasicBlockInContext(
      self.context.current,
      self.main_function,
      entry_name,
    );

    let alternative_block = LLVMAppendBasicBlockInContext(
      self.context.current,
      self.main_function,
      entry_name,
    );

    let merge_block = LLVMAppendBasicBlockInContext(
      self.context.current,
      self.main_function,
      entry_name,
    );

    LLVMBuildCondBr(
      self.context.builder,
      is_nonzero,
      consequence_block,
      alternative_block,
    );

    LLVMPositionBuilderAtEnd(self.context.builder, consequence_block);

    let mut consequence_return = zero;

    for stmt in &consequence.stmts {
      consequence_return = self.generate_stmt(&stmt);
    }

    LLVMBuildBr(self.context.builder, merge_block);

    let consequence_block = LLVMGetInsertBlock(self.context.builder);

    LLVMPositionBuilderAtEnd(self.context.builder, alternative_block);

    let mut alternative_return = zero;

    for stmt in &alternative.as_ref().unwrap().stmts {
      alternative_return = self.generate_stmt(stmt);
    }

    LLVMBuildBr(self.context.builder, merge_block);

    let alternative_block = LLVMGetInsertBlock(self.context.builder);

    LLVMPositionBuilderAtEnd(self.context.builder, merge_block);

    let phi_name = cstr!("iftmp");
    let phi = LLVMBuildPhi(self.context.builder, int_type, phi_name);
    let mut values = vec![consequence_return, alternative_return];
    let mut blocks = vec![consequence_block, alternative_block];

    LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), 2);

    phi
  }

  #[inline]
  unsafe fn print_ir_to_file(&mut self) {
    let out_file = cstr!("out.ll");
    LLVMPrintModuleToFile(self.context.module, out_file, ptr::null_mut());
  }
}
