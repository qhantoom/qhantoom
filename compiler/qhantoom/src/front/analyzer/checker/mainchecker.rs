use crate::front::analyzer::context::Context;
use crate::front::parser::ast::*;
use crate::util::constant::PROGRAM_ENTRY;
use crate::util::error::{Label, LabelKind, LabelMessage};
use crate::util::error::{Note, NoteKind};

use crate::util::error::{
  Report, ReportCode, ReportKind, ReportMessage, ReportOffset,
};

use crate::util::span::Span;

pub fn check(program: &Program) {
  let context = Context::new(program);

  if !context.program.items.iter().any(has_main(&context)) {
    add_report_main_not_found_error(context.program);
  }

  context.program.reporter.abort_if_has_error()
}

fn has_main<'a>(
  context: &'a Context,
) -> Box<impl FnMut(&'a PBox<Item>) -> bool + 'a> {
  Box::new(move |item: &'a PBox<Item>| {
    if let ItemKind::Fun(fun) = &item.kind {
      if fun.prototype.name.to_string() == PROGRAM_ENTRY {
        if !fun.prototype.inputs.is_empty() {
          add_report_main_has_inputs(context.program, fun);
        }

        return true;
      }
    }

    false
  })
}

fn add_report_main_not_found_error(program: &Program) {
  let code = program.reporter.code(0);
  let code = if code.is_empty() { " " } else { code };
  let path = program.reporter.path(Span::new(0, code.len()));

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(code.len() as u32),
    )
    .with_code(ReportCode(1))
    .with_message(ReportMessage::MainNotFound)
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), code.len() - 1..code.len()),
      )
      .with_message(LabelMessage::MainNotFound(path.display().to_string())),
    )
    .with_note(Note::new(NoteKind::MainNotFound)),
    path.display().to_string(),
    code,
  )
}

fn add_report_main_has_inputs(program: &Program, fun: &Fun) {
  let inputs = &fun.prototype.inputs;
  let single_span = fun.prototype.inputs[0].span;

  let merged_span = inputs
    .iter()
    .fold(single_span, |acc, value| Span::merge(&acc, &value.span));

  let source_id = program.reporter.source(merged_span);
  let code = program.reporter.code(source_id);
  let path = program.reporter.path(merged_span);

  program.reporter.add_report(
    Report::new(
      ReportKind::Error,
      path.display().to_string(),
      ReportOffset(single_span.lo),
    )
    .with_code(ReportCode(2))
    .with_message(ReportMessage::MainHasInputs)
    .with_label(
      Label::new(
        LabelKind::Error,
        (path.display().to_string(), merged_span.into()),
      )
      .with_message(LabelMessage::MainHasInputs),
    )
    .with_note(Note::new(NoteKind::MainHasInputs(
      fun
        .prototype
        .inputs
        .iter()
        .map(|input| input.ty.to_string())
        .collect::<Vec<_>>()
        .join(", "),
    ))),
    path.display().to_string(),
    code,
  )
}
