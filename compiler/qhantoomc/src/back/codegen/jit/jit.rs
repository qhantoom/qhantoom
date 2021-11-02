use std::mem;

use super::translator::Translator;

use crate::back::codegen::context::{
  print_builtin, print_str_builtin, ScopeMap,
};

use crate::front::parser;
use crate::front::parser::ast::Program;
use crate::util::error::Result;

use cranelift::prelude::{
  types, AbiParam, FunctionBuilder, FunctionBuilderContext, InstBuilder,
  Variable,
};

use cranelift_codegen::binemit::{NullStackMapSink, NullTrapSink};
use cranelift_codegen::ir::GlobalValue;
use cranelift_codegen::Context;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, DataContext, Linkage, Module};
use cranelift_preopt::optimize;

#[inline]
pub fn compile<O>(jit: &mut Jit, src: &str) -> Result<O> {
  let ast = parser::parse(src)?;
  let code_ptr = jit.compile(&ast)?;
  let code_fn = unsafe { mem::transmute::<_, fn() -> O>(code_ptr) };

  Ok(code_fn())
}

pub struct Jit {
  builder_context: FunctionBuilderContext,
  ctx: Context,
  data_ctx: DataContext,
  index: usize,
  module: JITModule,
  scope_map: ScopeMap<GlobalValue, Variable>,
}

impl Jit {
  #[inline]
  pub fn new() -> Self {
    let mut builder = JITBuilder::new(default_libcall_names());

    let print_addr = print_builtin as *const u8;
    let print_str_addr = print_str_builtin as *const u8;

    builder.symbol("print", print_addr);
    builder.symbol("print_str", print_str_addr);

    let module = JITModule::new(builder);

    Self {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      index: 0,
      module,
      scope_map: ScopeMap::new(),
    }
  }

  #[inline]
  pub fn compile(&mut self, program: &Program) -> Result<*const u8> {
    self.translate(program)?;
    self.index += 1;

    let id = self.module.declare_function(
      &format!("_main:{}", self.index),
      Linkage::Export,
      &self.ctx.func.signature,
    )?;

    let mut trap_sink = NullTrapSink {};
    let mut stack_map_sink = NullStackMapSink {};

    self.module.define_function(
      id,
      &mut self.ctx,
      &mut trap_sink,
      &mut stack_map_sink,
    )?;

    self.module.clear_context(&mut self.ctx);
    self.module.finalize_definitions();

    let code = self.module.get_finalized_function(id);

    Ok(code)
  }

  #[inline]
  fn translate(&mut self, program: &Program) -> Result<()> {
    self
      .ctx
      .func
      .signature
      .returns
      .push(AbiParam::new(types::I64));

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

    let entry_block = builder.create_block();

    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let mut translator = Translator {
      builder,
      data_ctx: &mut self.data_ctx,
      index: 0,
      module: &mut self.module,
      scope_map: &mut self.scope_map,
      ty: types::I64,
    };

    let return_value = translator.translate(program);

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();
    optimize(&mut self.ctx, self.module.isa())?;

    Ok(())
  }
}
