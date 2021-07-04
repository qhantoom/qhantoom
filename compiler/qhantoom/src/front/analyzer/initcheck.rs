use crate::front::parser::ast::{
  Capsule, Expr, ExprKind, Item, ItemKind, Stmt, StmtKind,
};

use crate::util::span::SPAN_ZERO;

#[inline]
pub fn check(capsule: &Box<Capsule>) {
  let mut initchecker = Initchecker::new();
  initchecker.run(capsule);
}

pub struct Initchecker {
  step: Box<Stmt>,
}

impl Initchecker {
  #[inline]
  pub fn new() -> Self {
    Self {
      step: box Stmt::new(StmtKind::Empty, SPAN_ZERO),
    }
  }

  #[inline]
  fn run(&mut self, capsule: &Box<Capsule>) {
    for item in capsule.module.items() {
      self.check_item(item);
    }
  }

  #[inline]
  fn check_item(&mut self, item: &Box<Item>) {
    match item.kind() {
      ItemKind::Fun(ref fun) => self.analyze_stmt(&fun.block),
      _ => {}
    }
  }

  #[inline]
  fn analyze_stmt(&mut self, stmt: &Box<Stmt>) {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.analyze_expr_stmt(expr),
      _ => {}
    }
  }

  #[inline]
  fn analyze_expr_stmt(&mut self, expr: &Box<Expr>) {
    match expr.kind() {
      ExprKind::If {
        condition: _,
        consequence: _,
        alternative: _,
      } => {}
      _ => {}
    }
  }
}
