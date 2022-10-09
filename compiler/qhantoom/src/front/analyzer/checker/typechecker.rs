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
//
// FIXME #2
//
// define an error classification to assign the corresponding code
// to each error report

pub fn check(program: &Program) {
  let mut context = Context::new(program);

  for item in &context.program.items {
    check_item(&mut context, item);
  }
}

fn check_item(context: &mut Context, item: &Item) {
  match &item.kind {
    ItemKind::Ext(ext) => check_item_ext(context, ext),
    ItemKind::Val(decl) => check_item_val(context, decl),
    ItemKind::Fun(fun) => check_item_fun(context, fun),
    _ => todo!("{}", item),
  }
}

fn check_item_ext(context: &mut Context, ext: &Ext) {
  match context.scope_map.set_fun(
    ext.prototype.name.to_string(),
    (ext.prototype.as_ty(), ext.prototype.as_inputs_tys()),
  ) {
    Ok(_) => {
      context.scope_map.enter_scope();
      check_prototype(context, &ext.prototype);

      if let Some(body) = &ext.body {
        check_block(context, body);
      };

      context.scope_map.exit_scope();
    }
    Err(_) => todo!(),
  }
}

fn check_item_val(context: &mut Context, decl: &Decl) {
  match context
    .scope_map
    .set_decl(decl.pattern.to_string(), decl.ty.clone())
  {
    Ok(_) => {
      check_verify(context, &decl.value, &decl.ty);
    }
    Err(_) => add_report_variable_already_exist_error(
      decl.pattern.to_string(),
      decl.span,
      context.program,
    ),
  }
}

fn check_item_fun(context: &mut Context, fun: &Fun) {
  match context.scope_map.set_fun(
    fun.prototype.name.to_string(),
    (fun.prototype.as_ty(), fun.prototype.as_inputs_tys()),
  ) {
    Ok(_) => {
      context.scope_map.enter_scope();
      check_prototype(context, &fun.prototype);
      check_block(context, &fun.body);
      context.scope_map.exit_scope();
    }
    Err(_error) => todo!(),
  }
}

fn check_prototype(context: &mut Context, prototype: &Prototype) {
  // register inputs to the function scope
  for input in &prototype.inputs {
    if context
      .scope_map
      .set_decl(input.pattern.to_string(), input.ty.to_owned())
      .is_err()
    {
      add_report_name_clash_if_error(context.program, input);
    }
  }

  // preserve the return type
  context.return_ty = prototype.as_ty();
}

fn check_block(context: &mut Context, block: &Block) {
  for stmt in &block.stmts {
    check_stmt(context, stmt);
  }
}

fn check_stmt(context: &mut Context, stmt: &Stmt) {
  match &stmt.kind {
    StmtKind::Item(item) => check_stmt_item(context, item),
    StmtKind::Decl(decl) => check_stmt_decl(context, decl),
    StmtKind::Expr(expr) => check_stmt_expr(context, expr),
  }
}

fn check_stmt_item(context: &mut Context, item: &Item) {
  check_item(context, item)
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
    todo!();
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
    ExprKind::UnOp(op, rhs) => check_expr_un_op(context, op, rhs),
    ExprKind::BinOp(lhs, op, rhs) => check_expr_bin_op(context, lhs, op, rhs),
    ExprKind::Assign(lhs, op, rhs) => check_expr_assign(context, lhs, op, rhs),
    ExprKind::AssignOp(lhs, op, rhs) => {
      check_expr_assign_op(context, lhs, op, rhs)
    }
    ExprKind::Return(maybe_expr) => {
      check_expr_return(context, maybe_expr, expr.span)
    }
    ExprKind::Block(body) => check_expr_block(context, body),
    ExprKind::Loop(body) => check_expr_loop(context, body),
    ExprKind::While(condition, body) => {
      check_expr_while(context, condition, body)
    }
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
  Ty::with_bool(lit.span).into()
}

fn check_expr_lit_int(lit: &Lit) -> PBox<Ty> {
  Ty::with_uint(lit.span).into()
}

fn check_expr_lit_float(lit: &Lit) -> PBox<Ty> {
  Ty::with_f64(lit.span).into()
}

fn check_expr_lit_str(lit: &Lit) -> PBox<Ty> {
  Ty::with_str(lit.span).into()
}

fn check_expr_identifier(
  context: &mut Context,
  identifier: &String,
  span: Span,
) -> PBox<Ty> {
  if let Some(ty) = context.scope_map.decl(identifier) {
    ty.to_owned()
  } else if let Some(ty) = context.scope_map.fun(identifier) {
    ty.0.to_owned()
  } else {
    raise_report_undefined_name_error(context.program, identifier, span)
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
      context.program,
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

fn check_expr_un_op(context: &mut Context, op: &UnOp, rhs: &Expr) -> PBox<Ty> {
  let t1 = check_expr(context, rhs);

  match &op.node {
    UnOpKind::Neg => {
      if !t1.is_numeric() {
        add_report_wrong_un_op_error(context.program, op, &Ty::UINT);
      }

      Ty::with_uint(Span::merge(&op.span, &rhs.span)).into()
    }
    UnOpKind::Not => {
      if !t1.is_boolean() {
        add_report_wrong_un_op_error(context.program, op, &Ty::BOOL);
      }

      Ty::with_bool(Span::merge(&op.span, &rhs.span)).into()
    }
  }
}

fn check_expr_bin_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);
  let t2 = check_expr(context, rhs);

  // TODO: ugly stuff, this will be improve later
  match &op.node {
    BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge => {
      if t1.kind != TyKind::Bool || t2.kind != TyKind::Bool {
        raise_report_wrong_bin_op_error(context.program, op, &t1, &t2);
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    BinOpKind::And | BinOpKind::Or => {
      if t1.kind != t2.kind {
        raise_report_wrong_bin_op_error(context.program, op, &t1, &t2);
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    BinOpKind::Eq | BinOpKind::Ne => {
      if t1.kind != t2.kind {
        raise_report_wrong_bin_op_error(context.program, op, &t1, &t2);
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    _ => {
      if t1.kind != t2.kind {
        raise_report_wrong_bin_op_error(context.program, op, &t1, &t2);
      }

      Ty::with_uint(Span::merge(&lhs.span, &rhs.span)).into()
    }
  }
}

fn check_expr_assign(
  context: &mut Context,
  lhs: &Expr,
  _: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);

  check_verify(context, rhs, &t1);
  Ty::with_void(Span::merge(&lhs.span, &rhs.span)).into()
}

fn check_expr_assign_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);
  let t2 = check_expr(context, rhs);

  if !op.node.is_assign_op() {
    raise_report_wrong_assign_op_error(context.program, op, &t1, &t2)
  }

  check_equality(context, &t1, &t2);
  Ty::with_void(Span::merge(&lhs.span, &rhs.span)).into()
}

fn check_expr_return(
  context: &mut Context,
  maybe_expr: &Option<PBox<Expr>>,
  return_span: Span,
) -> PBox<Ty> {
  if let Some(expr) = maybe_expr {
    let t1 = check_expr(context, expr);

    check_equality(context, &t1, &context.return_ty.clone());

    return t1;
  };

  Ty::with_void(return_span).into()
}

fn check_expr_block(context: &mut Context, body: &Block) -> PBox<Ty> {
  for stmt in &body.stmts {
    check_stmt(context, stmt);
  }

  Ty::with_void(body.span).into()
}

fn check_expr_loop(context: &mut Context, body: &Block) -> PBox<Ty> {
  context.loops += 1;
  check_block(context, body);
  context.loops -= 1;

  Ty::with_void(body.span).into()
}

fn check_expr_while(
  context: &mut Context,
  condition: &Expr,
  body: &Block,
) -> PBox<Ty> {
  check_verify(context, condition, &Ty::with_bool(condition.span));
  context.loops += 1;
  check_block(context, body);
  context.loops -= 1;

  Ty::with_void(body.span).into()
}

fn check_verify(context: &mut Context, expr: &Expr, t1: &Ty) -> bool {
  let t2 = check_expr(context, expr);

  check_equality(context, t1, &t2)
}

fn check_equality(context: &mut Context, t1: &Ty, t2: &Ty) -> bool {
  if t1.kind != t2.kind {
    add_report_type_mismatch_error(t1, t2, context.program);
    false
  } else {
    true
  }
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
    .with_code(ReportCode(3)) // FIXME #2
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
    .with_code(ReportCode(5)) // FIXME #2
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

fn raise_report_undefined_name_error(
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
    .with_code(ReportCode(3)) // FIXME #2
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
    .map(|input| format!("`{}`", input))
    .collect::<Vec<_>>()
    .join(", ");

  let actual_callee = format!("{}({})", callee, expected_inputs_fmt);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(callee.span.lo),
    )
    .with_code(ReportCode(3)) // FIXME #2
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

fn add_report_wrong_un_op_error(program: &Program, op: &UnOp, ty: &Ty) {
  let source_id = program.reporter.source(op.span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(op.span);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(op.span.lo),
    )
    .with_code(ReportCode(3)) // FIXME #2
    .with_message(ReportMessage::WrongUnOp(op.node.to_string()))
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), op.span.into()),
      )
      .with_message(LabelMessage::WrongUnOp(ty.to_string())),
    ),
    path.display().to_string(),
    code,
  )
}

fn add_report_name_clash_if_error(program: &Program, input: &Arg) {
  let span = input.span;
  let source_id = program.reporter.source(span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(span);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(span.lo),
    )
    .with_code(ReportCode(7)) // FIXME #2
    .with_message(ReportMessage::NameClash)
    .with_label(
      Label::new(LabelKind::Error, (path.display().to_string(), span.into()))
        .with_message(LabelMessage::NameClash),
    )
    .with_note(Note::new(NoteKind::NameClash)),
    path.display().to_string(),
    code,
  )
}

fn raise_report_wrong_bin_op_error(
  program: &Program,
  op: &BinOp,
  t1: &Ty,
  t2: &Ty,
) -> ! {
  let source_id = program.reporter.source(op.span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(op.span);

  program.reporter.raise(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(op.span.lo),
    )
    .with_code(ReportCode(7)) // FIXME #2
    .with_message(ReportMessage::WrongBinOp)
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), op.span.into()),
      )
      .with_message(LabelMessage::WrongBinOp(t1.to_string(), t2.to_string())),
    ),
    path.display().to_string(),
    code,
  )
}

fn raise_report_wrong_assign_op_error(
  program: &Program,
  op: &BinOp,
  t1: &Ty,
  t2: &Ty,
) -> ! {
  let source_id = program.reporter.source(op.span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(op.span);

  program.reporter.raise(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(op.span.lo),
    )
    .with_code(ReportCode(7)) // FIXME #2
    .with_message(ReportMessage::WrongAssignOp)
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), op.span.into()),
      )
      .with_message(LabelMessage::WrongAssignOp(
        t1.to_string(),
        t2.to_string(),
      )),
    ),
    path.display().to_string(),
    code,
  )
}
