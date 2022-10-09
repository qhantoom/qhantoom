use crate::front::analyzer::context::Context;
use crate::front::parser::ast::*;

use crate::util::error::{
  Label, LabelKind, LabelMessage, Report, ReportKind, ReportMessage,
  ReportOffset,
};

use crate::util::span::Span;

use inflector::cases::pascalcase::{is_pascal_case, to_pascal_case};

use inflector::cases::screamingsnakecase::{
  is_screaming_snake_case, to_screaming_snake_case,
};

use inflector::cases::snakecase::{is_snake_case, to_snake_case};

use std::fmt;

enum NamingConvention {
  Pascal,
  Snake,
  SnakeScreaming,
}

impl fmt::Display for NamingConvention {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Pascal => write!(f, "pascal case"),
      Self::Snake => write!(f, "snake case"),
      Self::SnakeScreaming => write!(f, "screaming snake case"),
    }
  }
}

pub fn check(program: &Program) {
  let mut context = Context::new(program);

  for item in &context.program.items {
    check_item(&mut context, item);
  }
}

fn check_item(context: &mut Context, item: &Item) {
  match &item.kind {
    ItemKind::Val(val) => check_item_val(context, val),
    ItemKind::Ext(ext) => check_item_ext(context, ext),
    ItemKind::Fun(fun) => check_item_fun(context, fun),
    _ => unimplemented!(),
  }
}

fn check_item_val(context: &mut Context, decl: &Decl) {
  verify_screaming_snake_case(
    decl.pattern.to_string(),
    decl.pattern.span,
    context.program,
  );

  check_expr(context, &decl.value)
}

fn check_item_ext(context: &mut Context, ext: &Ext) {
  check_prototype(context, &ext.prototype);

  let Some(body) = &ext.body else { return; };

  check_block(context, body)
}

fn check_prototype(context: &mut Context, prototype: &Prototype) {
  verify_pascal_case(
    prototype.name.to_string(),
    prototype.name.span,
    context.program,
  );
  check_prototype_inputs(context, &prototype.inputs);
}

fn check_prototype_inputs(context: &mut Context, inputs: &Vec<PBox<Arg>>) {
  for input in inputs {
    verify_snake_case(
      input.pattern.to_string(),
      input.pattern.span,
      context.program,
    );
  }
}

fn check_item_fun(context: &mut Context, fun: &Fun) {
  check_fun(context, fun)
}

fn check_fun(context: &mut Context, fun: &Fun) {
  verify_snake_case(
    fun.prototype.name.to_string(),
    fun.prototype.name.span,
    context.program,
  );

  for input in &fun.prototype.inputs {
    verify_snake_case(
      input.pattern.to_string(),
      input.pattern.span,
      context.program,
    );
  }

  check_block(context, &fun.body);
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
  verify_snake_case(
    decl.pattern.to_string(),
    decl.pattern.span,
    context.program,
  );

  check_expr(context, &decl.value)
}

fn check_stmt_expr(context: &mut Context, expr: &Expr) {
  check_expr(context, expr)
}

fn check_expr(_context: &mut Context, expr: &Expr) {
  match &expr.kind {
    _ => {}
  }
}

fn verify_pascal_case(name: String, span: Span, program: &Program) {
  if !is_pascal_case(&name) {
    emit_report_wrong_naming_convention(
      name,
      span,
      NamingConvention::Pascal,
      program,
    );
  }
}

fn verify_snake_case(name: String, span: Span, program: &Program) {
  if !is_snake_case(&name) {
    emit_report_wrong_naming_convention(
      name,
      span,
      NamingConvention::Snake,
      program,
    )
  }
}

fn verify_screaming_snake_case(name: String, span: Span, program: &Program) {
  if !is_screaming_snake_case(&name) {
    emit_report_wrong_naming_convention(
      name,
      span,
      NamingConvention::SnakeScreaming,
      program,
    )
  }
}

fn emit_report_wrong_naming_convention(
  name: String,
  span: Span,
  convention: NamingConvention,
  program: &Program,
) {
  let source_id = program.reporter.source(span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(span);

  let naming = match convention {
    NamingConvention::Pascal => to_pascal_case(&name),
    NamingConvention::Snake => to_snake_case(&name),
    NamingConvention::SnakeScreaming => to_screaming_snake_case(&name),
  };

  program.reporter.add_report(
    Report::new(
      ReportKind::Warning,
      path.display().to_string(),
      ReportOffset(span.lo),
    )
    .with_message(ReportMessage::NamingConvention(
      name,
      convention.to_string(),
    ))
    .with_label(
      Label::new(
        LabelKind::Warning,
        (path.display().to_string(), span.into()),
      )
      .with_message(LabelMessage::NamingConvention(
        naming,
        convention.to_string(),
      )),
    ),
    path.display().to_string(),
    code,
  );
}
