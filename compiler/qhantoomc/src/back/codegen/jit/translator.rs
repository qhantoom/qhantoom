use crate::back::codegen::context::ScopeMap;

use crate::front::parser::ast::{
  BinopKind, Block, Expr, ExprKind, Fun, Local, Program, Prototype, Stmt,
  StmtKind, UnopKind,
};

use cranelift::prelude::{
  types, /* AbiParam, */ EntityRef, FloatCC, FunctionBuilder, InstBuilder,
  Value, Variable,
};

use cranelift_jit::JITModule;
use cranelift_module::DataContext;

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

  #[inline]
  fn translate_return(&mut self, _expr: &Box<Expr>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_break(&mut self, _expr: &Option<Box<Expr>>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_continue(&mut self, _expr: &Option<Box<Expr>>) -> Value {
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
    }
  }

  #[inline]
  fn translate_bool(&mut self, _boolean: &bool) -> Value {
    todo!()
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
      BinopKind::And => self.translate_and_binop(lhs, rhs),
      BinopKind::Or => self.translate_or_binop(lhs, rhs),
      BinopKind::Lt => self.translate_lt_binop(lhs, rhs),
      BinopKind::Gt => self.translate_gt_binop(lhs, rhs),
      BinopKind::Le => self.translate_le_binop(lhs, rhs),
      BinopKind::Ge => self.translate_ge_binop(lhs, rhs),
      BinopKind::Eq => self.translate_eq_binop(lhs, rhs),
      BinopKind::Ne => self.translate_ne_binop(lhs, rhs),
      _ => todo!(),
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
    todo!()
  }

  #[inline]
  fn translate_or_binop(&mut self, _lhs: Value, _rhs: Value) -> Value {
    todo!()
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

  #[inline]
  fn translate_unop(&mut self, op: &UnopKind, rhs: &Box<Expr>) -> Value {
    let rhs = self.translate_expr(rhs);

    match op {
      UnopKind::Neg => self.builder.ins().fneg(rhs),
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
  fn translate_closure(&mut self, _fun: &Box<Fun>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_call(
    &mut self,
    _callee: &Box<Expr>,
    _args: &Vec<Box<Expr>>,
  ) -> Value {
    todo!()
  }

  #[inline]
  fn translate_if(
    &mut self,
    _condition: &Box<Expr>,
    _consequence: &Box<Block>,
    _alternative: &Option<Box<Block>>,
  ) -> Value {
    todo!()
  }

  #[inline]
  fn translate_loop(&mut self, _body: &Box<Block>) -> Value {
    todo!()
  }

  #[inline]
  fn translate_while(
    &mut self,
    _condition: &Box<Expr>,
    _body: &Box<Block>,
  ) -> Value {
    todo!()
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
}
