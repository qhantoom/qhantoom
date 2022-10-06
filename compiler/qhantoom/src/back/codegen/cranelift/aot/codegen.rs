use super::translator::Translator;

use crate::back::codegen::cranelift::interface::{
  CompiledFunction, DataContextBuilder, VariableBuilder,
};

use crate::front::parser::ast::{
  Ext, Fun, Item, ItemKind, Program, Prototype, ReturnTy, Ty,
};

use crate::util::constant::{PATH_DIRECTORY, PROGRAM_ENTRY};
use crate::util::pack;

use codegen::ir::{FuncRef, GlobalValue};
use cranelift::prelude::{Block as CBlock, *};
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{settings, Context};
use cranelift_module::default_libcall_names;
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_native::builder;
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_preopt::optimize;

use std::collections::HashMap;

pub type BuildResult = Result<Box<dyn FnOnce()>, String>;

pub fn generate(program: &Program) -> Codegen<'_> {
  Codegen::new(program).generate()
}

pub struct Codegen<'a> {
  builder_context: FunctionBuilderContext,
  module: ObjectModule,
  program: &'a Program,
  blocks: Vec<CBlock>,
  ctx: Context,
  ir: String,
  funs: HashMap<String, CompiledFunction>,
  globals: HashMap<String, GlobalValue>,
  data_ctx_builder: DataContextBuilder,
  variable_builder: VariableBuilder,
}

impl<'a> Codegen<'a> {
  fn new(program: &'a Program) -> Self {
    let mut flag_builder = settings::builder();

    flag_builder
      .set("opt_level", "speed_and_size")
      .expect("set optlevel");

    let isa_builder = builder().unwrap();
    let isa = isa_builder.finish(Flags::new(flag_builder)).unwrap();

    let object_builder =
      ObjectBuilder::new(isa, "qhantoom".to_string(), default_libcall_names())
        .unwrap();

    let module = ObjectModule::new(object_builder);

    Self {
      ctx: module.make_context(),
      builder_context: FunctionBuilderContext::new(),
      module,
      program,
      blocks: vec![],
      ir: String::new(),
      funs: HashMap::new(),
      globals: HashMap::new(),
      data_ctx_builder: DataContextBuilder::default(),
      variable_builder: VariableBuilder::default(),
    }
  }

  fn generate(mut self) -> Self {
    for item in &self.program.items {
      self.generate_item(item);
    }

    self
  }

  fn generate_item(&mut self, item: &Item) {
    match &item.kind {
      ItemKind::Fun(fun) => self.generate_item_fun(fun),
      ItemKind::Ext(ext) => self.generate_item_ext(ext, Linkage::Import),
      _ => panic!("generate item"),
    }
  }

  fn generate_item_fun(&mut self, fun: &Fun) {
    self.generate_fun(fun);
  }

  fn generate_fun(&mut self, fun: &Fun) {
    let signature = &mut self.ctx.func.signature;
    let params = &fun.prototype.inputs;

    for _param in params {
      signature.params.push(AbiParam::new(types::I64));
    }

    signature.returns.push(AbiParam::new(types::I64));

    let func_name = fun.prototype.name.to_string();

    let func_id = self
      .generate_prototype(&fun.prototype, Linkage::Export)
      .unwrap();

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

    make_c_fun(
      "puts",
      &[Box::new(Ty::STR)],
      &mut self.module,
      &mut builder,
      &mut self.funs,
    );

    let entry_block = builder.create_block();
    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let mut vars = HashMap::new();

    for (i, input) in params.iter().enumerate() {
      let val = builder.block_params(entry_block)[i];

      let variable =
        self
          .variable_builder
          .create_var(&mut builder, val, types::I64);

      vars.insert(input.pattern.to_string(), variable);
    }

    if let Some(ref mut func) = self.funs.get_mut(&func_name) {
      func.defined = true;
    }

    let mut translator = Translator {
      builder,
      module: &mut self.module,
      funs: &self.funs,
      globals: &mut self.globals,
      vars,
      program: self.program,
      ty: types::I64,
      blocks: &mut self.blocks,
      data_ctx_builder: &mut self.data_ctx_builder,
      variable_builder: &mut self.variable_builder,
    };

    let return_value = match translator.translate(&fun.body) {
      Ok(value) => value,
      Err(_e) => {
        translator.builder.finalize();
        self.funs.remove(&func_name);
        return; // TODO: error
      }
    };

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();

    optimize(&mut self.ctx, self.module.isa()).unwrap();

    self.ir = self.ctx.func.display().to_string();

    self.module.define_function(func_id, &mut self.ctx).unwrap();
    self.module.clear_context(&mut self.ctx);
  }

  fn generate_item_ext(&mut self, ext: &Ext, linkage: Linkage) {
    match self.generate_prototype(&ext.prototype, linkage) {
      Ok(_func_id) => {}
      Err(_error) => {}
    }
  }

  fn generate_prototype(
    &mut self,
    prototype: &Prototype,
    linkage: Linkage,
  ) -> Result<FuncId, String> {
    let func_name = &prototype.name.to_string();
    let params = &prototype.inputs;

    match self.funs.get(func_name) {
      Some(func) => {
        if func.defined {
          return Err(format!("Redefinition of function: {}", func_name));
        }

        if func.param_count != params.len() {
          return Err(format!(
            "`{}`: redefinition of function's parameters different, {}(before) vs {}(after)",
            func_name,
            func.param_count,
            params.len()
          ));
        }

        Ok(func.id)
      }
      None => {
        let mut signature = self.module.make_signature();

        for _param in params.iter() {
          signature.params.push(AbiParam::new(types::I64));
        }

        if let ReturnTy::Ty(_ty) = &prototype.output {
          signature.returns.push(AbiParam::new(types::I64));
        } else {
          signature.returns.push(AbiParam::new(types::I64));
        }

        let id =
          match self.module.declare_function(func_name, linkage, &signature) {
            Ok(id) => id,
            Err(e) => return Err(format!("{e}")),
          };

        self.funs.insert(
          func_name.to_string(),
          CompiledFunction::new(id, false, params.len()),
        );

        Ok(id)
      }
    }
  }

  pub fn build(self, output_ir: bool) -> BuildResult {
    let object = self.module.finish();
    let bytes = object.emit().unwrap();

    Ok(Box::new(move || {
      let path_object_file = format!("{PATH_DIRECTORY}/{PROGRAM_ENTRY}.o");
      let path_exe_file = format!("{PATH_DIRECTORY}/{PROGRAM_ENTRY}");

      pack::make_dir(PATH_DIRECTORY);
      pack::make_file(&path_object_file, &bytes);
      pack::make_exe(&path_object_file, &path_exe_file);

      if output_ir {
        println!("\n{}", self.ir);
      }
    }))
  }
}

fn make_c_fun(
  name: &str,
  args: &[Box<Ty>],
  module: &mut ObjectModule,
  builder: &mut FunctionBuilder,
  funs: &mut HashMap<String, CompiledFunction>,
) -> FuncRef {
  let mut signature = module.make_signature();

  for _arg in args {
    signature.params.push(AbiParam::new(types::I64));
  }

  signature.returns.push(AbiParam::new(types::I64));

  let id = module
    .declare_function(name, Linkage::Export, &signature)
    .unwrap();

  funs.insert(
    name.to_string(),
    CompiledFunction {
      defined: false,
      id,
      param_count: 1,
    },
  );

  module.declare_func_in_func(id, builder.func)
}
