use crate::back::codegen::context::ScopeMap;

use crate::front::parser::ast::{
  BinopKind, Block, Expr, ExprKind, Fun, Local, Program, Prototype, Stmt,
  StmtKind, Struct, StructExpr, UnopKind,
};

use cranelift::prelude::{
  types, AbiParam, EntityRef, FunctionBuilder, InstBuilder, IntCC, Value,
  Variable,
};

use cranelift_jit::JITModule;
use cranelift_module::{DataContext, Linkage, Module};

pub struct Translator<'a> {
  pub builder: FunctionBuilder<'a>,
  pub data_ctx: &'a mut DataContext,
  pub module: &'a mut JITModule,
  pub ty: types::Type,
  pub index: usize,
  pub scope_map: &'a mut ScopeMap<Variable>,
}

impl<'a> Translator<'a> {
  #[inline]
  fn create_variable(&mut self, name: String) -> Variable {
    let var = Variable::new(self.index);

    self.scope_map.add_variable(name, var).unwrap();
    self.builder.declare_var(var, self.ty);

    self.index += 1;

    var
  }

  #[inline]
  pub fn translate(&mut self, program: &Program) -> Value {
    let mut value = Value::new(0);

    for stmt in &program.stmts {
      value = self.translate_stmt(stmt);
    }

    value
  }

  #[inline]
  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match stmt.kind() {
      StmtKind::Ext(ref prototype) => self.translate_ext(prototype),
      StmtKind::Fun(ref fun) => self.translate_fun(fun),
      StmtKind::Val(ref local) | StmtKind::Mut(ref local) => {
        self.translate_var(local)
      }
      StmtKind::Return(ref expr) => self.translate_return(expr),
      StmtKind::Break(ref expr) => self.translate_break(expr),
      StmtKind::Continue(ref expr) => self.translate_continue(expr),
      StmtKind::Struct(ref def) => self.translate_struct(def),
      StmtKind::Expr(ref expr) => self.translate_expr(expr),
    }
  }

  #[inline]
  fn translate_ext(&mut self, _prototype: &Prototype) -> Value {
    todo!()
  }

  #[inline]
  fn translate_fun(&mut self, _fun: &Box<Fun>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_var(&mut self, local: &Local) -> Value {
    let var = self.create_variable(local.name.to_string());
    let value = self.translate_expr(&local.value);

    self.builder.def_var(var, value);

    value
  }

  // tmp
  #[inline]
  fn translate_return(&mut self, expr: &Option<Box<Expr>>) -> Value {
    if let Some(ref e) = expr {
      return self.translate_expr(e);
    }

    self.builder.ins().iconst(types::I64, 0)
  }

  #[inline]
  fn translate_break(&mut self, expr: &Option<Box<Expr>>) -> Value {
    let mut value = self.builder.ins().iconst(self.ty, 0);
    let end_block = *self.scope_map.blocks().last().unwrap();

    if let Some(ref e) = expr {
      value = self.translate_expr(e);
      self.builder.ins().jump(end_block, &[value]);
    } else {
      self.builder.ins().jump(end_block, &[]);
    }

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);

    value
  }

  #[inline]
  fn translate_continue(&mut self, _expr: &Option<Box<Expr>>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_struct(&mut self, _def: &Box<Struct>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_expr(&mut self, expr: &Box<Expr>) -> Value {
    match expr.kind() {
      ExprKind::Bool(ref boolean) => self.translate_bool(boolean),
      ExprKind::Int(ref num) => self.translate_int(num),
      ExprKind::Float(ref num) => self.translate_float(num),
      ExprKind::Char(ref ch) => self.translate_char(ch),
      ExprKind::Str(ref buf) => self.translate_str(buf),
      ExprKind::Ident(ref name) => self.translate_ident(name),
      ExprKind::Binop {
        ref op,
        ref lhs,
        ref rhs,
      } => self.translate_binop(op, lhs, rhs),
      ExprKind::Unop { ref op, ref rhs } => self.translate_unop(op, rhs),
      ExprKind::Assign { ref lhs, ref rhs } => self.translate_assign(lhs, rhs),
      ExprKind::AssignOp {
        ref op,
        ref lhs,
        ref rhs,
      } => self.translate_assign_op(op, lhs, rhs),
      ExprKind::Array(ref exprs) => self.translate_array(exprs),
      ExprKind::Index { ref lhs, ref rhs } => self.translate_index(lhs, rhs),
      ExprKind::Closure(ref fun) => self.translate_closure(fun),
      ExprKind::Call {
        ref callee,
        ref args,
      } => self.translate_call(callee, args),
      ExprKind::If {
        ref condition,
        ref consequence,
        ref alternative,
      } => self.translate_if(condition, consequence, alternative),
      ExprKind::Loop { ref body } => self.translate_loop(body),
      ExprKind::While {
        ref condition,
        ref body,
      } => self.translate_while(condition, body),
      ExprKind::For {
        ref iterable,
        ref iterator,
        ref body,
      } => self.translate_for(iterable, iterator, body),
      ExprKind::Range {
        ref start,
        ref end,
        ref body,
      } => self.translate_range(start, end, body),
      ExprKind::StructExpr(ref def) => self.translate_struct_expr(def),
      ExprKind::FieldAccess { ref lhs, ref name } => {
        self.translate_field_access(lhs, name)
      }
    }
  }

  #[inline]
  fn translate_bool(&mut self, boolean: &bool) -> Value {
    self.builder.ins().bconst(types::B1, *boolean)
  }

  #[inline]
  fn translate_int(&mut self, num: &i64) -> Value {
    self.builder.ins().iconst(self.ty, *num)
  }

  // TODO: how to support floats?
  #[inline]
  fn translate_float(&mut self, num: &f64) -> Value {
    self.builder.ins().f64const(*num)
  }

  #[inline]
  fn translate_char(&mut self, _ch: &char) -> Value {
    todo!()
  }

  #[inline]
  fn translate_str(&mut self, _buf: &String) -> Value {
    todo!()
  }

  #[inline]
  fn translate_ident(&mut self, name: &str) -> Value {
    let var = self.scope_map.get_variable(&name).unwrap();

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
      BinopKind::Sub => self.translate_sub_binop(lhs, rhs),
      BinopKind::Mul => self.translate_mul_binop(lhs, rhs),
      BinopKind::Div => self.translate_div_binop(lhs, rhs),
      BinopKind::Rem => self.translate_rem_binop(lhs, rhs),
      BinopKind::Lt => self.translate_lt_binop(lhs, rhs),
      BinopKind::Gt => self.translate_gt_binop(lhs, rhs),
      BinopKind::Le => self.translate_le_binop(lhs, rhs),
      BinopKind::Ge => self.translate_ge_binop(lhs, rhs),
      BinopKind::Eq => self.translate_eq_binop(lhs, rhs),
      BinopKind::Ne => self.translate_ne_binop(lhs, rhs),
      BinopKind::Or => self.translate_or_binop(lhs, rhs),
      BinopKind::And => self.translate_and_binop(lhs, rhs),
      _ => unreachable!(),
    }
  }

  #[inline]
  fn translate_add_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().iadd(lhs, rhs)
  }

  #[inline]
  fn translate_sub_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().isub(lhs, rhs)
  }

  #[inline]
  fn translate_mul_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().imul(lhs, rhs)
  }

  #[inline]
  fn translate_div_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().sdiv(lhs, rhs)
  }

  #[inline]
  fn translate_rem_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  #[inline]
  fn translate_logical_binop(
    &mut self,
    op: &BinopKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let body_block = self.builder.create_block();
    let merge_block = self.builder.create_block();

    self.builder.append_block_param(merge_block, self.ty);

    match op {
      BinopKind::And => self.builder.ins().brnz(lhs, body_block, &[]),
      BinopKind::Or => self.builder.ins().brz(lhs, body_block, &[]),
      _ => unreachable!(),
    };

    self.builder.ins().jump(merge_block, &[lhs]);
    self.builder.seal_block(body_block);
    self.builder.switch_to_block(body_block);
    self.builder.ins().jump(merge_block, &[rhs]);
    self.builder.seal_block(merge_block);
    self.builder.switch_to_block(merge_block);
    self.builder.block_params(merge_block)[0]
  }

  #[inline]
  fn translate_and_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.translate_logical_binop(&BinopKind::And, lhs, rhs)
  }

  #[inline]
  fn translate_or_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.translate_logical_binop(&BinopKind::Or, lhs, rhs)
  }

  #[inline]
  fn translate_lt_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_gt_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_le_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_ge_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_eq_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::Equal, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_ne_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  #[inline]
  fn translate_unop(&mut self, op: &UnopKind, rhs: &Box<Expr>) -> Value {
    let rhs = self.translate_expr(rhs);

    match op {
      UnopKind::Neg => self.builder.ins().ineg(rhs),
      _ => unimplemented!(),
    }
  }

  #[inline]
  fn translate_array(&mut self, _exprs: &Vec<Box<Expr>>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_index(&mut self, _lhs: &Box<Expr>, _rhs: &Box<Expr>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_assign(&mut self, lhs: &Box<Expr>, rhs: &Box<Expr>) -> Value {
    let rhs = self.translate_expr(rhs);

    match lhs.kind() {
      ExprKind::Ident(ref name) => {
        let var = self.scope_map.get_variable(name).unwrap();

        self.builder.def_var(*var, rhs);
      }
      _ => unreachable!(),
    }

    rhs
  }

  #[inline]
  fn translate_assign_op(
    &mut self,
    op: &BinopKind,
    lhs: &Box<Expr>,
    rhs: &Box<Expr>,
  ) -> Value {
    let rhs = self.translate_expr(rhs);

    match lhs.kind() {
      ExprKind::Ident(ref name) => {
        let var = *self.scope_map.get_variable(name).unwrap();
        let lhs = self.translate_expr(lhs);

        let new_rhs = match op {
          BinopKind::AddOp => self.translate_add_binop(lhs, rhs),
          BinopKind::SubOp => self.translate_sub_binop(lhs, rhs),
          BinopKind::MulOp => self.translate_mul_binop(lhs, rhs),
          BinopKind::DivOp => self.translate_div_binop(lhs, rhs),
          BinopKind::RemOp => self.translate_rem_binop(lhs, rhs),
          _ => unreachable!(),
        };

        self.builder.def_var(var, new_rhs);

        new_rhs
      }
      _ => unreachable!(),
    }
  }

  #[inline]
  fn translate_closure(&mut self, _fun: &Box<Fun>) -> Value {
    todo!()
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
  fn translate_if(
    &mut self,
    condition: &Box<Expr>,
    consequence: &Box<Block>,
    alternative: &Option<Box<Block>>,
  ) -> Value {
    let then_block = self.builder.create_block();
    let else_block = self.builder.create_block();
    let merge_block = self.builder.create_block();

    self.builder.append_block_param(merge_block, self.ty);

    let condition_value = self.translate_expr(condition);

    self.builder.ins().brz(condition_value, else_block, &[]);
    self.builder.ins().jump(then_block, &[]);
    self.builder.seal_block(else_block);
    self.builder.seal_block(then_block);
    self.builder.switch_to_block(then_block);

    let mut value = self.builder.ins().iconst(self.ty, 0);

    for stmt in &consequence.stmts {
      value = self.translate_stmt(stmt);
    }

    self.builder.ins().jump(merge_block, &[value]);
    self.builder.switch_to_block(else_block);

    let mut value = self.builder.ins().iconst(self.ty, 0);

    if let Some(alternative) = alternative {
      for stmt in &alternative.stmts {
        value = self.translate_stmt(stmt);
      }
    }

    self.builder.ins().jump(merge_block, &[value]);
    self.builder.seal_block(merge_block);
    self.builder.switch_to_block(merge_block);
    self.builder.block_params(merge_block)[0]
  }

  #[inline]
  fn translate_loop(&mut self, body: &Box<Block>) -> Value {
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(body_block, &[]);
    self.builder.switch_to_block(body_block);
    self.scope_map.blocks().push(end_block);
    self.builder.switch_to_block(body_block);

    for stmt in &body.stmts {
      self.translate_stmt(stmt);
    }

    self.builder.ins().jump(body_block, &[]);
    self.scope_map.blocks().pop();
    self.builder.seal_block(body_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  #[inline]
  fn translate_while(
    &mut self,
    condition: &Box<Expr>,
    body: &Box<Block>,
  ) -> Value {
    let header_block = self.builder.create_block();
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(header_block, &[]);
    self.builder.switch_to_block(header_block);

    let cond = self.translate_expr(condition);

    self.builder.ins().brz(cond, end_block, &[]);
    self.builder.ins().jump(body_block, &[]);
    self.scope_map.blocks().push(end_block);
    self.builder.seal_block(body_block);
    self.builder.switch_to_block(body_block);

    for stmt in &body.stmts {
      self.translate_stmt(stmt);
    }

    self.builder.ins().jump(header_block, &[]);
    self.scope_map.blocks().pop();
    self.builder.seal_block(header_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  #[inline]
  fn translate_for(
    &mut self,
    _iterable: &Box<Expr>,
    _iterator: &Box<Expr>,
    _body: &Box<Block>,
  ) -> Value {
    todo!()
  }

  #[inline]
  fn translate_range(
    &mut self,
    _start: &Box<Expr>,
    _end: &Box<Expr>,
    _body: &Box<Block>,
  ) -> Value {
    todo!()
  }

  #[inline]
  fn translate_struct_expr(&mut self, _struct_expr: &Box<StructExpr>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_field_access(
    &mut self,
    _lhs: &Box<Expr>,
    _name: &Box<Expr>,
  ) -> Value {
    todo!()
  }
}
