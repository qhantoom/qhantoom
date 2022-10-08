use crate::front::analyzer::context::Context;
use crate::front::parser::ast::*;
use crate::util::error::{Help, HelpKind};
use crate::util::error::{Label, LabelKind, LabelMessage};
use crate::util::error::{Note, NoteKind};

use crate::util::error::{
  Report, ReportCode, ReportKind, ReportMessage, ReportOffset,
};

use crate::util::span::Span;

// FIXME #1
//
// too much error panic. The functions should return a result type.
// in case of an error, an error report is returned. this way,
// i collect all possible errors in a vector. and once the
// verification phase is over, i display all the errors connected.

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

fn check_expr(context: &mut Context, expr: &Expr) -> PBox<Ty> {
  match &expr.kind {
    ExprKind::Lit(lit) => check_expr_lit(context, lit),
    ExprKind::Identifier(identifier) => {
      check_expr_identifier(context, identifier, expr.span)
    }
    ExprKind::Call(callee, args) => check_expr_call(context, callee, args),
    _ => unimplemented!("{}", expr),
  }
}

fn check_expr_lit(_context: &mut Context, lit: &Lit) -> PBox<Ty> {
  match &lit.kind {
    LitKind::Bool(_) => check_expr_lit_bool(lit),
    LitKind::Int(_) => check_expr_lit_int(lit),
    LitKind::Float(_) => check_expr_lit_float(lit),
    LitKind::Str(_) => check_expr_lit_str(lit),
  }
}

fn check_expr_lit_bool(lit: &Lit) -> PBox<Ty> {
  pbox(Ty::with_bool(lit.span))
}

fn check_expr_lit_int(lit: &Lit) -> PBox<Ty> {
  pbox(Ty::with_uint(lit.span))
}

fn check_expr_lit_float(lit: &Lit) -> PBox<Ty> {
  pbox(Ty::with_f64(lit.span))
}

fn check_expr_lit_str(lit: &Lit) -> PBox<Ty> {
  pbox(Ty::with_str(lit.span))
}

fn check_expr_identifier(
  context: &mut Context,
  identifier: &String,
  span: Span,
) -> PBox<Ty> {
  if let Some(ty) = context.scope_map.decl(identifier) {
    return ty.to_owned();
  } else if let Some(ty) = context.scope_map.fun(identifier) {
    return ty.0.to_owned();
  } else if let Some(ty) = context.scope_map.ty(identifier) {
    return ty.to_owned();
  } else {
    add_report_undefined_name_error(&context.program, identifier, span)
  }
}

fn check_expr_call(
  context: &mut Context,
  callee: &Expr,
  inputs: &Vec<PBox<Expr>>,
) -> PBox<Ty> {
  let (fun_return_ty, fun_input_tys) =
    match context.scope_map.fun(&callee.to_string()) {
      Some(fun_ty) => fun_ty,
      None => panic!("calling not defined function"), // FIXME #1
    };

  if inputs.len() != fun_input_tys.len() {
    add_report_wrong_input_count_error(
      &context.program,
      callee,
      inputs,
      fun_input_tys,
    );
  }

  for (x, input) in inputs.iter().enumerate() {
    if x < fun_input_tys.len() {
      check_verify(&mut context.to_owned(), input, &fun_input_tys[x]);
    }
  }

  check_verify(&mut context.to_owned(), callee, fun_return_ty);

  fun_return_ty.clone()
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

fn add_report_undefined_name_error(
  program: &Program,
  identifier: &String,
  span: Span,
) -> ! {
  let source_id = program.reporter.source(span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(span);

  program.reporter.raise(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(span.lo),
    )
    .with_code(ReportCode(3))
    .with_message(ReportMessage::UndefinedName(identifier.into()))
    .with_label(
      Label::new(LabelKind::Error, (path.display().to_string(), span.into()))
        .with_message(LabelMessage::UndefinedName),
    ),
    path.display().to_string(),
    code,
  )
}

fn add_report_wrong_input_count_error(
  program: &Program,
  callee: &Expr,
  actual_inputs: &Vec<PBox<Expr>>,
  expected_inputs: &Vec<PBox<Ty>>,
) {
  let source_id = program.reporter.source(callee.span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(callee.span);

  let expected_inputs_fmt = expected_inputs
    .iter()
    .map(|input| format!("`{}`", input.to_string()))
    .collect::<Vec<_>>()
    .join(", ");

  let actual_callee =
    format!("{}({})", callee.to_string(), expected_inputs_fmt);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(callee.span.lo),
    )
    .with_code(ReportCode(3))
    .with_message(ReportMessage::MissingInputs)
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), callee.span.into()),
      )
      .with_message(LabelMessage::MissingInputs(expected_inputs_fmt)),
    )
    .with_note(Note::new(NoteKind::MissingInputs(
      expected_inputs.len(),
      actual_inputs.len(),
    )))
    .with_help(Help::new(HelpKind::MissingInputs(actual_callee))),
    path.display().to_string(),
    code,
  );
}
