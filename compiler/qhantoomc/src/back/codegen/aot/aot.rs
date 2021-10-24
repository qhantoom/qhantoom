use super::translator::Translator;

use crate::back::codegen::context::ScopeMap;
use crate::front::parser::ast::Program;
use crate::util::error::Result;

use cranelift_codegen::binemit::{NullStackMapSink, NullTrapSink};
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{settings, Context};
use cranelift_module::{default_libcall_names, DataContext, Linkage, Module};
use cranelift_native::builder;
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_preopt::optimize;

use cranelift::prelude::{
  types, AbiParam, FunctionBuilder, FunctionBuilderContext, InstBuilder,
  Variable,
};

#[inline]
pub fn generate(ast: &Program) -> Result<Vec<u8>> {
  let aot = Aot::new();

  aot.generate(ast)
}

pub struct Aot {
  ctx: Context,
  data_ctx: DataContext,
  module: ObjectModule,
  scope_map: ScopeMap<Variable>,
}

impl Aot {
  #[inline]
  pub fn new() -> Self {
    let flag_builder = settings::builder();
    let isa_builder = builder().unwrap();
    let isa = isa_builder.finish(Flags::new(flag_builder));

    let object_builder =
      ObjectBuilder::new(isa, "qhantoom".to_string(), default_libcall_names())
        .unwrap();

    let module = ObjectModule::new(object_builder);

    Self {
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module: module,
      scope_map: ScopeMap::new(),
    }
  }

  #[inline]
  pub fn generate(mut self, program: &Program) -> Result<Vec<u8>> {
    self.translate(program)?;

    let id = self.module.declare_function(
      "main",
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

    let object = self.module.finish();
    let bytes = object.emit()?;

    Ok(bytes)
  }

  #[inline]
  fn translate(&mut self, program: &Program) -> Result<()> {
    self
      .ctx
      .func
      .signature
      .returns
      .push(AbiParam::new(types::F64));

    let mut builder_context = FunctionBuilderContext::new();

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut builder_context);

    let entry_block = builder.create_block();

    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let mut translator = Translator {
      builder,
      data_ctx: &mut self.data_ctx,
      index: 0,
      module: &mut self.module,
      scope_map: &mut self.scope_map,
      ty: types::F64,
    };

    let return_value = translator.translate(program);

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();
    optimize(&mut self.ctx, self.module.isa())?;
    println!("\nir: {}", self.ctx.func.display(None).to_string());

    Ok(())
  }
}