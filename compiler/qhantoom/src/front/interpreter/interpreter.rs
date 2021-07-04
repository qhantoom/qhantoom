use std::path::Path;

use crate::front::parser::ast::{
  BinopKind, Capsule, Expr, ExprKind, FunDecl, Item, ItemKind, Local, Mod, Stmt,
  StmtKind, Ty,
};

use crate::front::parser::{
  parse_capsule_from_file, parse_capsule_from_source,
};

use crate::util::symbol::Symbol;

use super::scope::Scopemap;
use super::value;
use super::value::{Fun, Value, ValueKind};

#[inline]
pub fn interpret_capsule_from_file(
  path: &Path,
  interpreter: &mut Interpreter,
) -> Result<Box<Value>, String> {
  let capsule = parse_capsule_from_file(path)?;
  let value = interpreter.interpret_capsule_mod(capsule)?;

  Ok(value)
}

#[inline]
pub fn interpret_capsule_from_source(
  src: &str,
  interpreter: &mut Interpreter,
) -> Result<Box<Value>, String> {
  let capsule = parse_capsule_from_source(src)?;
  let value = interpreter.interpret_capsule_mod(capsule)?;

  Ok(value)
}

pub struct Interpreter {
  scopemap: Scopemap,
}

impl Interpreter {
  #[inline]
  pub fn new() -> Self {
    let mut i = Self {
      scopemap: Scopemap::new(),
    };

    i.bind();
    i
  }

  #[inline]
  pub fn bind(&mut self) {
    self.scopemap.bind()
  }

  #[inline]
  pub fn unbind(&mut self) {
    self.scopemap.unbind()
  }

  #[inline]
  pub fn bind_function(
    &mut self,
    fun: &Box<value::Fun>,
  ) -> Result<(), String> {
    self.scopemap.bind_function(*fun.to_owned())
  }

  #[inline]
  pub fn bind_variable(
    &mut self,
    local: &Box<value::Local>,
  ) -> Result<(), String> {
    self.scopemap.bind_variable(*local.to_owned())
  }

  #[inline]
  fn get_function(&self, name: &str) -> Option<&value::Fun> {
    self.scopemap.get_function(name)
  }

  #[inline]
  fn get_variable(&self, name: &str) -> Option<&value::Local> {
    self.scopemap.get_variable(name)
  }

  #[inline]
  fn interpret_capsule_mod(
    &mut self,
    capsule: Box<Capsule>,
  ) -> Result<Box<Value>, String> {
    let value = self.interpret_mod(&capsule.module)?;

    Ok(value::make_capsule(value))
  }

  #[inline]
  fn interpret_mod(&mut self, module: &Mod) -> Result<Box<Value>, String> {
    let obj = self.interpret_block(&module.items())?;

    Ok(value::make_mod(obj))
  }

  #[inline]
  fn interpret_block(
    &mut self,
    items: &Vec<Box<Item>>,
  ) -> Result<Box<Value>, String> {
    let mut obj = value::make_void_obj();

    for item in items {
      obj = self.interpret_item(item)?;

      if let ValueKind::Ret(ref expr) = obj.kind() {
        return Ok(expr.to_owned());
      }
    }

    Ok(obj)
  }

  fn interpret_item(
    &mut self,
    item: &Box<Item>,
  ) -> Result<Box<Value>, String> {
    match item.kind() {
      ItemKind::Fun(ref fun_decl) => self.interpret_fun_item(fun_decl),
      _ => unreachable!(),
    }
  }

  fn interpret_fun_item(
    &mut self,
    fun_decl: &Box<FunDecl>,
  ) -> Result<Box<Value>, String> {
    let name = match fun_decl.ident.kind() {
      ExprKind::Ident(ref sym) => value::make_ident(*sym),
      _ => unreachable!(),
    };

    let mut args = vec![];
    for (arg, ty) in &fun_decl.args {
      let name = match arg.kind() {
        ExprKind::Ident(ref sym) => value::make_ident(*sym),
        _ => unreachable!(),
      };

      let ty = self.interpret_ty_expr(ty)?;

      args.push((name, ty));
    }

    let ty = self.interpret_ty_expr(&fun_decl.ty)?;

    let block = match fun_decl.block.kind() {
      StmtKind::Block { ref kind } => self.interpret_block_stmt(kind)?,
      _ => unreachable!(),
    };

    let fun = box Fun::new(name, args, ty, block);

    self
      .bind_function(&fun)
      .and_then(|_| Ok(value::make_fun(fun.clone())))
  }

  fn interpret_block_stmt(
    &mut self,
    stmts: &Vec<Box<Stmt>>,
  ) -> Result<Box<Value>, String> {
    let mut obj = value::make_void_obj();

    for stmt in stmts {
      obj = self.interpret_stmt(stmt)?;

      if let ValueKind::Ret(ref expr) = obj.kind() {
        return Ok(expr.to_owned());
      }
    }

    Ok(obj)
  }

  fn interpret_stmt(
    &mut self,
    stmt: &Box<Stmt>,
  ) -> Result<Box<Value>, String> {
    match stmt.kind() {
      StmtKind::Local(ref local) => self.interpret_local_stmt(local),
      StmtKind::Ret(ref expr) => self.interpret_ret_stmt(expr),
      StmtKind::Expr(ref expr) => self.interpret_expr_stmt(expr),
      _ => unreachable!(),
    }
  }

  #[inline]
  fn interpret_local_stmt(
    &mut self,
    local: &Box<Local>,
  ) -> Result<Box<Value>, String> {
    let name = match local.ident.kind() {
      ExprKind::Ident(ref sym) => value::make_ident(*sym),
      _ => unreachable!(),
    };

    let ty = self.interpret_ty_expr(&local.ty)?;
    let value = self.interpret_expr_stmt(&local.value)?;
    let new_local = box value::Local::new(name, ty, value);

    self.bind_variable(&new_local).and_then(|_| Ok(value::make_local(new_local)))
  }

  #[inline]
  fn interpret_ret_stmt(
    &mut self,
    expr: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    let obj = self.interpret_expr_stmt(expr)?;

    Ok(value::make_ret(obj))
  }

  #[inline]
  fn interpret_expr_stmt(
    &mut self,
    expr: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    match expr.kind() {
      ExprKind::Int(ref int) => self.interpret_int_expr(int),
      ExprKind::Ident(ref ident) => self.interpret_ident_expr(ident),
      ExprKind::Array(ref array) => self.interpret_array_expr(array),
      ExprKind::Binop {
        ref lhs,
        ref op,
        ref rhs,
      } => self.interpret_binop_expr(lhs, op, rhs),
      ExprKind::Call {
        ref callee,
        ref args,
      } => self.interpret_call_expr(callee, args),
      _ => Ok(value::make_void_obj()),
    }
  }

  #[inline]
  fn interpret_int_expr(&mut self, int: &i32) -> Result<Box<Value>, String> {
    Ok(value::make_int(*int))
  }

  #[inline]
  fn interpret_binop_expr(
    &mut self,
    lhs_expr: &Box<Expr>,
    op: &BinopKind,
    rhs_expr: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    let lhs = self.interpret_expr_stmt(lhs_expr)?;
    let rhs = self.interpret_expr_stmt(rhs_expr)?;

    print!("\n\nBINOP: {:?}-{:?}-{:?}\n\n", lhs, op, rhs);

    let kind = match (lhs.kind(), rhs.kind()) {
      (ValueKind::Int(ref lhs), ValueKind::Int(ref rhs)) => match op {
        BinopKind::Add => ValueKind::Int(lhs + rhs),
        BinopKind::Sub => ValueKind::Int(lhs - rhs),
        _ => unreachable!(),
      },
      _ => unreachable!(),
    };

    Ok(value::make_obj(kind))
  }

  fn interpret_ident_expr(
    &mut self,
    ident: &Symbol,
  ) -> Result<Box<Value>, String> {
    let name = ident.to_string();

    if let Some(fun) = self.get_function(&name) {
      Ok(value::make_fun(box fun.to_owned()))
    } else if let Some(local) = self.get_variable(&name) {
      Ok(value::make_local(box local.to_owned()))
    } else {
      Err(format!(
        "error eval unknown identifier expression: {}",
        name
      ))
    }
  }

  #[inline]
  fn interpret_array_expr(&mut self, array: &Vec<Box<Expr>>) -> Result<Box<Value>, String> {
    let new_array = eval_expressions(self, array)?;

    Ok(value::make_array(new_array))
  }

  fn interpret_args_expr(
    &mut self,
    ident: &Box<Expr>,
  ) -> Result<Box<Value>, String> {
    match ident.kind() {
      ExprKind::Ident(ref ident) => Ok(value::make_ident(*ident)),
      _ => unreachable!(),
    }
  }

  fn interpret_call_expr(
    &mut self,
    callee: &Box<Expr>,
    args_expr: &[Box<Expr>],
  ) -> Result<Box<Value>, String> {
    let obj = self.interpret_expr_stmt(callee)?;

    match obj.kind() {
      ValueKind::Fun(ref fun) => self.interpret_fun_call(fun, args_expr),
      _ => unreachable!(),
    }
  }

  fn interpret_fun_call(
    &mut self,
    fun: &Box<Fun>,
    args: &[Box<Expr>],
  ) -> Result<Box<Value>, String> {
    if fun.args.len() != args.len() {
      return Err(format!(""));
    }

    // let fun_args = fun
    //   .args
    //   .iter()
    //   .map(|(o, t)| self.interpret_expr_stmt(o.clone()))
    //   .collect::<Result<Vec<_>>>();

    let _zargs = args.iter().zip(fun.args.iter());

    self.bind();

    // for (_, (ident, o)) in zargs.enumerate() {
    //   let Ident(name) = ident.clone();
    //   new_env.set(&name.clone(), o);
    // }

    self.unbind();

    // self.env = Rc::new(RefCell::new(new_env));
    // let obj = self.interpret_block(fun.block.stmts)?;

    // self.env = old_env;

    // Ok(value::unwrap(&obj))
    todo!()
  }

  fn interpret_ty_expr(
    &mut self,
    ty: &Box<Ty>,
  ) -> Result<Box<value::Ty>, String> {
    Ok(value::make_ty(ty))
  }
}


fn eval_expressions(
  interpreter: &mut Interpreter,
  exprs: &Vec<Box<Expr>>,
) -> Result<Vec<Box<Value>>, String> {
  let values = exprs
    .iter()
    .map(|expr| -> Result<Box<Value>, String> {
      Ok(interpreter.interpret_expr_stmt(expr)?)
    })
    .collect::<Result<Vec<Box<Value>>, String>>()?;

  Ok(values)
}