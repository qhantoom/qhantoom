use crate::front::parser::ast::{Ty, TyKind};

use codegen::ir::GlobalValue;
use cranelift::prelude::*;
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use cranelift_object::ObjectModule;

use core::panic;
use std::collections::HashMap;

pub struct CompiledFunction {
  pub id: FuncId,
  pub defined: bool,
  pub param_count: usize,
}

impl CompiledFunction {
  pub fn new(id: FuncId, defined: bool, param_count: usize) -> Self {
    Self {
      id,
      defined,
      param_count,
    }
  }
}

#[derive(Default)]
pub struct VariableBuilder {
  pub index: u32,
}

impl VariableBuilder {
  pub fn create_var(
    &mut self,
    builder: &mut FunctionBuilder,
    value: Value,
    ty: types::Type,
  ) -> Variable {
    let variable = Variable::with_u32(self.index);

    builder.declare_var(variable, ty);

    self.index += 1;

    builder.def_var(variable, value);

    variable
  }
}

#[derive(Default)]
pub struct DataContextBuilder {
  pub index: u32,
}

impl DataContextBuilder {
  pub fn create_data(
    &mut self,
    builder: &mut FunctionBuilder,
    module: &mut ObjectModule,
    globals: &mut HashMap<String, GlobalValue>,
    data: &String,
  ) -> Value {
    let data_id = if let Some(data_id) = globals.get(data) {
      *data_id
    } else {
      match module.declare_data(
        &format!("_data{}", self.index),
        Linkage::Local,
        true,
        false,
      ) {
        Ok(id) => {
          let mut data_ctx = DataContext::new();

          data_ctx.define(data.as_bytes().to_vec().into_boxed_slice());
          module.define_data(id, &data_ctx).unwrap();

          let data_id = module.declare_data_in_func(id, builder.func);

          data_ctx.clear();
          globals.insert(data.to_string(), data_id);

          self.index += 1;

          data_id
        }
        Err(_err) => {
          panic!("_data{} already used/declared", self.index)
        }
      }
    };

    builder.ins().symbol_value(types::I64, data_id)
  }
}

impl From<&Box<Ty>> for types::Type {
  fn from(ty: &Box<Ty>) -> Self {
    match ty.kind {
      TyKind::Bool => types::B1,
      TyKind::S8 => types::I8,
      TyKind::S16 => types::I16,
      TyKind::S32 => types::I32,
      TyKind::S64 => types::I64,
      TyKind::U8 => types::B8,
      TyKind::U16 => types::B16,
      TyKind::U32 => types::B32,
      TyKind::U64 => types::B64,
      TyKind::F32 => types::F32,
      TyKind::F64 => types::F64,
      _ => panic!("fom ty to types"),
    }
  }
}
