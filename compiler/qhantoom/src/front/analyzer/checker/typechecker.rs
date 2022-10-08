use crate::front::analyzer::context::Context;
use crate::front::parser::ast::*;
use crate::util::error::{Label, LabelKind, LabelMessage};

use crate::util::error::{
  Report, ReportCode, ReportKind, ReportMessage, ReportOffset,
};

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
    StmtKind::Decl(decl) => check_stmt_decl(context, decl),
    StmtKind::Expr(expr) => check_stmt_expr(context, expr),
    _ => unimplemented!("{}", stmt),
  }
}

fn check_stmt_decl(context: &mut Context, decl: &Decl) {
  check_decl(context, decl)
}

fn check_decl(context: &mut Context, decl: &Decl) {
  let name = &decl.pattern;
  let ty = &decl.ty;

  let Ok(_) = context.scope_map.set_decl(name.to_string(), ty.to_owned()) else {
    return add_report_variable_already_exist_error(
      name.to_string(),
      name.span,
      context.program,
    );
  };

  let name = if let PatternKind::Identifier(_, identifier) = &name.kind {
    identifier
  } else {
    // TODO: report error?
    unimplemented!();
  };

  let t1 = check_expr(context, name);
  let t2 = check_expr(context, &decl.value);

  check_equality(context, &t1, &t2);
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

fn check_equality(context: &mut Context, t1: &Ty, t2: &Ty) -> bool {
  if t1.kind != t2.kind {
    add_report_type_mismatch_error(t1, t2, context.program);
    false
  } else {
    true
  }
}

fn check_verify(context: &mut Context, expr: &Expr, t1: &Ty) -> bool {
  let t2 = check_expr(context, expr);

  check_equality(context, t1, &t2)
}

fn add_report_variable_already_exist_error(
  name: String,
  span: Span,
  program: &Program,
) {
  let source_id = program.reporter.source(span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(span);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(span.lo),
    )
    .with_code(ReportCode(3))
    .with_message(ReportMessage::DuplicateDeclaration(name))
    .with_label(
      Label::new(LabelKind::Error, (path.display().to_string(), span.into()))
        .with_message(LabelMessage::DuplicateDeclaration),
    ),
    path.display().to_string(),
    code,
  );
}

fn add_report_type_mismatch_error(t1: &Ty, t2: &Ty, program: &Program) {
  let source_id = program.reporter.source(t1.span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(t1.span);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(t2.span.lo),
    )
    .with_message(ReportMessage::TypeMismatch)
    .with_code(ReportCode(5))
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), t2.span.into()),
      )
      .with_message(LabelMessage::TypeMismatch(t1.to_string(), t2.to_string())),
    )
    .with_label(
      Label::new(
        LabelKind::Hint,
        (
          path.display().to_string(),
          t1.span.lo as usize..t1.span.hi as usize,
        ),
      )
      .with_message(LabelMessage::TypeMismatchDefinedAs(t1.to_string())),
    ),
    path.display().to_string(),
    code,
  );
}
