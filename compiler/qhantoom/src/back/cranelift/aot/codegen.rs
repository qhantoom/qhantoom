use super::translator::Translator;

use crate::back::context::ScopeMap;
use crate::front::parser::ast::Program;

use cranelift::prelude::{
  types, AbiParam, FunctionBuilder, FunctionBuilderContext, InstBuilder,
  Variable,
};

use cranelift_codegen::ir::GlobalValue;
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{settings, Context};
use cranelift_module::{default_libcall_names, DataContext, Linkage, Module};
use cranelift_native::builder;
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_preopt::optimize;

pub struct Codegen {
  ctx: Context,
  data_ctx: DataContext,
  module: ObjectModule,
  scope_map: ScopeMap<GlobalValue, Variable>,
}

impl Codegen {
  #[inline]
  pub fn new() -> Self {
    let flag_builder = settings::builder();
    let isa_builder = builder().unwrap();
    let isa = isa_builder.finish(Flags::new(flag_builder)).unwrap();

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
  pub fn generate(mut self, program: &Program) -> Vec<u8> {
    self.translate(program);

    let id = self
      .module
      .declare_function("main", Linkage::Export, &self.ctx.func.signature)
      .unwrap();

    self.module.define_function(id, &mut self.ctx).unwrap();
    self.module.clear_context(&mut self.ctx);

    let object = self.module.finish();
    let bytes = object.emit().unwrap();

    bytes
  }

  #[inline]
  fn translate(&mut self, program: &Program) {
    self
      .ctx
      .func
      .signature
      .returns
      .push(AbiParam::new(types::I64));

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
      ty: types::I64,
    };

    let return_value = translator.translate(program);

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();
    optimize(&mut self.ctx, self.module.isa()).unwrap();
    println!("\nir: {}", self.ctx.func.display().to_string());
  }
}
