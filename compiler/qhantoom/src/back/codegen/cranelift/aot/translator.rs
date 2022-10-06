use crate::back::codegen::cranelift::interface::{
  CompiledFunction, DataContextBuilder, VariableBuilder,
};

use crate::front::parser::ast::*;

use cranelift::prelude::{
  types, Block as CBlock, EntityRef, FunctionBuilder, InstBuilder, IntCC,
  Value, Variable,
};

use cranelift_codegen::ir::GlobalValue;
use cranelift_module::Module;
use cranelift_object::ObjectModule;

use std::collections::HashMap;

pub struct Translator<'a> {
  pub builder: FunctionBuilder<'a>,
  pub module: &'a mut ObjectModule,
  pub funs: &'a HashMap<String, CompiledFunction>,
  pub globals: &'a mut HashMap<String, GlobalValue>,
  pub vars: HashMap<String, Variable>,
  pub program: &'a Program,
  pub ty: types::Type,
  pub blocks: &'a mut Vec<CBlock>,
  pub variable_builder: &'a mut VariableBuilder,
  pub data_ctx_builder: &'a mut DataContextBuilder,
}

impl<'a> Translator<'a> {
  pub fn translate(&mut self, block: &Block) -> Result<Value, String> {
    let mut value = Value::new(0);

    for stmt in &block.stmts {
      value = self.translate_stmt(stmt);
    }

    Ok(value)
  }

  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match &stmt.kind {
      StmtKind::Decl(decl) => self.translate_stmt_decl(decl),
      StmtKind::Expr(expr) => self.translate_stmt_expr(expr),
      _ => panic!(),
    }
  }

  fn translate_stmt_decl(&mut self, decl: &Decl) -> Value {
    self.translate_decl(decl)
  }

  fn translate_decl(&mut self, decl: &Decl) -> Value {
    let value = self.translate_stmt_expr(&decl.value);

    let var =
      self
        .variable_builder
        .create_var(&mut self.builder, value, types::I64);

    self.vars.insert(decl.pattern.to_string(), var);

    value
  }

  fn translate_stmt_expr(&mut self, expr: &Expr) -> Value {
    self.translate_expr(expr)
  }

  fn translate_expr(&mut self, expr: &Expr) -> Value {
    match &expr.kind {
      ExprKind::Lit(lit) => self.translate_expr_lit(lit),
      ExprKind::Identifier(s) => self.translate_expr_id(s),
      ExprKind::Call(callee, args) => self.translate_expr_call(callee, args),
      ExprKind::UnOp(op, rhs) => self.translate_expr_un_op(op, rhs),
      ExprKind::BinOp(lhs, op, rhs) => self.translate_expr_bin_op(lhs, op, rhs),
      ExprKind::Assign(id, op, rhs) => self.translate_expr_assign(id, op, rhs),
      ExprKind::AssignOp(lhs, op, rhs) => {
        self.translate_expr_assign_op(lhs, op, rhs)
      }
      ExprKind::Loop(body) => self.translate_expr_loop(body),
      ExprKind::While(condition, body) => {
        self.translate_expr_while(condition, body)
      }
      ExprKind::Return(value) => self.translate_expr_return(value),
      ExprKind::Block(block) => self.translate_expr_block(block),
    }
  }

  fn translate_expr_lit(&mut self, lit: &Lit) -> Value {
    match &lit.kind {
      LitKind::Bool(boolean) => self.translate_expr_lit_bool(boolean),
      LitKind::Int(num) => self.translate_expr_lit_int(num),
      LitKind::Float(num) => self.translate_expr_lit_float(num),
      LitKind::Str(s) => self.translate_expr_lit_str(s),
    }
  }

  fn translate_expr_lit_bool(&mut self, boolean: &bool) -> Value {
    self.builder.ins().bconst(types::B1, *boolean)
  }

  fn translate_expr_lit_int(&mut self, num: &i64) -> Value {
    self.builder.ins().iconst(types::I64, *num)
  }

  fn translate_expr_lit_float(&mut self, num: &f64) -> Value {
    self.builder.ins().f64const(*num)
  }

  fn _translate_expr_char(&mut self, _ch: &char) -> Value {
    todo!()
  }

  fn translate_expr_lit_str(&mut self, string: &String) -> Value {
    self.data_ctx_builder.create_data(
      &mut self.builder,
      self.module,
      self.globals,
      string,
    )
  }

  fn translate_expr_id(&mut self, name: &String) -> Value {
    if let Some(decl) = self.vars.get(&name.to_string()) {
      return self.builder.use_var(*decl);
    }

    if let Some(_fun) = self.funs.get(&name.to_string()) {
      todo!();
    }

    panic!("translate expr id")
  }

  fn translate_expr_un_op(&mut self, op: &UnOp, rhs: &Expr) -> Value {
    let rhs = self.translate_stmt_expr(rhs);

    match &op.node {
      UnOpKind::Neg => self.translate_expr_un_op_neg(rhs),
      UnOpKind::Not => self.translate_expr_un_op_not(rhs),
    }
  }

  fn translate_expr_un_op_neg(&mut self, rhs: Value) -> Value {
    self.builder.ins().ineg(rhs)
  }

  fn translate_expr_un_op_not(&mut self, rhs: Value) -> Value {
    let value = self.builder.ins().icmp_imm(IntCC::Equal, rhs, 0);

    self.builder.ins().bint(self.ty, value)
  }

  fn translate_expr_bin_op(
    &mut self,
    lhs: &Expr,
    op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let lhs = self.translate_stmt_expr(lhs);
    let rhs = self.translate_stmt_expr(rhs);

    match &op.node {
      BinOpKind::Add => self.translate_expr_bin_op_add(lhs, rhs),
      BinOpKind::Sub => self.translate_expr_bin_op_sub(lhs, rhs),
      BinOpKind::Mul => self.translate_expr_bin_op_mul(lhs, rhs),
      BinOpKind::Div => self.translate_expr_bin_op_div(lhs, rhs),
      BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
      BinOpKind::Lt => self.translate_expr_bin_op_lt(lhs, rhs),
      BinOpKind::Gt => self.translate_expr_bin_op_gt(lhs, rhs),
      BinOpKind::Le => self.translate_expr_bin_op_le(lhs, rhs),
      BinOpKind::Ge => self.translate_expr_bin_op_ge(lhs, rhs),
      BinOpKind::Eq => self.translate_expr_bin_op_eq(lhs, rhs),
      BinOpKind::Ne => self.translate_expr_bin_op_ne(lhs, rhs),
      BinOpKind::Or => self.translate_expr_bin_op_or(lhs, rhs),
      BinOpKind::And => self.translate_expr_bin_op_and(lhs, rhs),
      BinOpKind::Shl => self.translate_expr_bin_op_shl(lhs, rhs),
      BinOpKind::Shr => self.translate_expr_bin_op_shr(lhs, rhs),
      BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
      BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
      BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
      _ => panic!("translate expr bin op"),
    }
  }

  fn translate_expr_bin_op_add(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().iadd(lhs, rhs)
  }

  fn translate_expr_bin_op_sub(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().isub(lhs, rhs)
  }

  fn translate_expr_bin_op_mul(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().imul(lhs, rhs)
  }

  fn translate_expr_bin_op_div(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().sdiv(lhs, rhs)
  }

  fn translate_expr_bin_op_rem(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_lt(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_gt(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_le(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ge(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_eq(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::Equal, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ne(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_or(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_and(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_shl(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().ishl(lhs, rhs)
  }

  fn translate_expr_bin_op_shr(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().sshr(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_and(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().band(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_xor(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().bxor(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_or(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().bor(lhs, rhs)
  }

  fn translate_expr_assign(
    &mut self,
    lhs: &Expr,
    _op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let rhs = self.translate_expr(rhs);
    let variable = self.vars.get(&lhs.to_string()).unwrap();

    self.builder.def_var(*variable, rhs);

    rhs
  }

  fn translate_expr_assign_op(
    &mut self,
    lhs: &Expr,
    op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let rhs = self.translate_stmt_expr(rhs);

    match &lhs.kind {
      ExprKind::Identifier(name) => {
        let var = *self.vars.get(&name.to_string()).unwrap();
        let lhs = self.translate_stmt_expr(lhs);

        let new_rhs = match &op.node {
          BinOpKind::Add => self.translate_expr_bin_op_add(lhs, rhs),
          BinOpKind::Sub => self.translate_expr_bin_op_sub(lhs, rhs),
          BinOpKind::Mul => self.translate_expr_bin_op_mul(lhs, rhs),
          BinOpKind::Div => self.translate_expr_bin_op_div(lhs, rhs),
          BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
          BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
          BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
          BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
          _ => panic!("binary operation not valid"),
        };

        self.builder.def_var(var, new_rhs);

        new_rhs
      }
      _ => unreachable!(),
    }
  }

  fn translate_expr_loop(&mut self, body: &Block) -> Value {
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(body_block, &[]);
    self.builder.switch_to_block(body_block);
    self.blocks.push(end_block);
    self.builder.switch_to_block(body_block);

    for stmt in &body.stmts {
      self.translate_stmt(stmt);
    }

    self.builder.ins().jump(body_block, &[]);
    self.blocks.pop();
    self.builder.seal_block(body_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  fn translate_expr_while(&mut self, condition: &Expr, body: &Block) -> Value {
    let header_block = self.builder.create_block();
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(header_block, &[]);
    self.builder.switch_to_block(header_block);

    let condition_value = self.translate_stmt_expr(condition);

    self.builder.ins().brz(condition_value, end_block, &[]);
    self.builder.ins().jump(body_block, &[]);
    self.blocks.push(end_block);
    self.builder.seal_block(body_block);
    self.builder.switch_to_block(body_block);

    for stmt in &body.stmts {
      self.translate_stmt(stmt);
    }

    self.builder.ins().jump(header_block, &[]);
    self.blocks.pop();
    self.builder.seal_block(header_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  fn translate_expr_return(
    &mut self,
    return_value: &Option<PBox<Expr>>,
  ) -> Value {
    let mut value = self.translate_expr_lit_int(&0);

    if let Some(e) = return_value {
      value = self.translate_stmt_expr(e);
      self.builder.ins().return_(&[value]);
    } else {
      self.builder.ins().return_(&[]);
    }

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);

    value
  }

  fn translate_expr_block(&mut self, block: &Block) -> Value {
    let mut value = self.translate_expr_lit_int(&0);

    for stmt in &block.stmts {
      value = self.translate_stmt(stmt);
    }

    value
  }

  fn translate_expr_call(
    &mut self,
    callee: &Expr,
    args: &Vec<PBox<Expr>>,
  ) -> Value {
    match self.funs.get(&callee.to_string()) {
      Some(func) => {
        if func.param_count != args.len() {
          // TODO: handle error
        }

        let local_func =
          self.module.declare_func_in_func(func.id, self.builder.func);

        let arguments = args
          .iter()
          .map(|arg| self.translate_expr(arg))
          .collect::<Vec<_>>();

        let call = self.builder.ins().call(local_func, &arguments);

        self.builder.inst_results(call)[0]
      }
      None => panic!("translate_expr_call error"),
    }
  }
}
