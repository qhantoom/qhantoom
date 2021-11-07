use std::mem;

use crate::front::parser;

use crate::front::parser::ast::{
  BinopKind, Expr, ExprKind, Fun, Program, Prototype, Stmt, StmtKind,
};

use crate::back::codegen::context::{print_builtin, ScopeMap};
use crate::util::error::{Error, Result};
use crate::util::symbol::Symbol;

use cranelift::prelude::{
  types, AbiParam, EntityRef, FunctionBuilder, FunctionBuilderContext,
  InstBuilder, Value, Variable,
};

use cranelift_codegen::binemit::{NullStackMapSink, NullTrapSink};
use cranelift_codegen::ir::GlobalValue;
use cranelift_codegen::Context;
use cranelift_jit::{JITBuilder, JITModule};

use cranelift_module::{
  default_libcall_names, /* DataContext, */ FuncId, Linkage, Module,
};

#[inline]
pub fn codegen(codegen: &mut Codegen, src: &str) -> Result<i64> {
  let ast = parser::parse(src)?;
  let code_fn = codegen.codegen(&ast)?;

  match code_fn {
    Some(f) => Ok(f()),
    None => Err(Error::Custom("codegen error function")),
  }
}

pub struct Codegen {
  module: JITModule,
  builder_ctx: FunctionBuilderContext,
  ctx: Context,
  scope_map: ScopeMap<GlobalValue, Variable>,
}

impl Codegen {
  #[inline]
  pub fn new() -> Self {
    let mut builder = JITBuilder::new(default_libcall_names());
    let print_addr = print_builtin as *const u8;

    builder.symbol("print", print_addr);

    let module = JITModule::new(builder);
    let ctx = module.make_context();

    Self {
      module,
      builder_ctx: FunctionBuilderContext::new(),
      ctx,
      scope_map: ScopeMap::new(),
    }
  }

  #[inline]
  pub fn codegen(&mut self, program: &Program) -> Result<Option<fn() -> i64>> {
    let mut code = None;

    for fun in &program.stmts {
      code = Some(self.define_fun(fun)?);
    }

    Ok(code)
  }

  #[inline]
  fn define_fun(&mut self, stmt: &Stmt) -> Result<fn() -> i64> {
    match stmt.kind() {
      StmtKind::Fun(ref fun) => {
        let func_id = self.declare_function(&fun.prototype, Linkage::Export);

        let builder =
          FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        let mut translator = Translator {
          module: &mut self.module,
          builder,
          scope_map: &mut self.scope_map,
          ty: types::I64,
          index: 0,
        };

        translator.translate_fun(&fun);
        println!("\n{}", self.ctx.func.display(None).to_string());

        let mut trap_sink = NullTrapSink {};
        let mut stack_map_sink = NullStackMapSink {};

        self
          .module
          .define_function(
            func_id,
            &mut self.ctx,
            &mut trap_sink,
            &mut stack_map_sink,
          )
          .unwrap();

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();

        unsafe {
          Ok(mem::transmute::<_, fn() -> i64>(
            self.module.get_finalized_function(func_id),
          ))
        }
      }
      _ => unreachable!(),
    }
  }

  #[inline]
  fn declare_function(
    &mut self,
    prototype: &Prototype,
    linkage: Linkage,
  ) -> FuncId {
    for _arg in &prototype.args {
      self
        .ctx
        .func
        .signature
        .params
        .push(AbiParam::new(types::I64));
    }

    self
      .ctx
      .func
      .signature
      .returns
      .push(AbiParam::new(types::I64));

    let fun_id = self
      .module
      .declare_function(
        &prototype.name.to_string(),
        linkage,
        &self.ctx.func.signature,
      )
      .unwrap();

    fun_id
  }
}

pub struct Translator<'a> {
  pub module: &'a mut JITModule,
  pub scope_map: &'a mut ScopeMap<GlobalValue, Variable>,
  builder: FunctionBuilder<'a>,
  pub ty: types::Type,
  index: usize,
}

impl<'a> Translator<'a> {
  #[inline]
  fn translate_fun(&mut self, fun: &Fun) {
    let entry_block = self.builder.create_block();

    self
      .builder
      .append_block_params_for_function_params(entry_block);

    self.builder.seal_block(entry_block);
    self.builder.switch_to_block(entry_block);

    for (i, arg) in fun.prototype.args.iter().enumerate() {
      let value = self.builder.block_params(entry_block)[i];
      let var = self.mk_variable(arg.name.to_string());

      self.builder.def_var(var, value);
    }

    let mut value = self.builder.ins().iconst(self.ty, 0);

    for stmt in &fun.body.stmts {
      value = self.translate_stmt(stmt);
    }

    self.builder.ins().return_(&[value]);
    self.builder.finalize();
  }

  #[inline]
  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.translate_expr(expr),
      _ => unreachable!(),
    }
  }

  #[inline]
  fn translate_expr(&mut self, expr: &Expr) -> Value {
    match expr.kind() {
      ExprKind::Int(ref num) => self.translate_int(num),
      ExprKind::Ident(ref name) => self.translate_ident(name),
      ExprKind::Binop {
        ref op,
        ref lhs,
        ref rhs,
      } => self.translate_binop(op, lhs, rhs),
      ExprKind::Call {
        ref callee,
        ref args,
      } => self.translate_call(callee, args),
      _ => unreachable!(),
    }
  }

  #[inline]
  fn translate_int(&mut self, num: &i64) -> Value {
    self.builder.ins().iconst(self.ty, *num)
  }

  #[inline]
  fn translate_call(
    &mut self,
    callee: &Box<Expr>,
    args: &Vec<Box<Expr>>,
  ) -> Value {
    let mut sig = self.module.make_signature();

    for _arg in args {
      sig.params.push(AbiParam::new(self.ty));
    }

    sig.returns.push(AbiParam::new(self.ty));

    let callee = self
      .module
      .declare_function(&callee.to_string(), Linkage::Import, &sig)
      .expect("function been declared");

    let local_callee = self
      .module
      .declare_func_in_func(callee, &mut self.builder.func);

    let mut arg_values = Vec::new();

    for arg in args {
      arg_values.push(self.translate_expr(arg))
    }

    let call = self.builder.ins().call(local_callee, &arg_values);
    self.builder.inst_results(call)[0]
  }

  #[inline]
  fn translate_ident(&mut self, name: &Symbol) -> Value {
    let var = self.scope_map.get_variable(&name.to_string()).unwrap();

    self.builder.use_var(*var)
  }

  #[inline]
  fn translate_binop(
    &mut self,
    op: &BinopKind,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> Value {
    let lhs = self.translate_expr(lhs);
    let rhs = self.translate_expr(rhs);

    match op {
      BinopKind::Add => self.translate_add_binop(lhs, rhs),
      _ => unreachable!(),
    }
  }

  #[inline]
  fn translate_add_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().iadd(lhs, rhs)
  }

  #[inline]
  fn mk_variable(&mut self, name: String) -> Variable {
    let var = Variable::new(self.index);

    self.index += 1;

    self.scope_map.add_variable(name, var).unwrap();
    self.builder.declare_var(var, self.ty);

    var
  }
}
