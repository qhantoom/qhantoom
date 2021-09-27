use super::runtime::Runtime;
use super::value::{Fun, Local as VLocal, Ty, Value, VALUE_VOID};

use crate::front::parser::parse_stmts;

use crate::front::parser::ast::{
  self, BinopKind, Expr, ExprKind, FunDecl, Local, Stmt, StmtKind, UnopKind,
};

// interpret a program from a code source
#[inline]
pub fn interpret(
  runtime: &mut Runtime,
  src: &str,
) -> Result<Box<Value>, String> {
  let stmts = parse_stmts(src)?;
  let mut interpreter = Interpreter::new(runtime);

  interpreter.interpret(&stmts)
}

pub struct Interpreter<'a> {
  runtime: &'a mut Runtime,
}

impl<'a> Interpreter<'a> {
  #[inline]
  pub fn new(runtime: &'a mut Runtime) -> Self {
    Self { runtime: runtime }
  }

  #[inline]
  pub fn interpret(
    &mut self,
    stmts: &Vec<Box<Stmt>>,
  ) -> Result<Box<Value>, String> {
    let mut value = box VALUE_VOID;

    for stmt in stmts {
      value = self.interpret_stmt(stmt)?;

      if let Value::Return(ref v) = value.kind() {
        return Ok(v.to_owned());
      }
    }

    Ok(value)
  }

  #[inline]
  fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Box<Value>, String> {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.interpret_expr(expr),
      _ => todo!(),
    }
  }

  #[inline]
  fn interpret_expr(&mut self, expr: &Expr) -> Result<Box<Value>, String> {
    match expr.kind() {
      ExprKind::Int(ref expr) => Ok(box Value::Int(*expr)),
      ExprKind::Float(ref expr) => Ok(box Value::Float(*expr)),
      ExprKind::Bool(ref expr) => Ok(box Value::Bool(*expr)),
      ExprKind::Char(ref expr) => Ok(box Value::Char(*expr)),
      ExprKind::Str(ref expr) => Ok(box Value::Str(expr.to_owned())),
      ExprKind::Unop { ref op, ref rhs } => self.interpret_unop_expr(op, rhs),
      ExprKind::Binop {
        ref lhs,
        ref op,
        ref rhs,
      } => self.interpret_binop_expr(lhs, op, rhs),
      ExprKind::If {
        ref condition,
        ref consequence,
        ref alternative,
      } => self.interpret_if_expr(condition, consequence, alternative),
      _ => todo!(),
    }
  }

  #[inline]
  pub fn interpret_unop_expr(
    &mut self,
    op: &UnopKind,
    rhs: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    let rhs = self.interpret_expr(rhs)?;

    match op {
      UnopKind::Neg => self.interpret_neg_unop(op, &rhs),
      UnopKind::Not => self.interpret_not_unop(op, &rhs),
    }
  }

  #[inline]
  pub fn interpret_neg_unop(
    &mut self,
    op: &UnopKind,
    rhs: &Box<Value>,
  ) -> Result<Box<Value>, String> {
    match rhs.kind() {
      Value::Int(ref num) => Ok(box Value::Int(-num.to_owned())),
      Value::Float(ref num) => Ok(box Value::Float(-num.to_owned())),
      _ => Err(format!("unsupported neg unop: {}{:?}", op, rhs)),
    }
  }

  #[inline]
  pub fn interpret_not_unop(
    &mut self,
    op: &UnopKind,
    rhs: &Box<Value>,
  ) -> Result<Box<Value>, String> {
    match rhs.kind() {
      Value::Bool(ref boolean) => Ok(box Value::Bool(!boolean.to_owned())),
      Value::Int(ref num) => Ok(box Value::Bool(*num == 0)),
      _ => Err(format!("unsupported not unop: {}{:?}", op, rhs)),
    }
  }

  #[inline]
  pub fn interpret_binop_expr(
    &mut self,
    lhs: &Box<Expr>,
    op: &BinopKind,
    rhs: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    let lhs = self.interpret_expr(lhs)?;
    let rhs = self.interpret_expr(rhs)?;

    match (*lhs, *rhs) {
      (Value::Int(ref lhs), Value::Int(ref rhs)) => {
        self.interpret_int_binop(lhs, op, rhs)
      }
      (Value::Float(ref lhs), Value::Float(ref rhs)) => {
        self.interpret_float_binop(lhs, op, rhs)
      }
      (Value::Bool(ref lhs), Value::Bool(ref rhs)) => {
        self.interpret_bool_binop(lhs, op, rhs)
      }
      (Value::Char(ref lhs), Value::Char(ref rhs)) => {
        self.interpret_char_binop(lhs, op, rhs)
      }
      (Value::Str(ref lhs), Value::Str(ref rhs)) => {
        self.interpret_str_binop(lhs, op, rhs)
      }
      (lhs, rhs) => {
        Err(format!("unsupported binop: {:?} {:?} {:?}", lhs, op, rhs))
      }
    }
  }

  #[inline]
  pub fn interpret_int_binop(
    &mut self,
    lhs: &i32,
    op: &BinopKind,
    rhs: &i32,
  ) -> Result<Box<Value>, String> {
    let value = match op {
      BinopKind::Add => Value::Int(lhs + rhs),
      BinopKind::Sub => Value::Int(lhs - rhs),
      BinopKind::Mul => Value::Int(lhs * rhs),
      BinopKind::Div => Value::Int(lhs / rhs),
      BinopKind::Mod => Value::Int(lhs % rhs),
      BinopKind::Lt => Value::Bool(lhs < rhs),
      BinopKind::Gt => Value::Bool(lhs > rhs),
      BinopKind::Le => Value::Bool(lhs <= rhs),
      BinopKind::Ge => Value::Bool(lhs >= rhs),
      BinopKind::Eq => Value::Bool(lhs == rhs),
      BinopKind::Ne => Value::Bool(lhs != rhs),
    };

    Ok(box value)
  }

  #[inline]
  pub fn interpret_float_binop(
    &mut self,
    lhs: &f32,
    op: &BinopKind,
    rhs: &f32,
  ) -> Result<Box<Value>, String> {
    let value = match op {
      BinopKind::Add => Value::Float(lhs + rhs),
      BinopKind::Sub => Value::Float(lhs - rhs),
      BinopKind::Mul => Value::Float(lhs * rhs),
      BinopKind::Div => Value::Float(lhs / rhs),
      BinopKind::Mod => Value::Float(lhs % rhs),
      BinopKind::Lt => Value::Bool(lhs < rhs),
      BinopKind::Gt => Value::Bool(lhs > rhs),
      BinopKind::Le => Value::Bool(lhs <= rhs),
      BinopKind::Ge => Value::Bool(lhs >= rhs),
      BinopKind::Eq => Value::Bool(lhs == rhs),
      BinopKind::Ne => Value::Bool(lhs != rhs),
    };

    Ok(box value)
  }

  #[inline]
  fn interpret_bool_binop(
    &mut self,
    lhs: &bool,
    op: &BinopKind,
    rhs: &bool,
  ) -> Result<Box<Value>, String> {
    match op {
      BinopKind::Eq => Ok(box Value::Bool(lhs == rhs)),
      BinopKind::Ne => Ok(box Value::Bool(lhs != rhs)),
      _ => Err(format!(
        "unsupported bool binop: {:?} {:?} {:?}",
        lhs, op, rhs,
      )),
    }
  }

  #[inline]
  fn interpret_char_binop(
    &mut self,
    lhs: &char,
    op: &BinopKind,
    rhs: &char,
  ) -> Result<Box<Value>, String> {
    match op {
      BinopKind::Eq => Ok(box Value::Bool(lhs == rhs)),
      BinopKind::Ne => Ok(box Value::Bool(lhs != rhs)),
      _ => Err(format!(
        "unsupported char binop: {:?} {:?} {:?}",
        lhs, op, rhs,
      )),
    }
  }

  #[inline]
  fn interpret_str_binop(
    &mut self,
    lhs: &str,
    op: &BinopKind,
    rhs: &str,
  ) -> Result<Box<Value>, String> {
    match op {
      BinopKind::Add => Ok(box Value::Str(format!("{}{}", lhs, rhs))),
      BinopKind::Eq => Ok(box Value::Bool(lhs == rhs)),
      BinopKind::Ne => Ok(box Value::Bool(lhs != rhs)),
      _ => Err(format!(
        "unsupported str binop: {:?} {:?} {:?}",
        lhs, op, rhs,
      )),
    }
  }

  #[inline]
  fn interpret_if_expr(
    &mut self,
    condition: &Box<ast::Expr>,
    consequence: &Box<ast::Block>,
    alternative: &Option<Box<ast::Block>>,
  ) -> Result<Box<Value>, String> {
    let condition = self.interpret_expr(condition)?;

    if condition.as_bool() {
      self.interpret(&consequence.stmts)
    } else {
      alternative
        .as_ref()
        .map(|alt| self.interpret(&alt.stmts))
        .unwrap_or(Ok(box VALUE_VOID))
    }
  }
}
