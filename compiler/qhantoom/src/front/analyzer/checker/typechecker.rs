use crate::front::analyzer::context::Context;
use crate::front::parser::ast::*;
use crate::util::span::Span;

pub fn check(program: &Program) {
  let mut context = Context::new(program);

  for item in &context.program.items {
    check_item(&mut context, item);
  }
}

fn check_item(context: &mut Context, item: &Item) {
  match &item.kind {
    ItemKind::Fun(fun) => check_item_fun(context, fun),
    _ => unimplemented!("{}", item),
  }
}

fn check_item_fun(context: &mut Context, fun: &Fun) {
  check_block(context, &fun.body)
}

fn check_block(context: &mut Context, block: &Block) {
  for stmt in &block.stmts {
    check_stmt(context, stmt);
  }
}

fn check_stmt(context: &mut Context, stmt: &Stmt) {
  match &stmt.kind {
    StmtKind::Expr(expr) => check_stmt_expr(context, expr),
    _ => unimplemented!("{}", stmt),
  }
}

fn check_stmt_expr(context: &mut Context, expr: &Expr) {
  check_expr(context, expr);
}

fn check_expr(context: &mut Context, expr: &Expr) -> Ty {
  match &expr.kind {
    ExprKind::Lit(lit) => check_expr_lit(context, lit),
    _ => unimplemented!("{}", expr),
  }
}

fn check_expr_lit(_context: &mut Context, lit: &Lit) -> Ty {
  match &lit.kind {
    LitKind::Bool(_) => check_expr_lit_bool(lit),
    LitKind::Int(_) => check_expr_lit_int(lit),
    LitKind::Float(_) => check_expr_lit_float(lit),
    LitKind::Str(_) => check_expr_lit_str(lit),
  }
}

fn check_expr_lit_bool(lit: &Lit) -> Ty {
  Ty::with_bool(lit.span)
}

fn check_expr_lit_int(lit: &Lit) -> Ty {
  Ty::with_uint(lit.span)
}

fn check_expr_lit_float(lit: &Lit) -> Ty {
  Ty::with_f64(lit.span)
}

fn check_expr_lit_str(lit: &Lit) -> Ty {
  Ty::with_str(lit.span)
}

fn _check_equality(
  _context: &mut Context,
  span: &Span,
  t1: &Ty,
  t2: &Ty,
) -> bool {
  if t1.kind != t2.kind {
    println!("[{:?}:{:?}] {:?} - {:?}", t1, t2, span.lo, span.hi);
    false
  } else {
    true
  }
}

fn _check_verify(context: &mut Context, expr: &Expr, t1: &Ty) -> bool {
  let t2 = check_expr(context, expr);

  _check_equality(context, &expr.span, t1, &t2)
}
