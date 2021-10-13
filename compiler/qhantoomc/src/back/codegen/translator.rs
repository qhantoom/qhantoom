use crate::front::parser::ast::{
  BinopKind, Expr, ExprKind, Program, Stmt, StmtKind,
};

use cranelift::prelude::{
  types, EntityRef, FloatCC, FunctionBuilder, InstBuilder, Value,
};

pub struct Translator<'a, T> {
  pub builder: FunctionBuilder<'a>,
  pub module: &'a mut T,
  pub ty: types::Type,
}

impl<'a, T> Translator<'a, T> {
  #[inline]
  pub fn translate(&mut self, program: &Program) -> Value {
    let mut return_value = Value::new(0);

    for stmt in &program.stmts {
      return_value = self.translate_stmt(stmt);
    }

    return_value
  }

  #[inline]
  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match stmt.kind() {
      StmtKind::Expr(expr) => self.translate_expr(expr),
      _ => unimplemented!(),
    }
  }

  #[inline]
  fn translate_expr(&mut self, expr: &Expr) -> Value {
    match expr.kind() {
      ExprKind::Int(ref num) => self.translate_int(num),
      ExprKind::Float(ref num) => self.translate_float(num),
      ExprKind::Binop {
        ref op,
        ref lhs,
        ref rhs,
      } => self.translate_binop(op, lhs, rhs),
      _ => unimplemented!(),
    }
  }

  #[inline]
  fn translate_int(&mut self, num: &i64) -> Value {
    self.builder.ins().f64const(*num as f64)
  }

  #[inline]
  fn translate_float(&mut self, num: &f64) -> Value {
    self.builder.ins().f64const(*num)
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
      BinopKind::And => self.translate_and_binop(lhs, rhs),
      BinopKind::Or => self.translate_or_binop(lhs, rhs),
      BinopKind::Lt => self.translate_lt_binop(lhs, rhs),
      BinopKind::Gt => self.translate_gt_binop(lhs, rhs),
      BinopKind::Le => self.translate_le_binop(lhs, rhs),
      BinopKind::Ge => self.translate_ge_binop(lhs, rhs),
      BinopKind::Eq => self.translate_eq_binop(lhs, rhs),
      BinopKind::Ne => self.translate_ne_binop(lhs, rhs),
      _ => unimplemented!(),
    }
  }

  #[inline]
  fn translate_add_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().fadd(lhs, rhs)
  }

  #[inline]
  fn translate_sub_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().fsub(lhs, rhs)
  }

  #[inline]
  fn translate_mul_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().fmul(lhs, rhs)
  }

  #[inline]
  fn translate_div_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().fdiv(lhs, rhs)
  }

  #[inline]
  fn translate_and_binop(&mut self, _lhs: Value, _rhs: Value) -> Value {
    unimplemented!()
  }

  #[inline]
  fn translate_or_binop(&mut self, _lhs: Value, _rhs: Value) -> Value {
    unimplemented!()
  }

  #[inline]
  fn translate_lt_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().fcmp(FloatCC::LessThan, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }

  #[inline]
  fn translate_gt_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }

  #[inline]
  fn translate_le_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }

  #[inline]
  fn translate_ge_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }

  #[inline]
  fn translate_eq_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().fcmp(FloatCC::Equal, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }

  #[inline]
  fn translate_ne_binop(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs);
    let int = self.builder.ins().bint(types::I32, boolean);
    self.builder.ins().fcvt_from_sint(types::F64, int)
  }
}
