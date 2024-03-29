use crate::util::color::Color;
use crate::util::constant::PROGRAM_ENTRY;

use ariadne::Fmt;

use std::fmt;
use std::ops::Range;

type Source = (String, Range<usize>);

pub struct Report {
  pub kind: ReportKind,
  pub code: Option<ReportCode>,
  pub message: Option<ReportMessage>,
  pub labels: Vec<Label>,
  pub source: Source,
  pub help: Option<Help>,
  pub note: Option<Note>,
  pub offset: ReportOffset,
}

impl Report {
  pub fn new(kind: ReportKind, source: String, offset: ReportOffset) -> Self {
    Self {
      kind,
      code: None,
      message: None,
      labels: vec![],
      note: None,
      help: None,
      source: (source, 0..offset.into()),
      offset,
    }
  }

  pub fn with_code(mut self, code: ReportCode) -> Self {
    self.code = Some(code);
    self
  }

  pub fn with_message(mut self, message: ReportMessage) -> Self {
    self.message = Some(message);
    self
  }

  pub fn with_label(mut self, label: Label) -> Self {
    self.labels.push(label);
    self
  }

  pub fn with_note(mut self, note: Note) -> Self {
    self.note = Some(note);
    self
  }

  pub fn with_help(mut self, help: Help) -> Self {
    self.help = Some(help);
    self
  }
}

impl From<Report> for ariadne::Report<Source> {
  fn from(report: Report) -> Self {
    let mut report_builder = ariadne::Report::build(
      report.kind.into(),
      report.source.0,
      report.offset.into(),
    );

    if let Some(code) = report.code {
      report_builder = report_builder.with_code(code);
    }

    if let Some(message) = report.message {
      report_builder = report_builder.with_message(message);
    }

    if let Some(note) = report.note {
      report_builder = report_builder.with_note(note);
    }

    if let Some(help) = report.help {
      report_builder = report_builder.with_help(help);
    }

    for label in report.labels {
      report_builder = report_builder.with_label(label.into());
    }

    report_builder.finish()
  }
}

pub enum ReportKind {
  Advice,
  Error,
  Warning,
}

impl From<ReportKind> for ariadne::ReportKind {
  fn from(level: ReportKind) -> Self {
    match level {
      ReportKind::Advice => ariadne::ReportKind::Advice,
      ReportKind::Error => ariadne::ReportKind::Error,
      ReportKind::Warning => ariadne::ReportKind::Warning,
    }
  }
}

pub struct ReportCode(pub u8);

impl fmt::Display for ReportCode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "E{:02}", self.0)
  }
}

pub enum ReportMessage {
  DuplicateDeclaration(String),
  MainHasInputs,
  MainNotFound,
  MissingInputs,
  NameClash,
  NamingConvention(String, String),
  OutOfLoop(String),
  TypeMismatch,
  UndefinedName(String),
  WrongAssignOp,
  WrongBinOp,
  WrongUnOp(String),
}

impl fmt::Display for ReportMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::DuplicateDeclaration(name) => {
        write!(f, "{}", "variable".fg(Color::BLUE_100))?;
        write!(f, " {} ", format!("`{name}`").fg(Color::GREEN_100))?; // TODO: backticks should be in colour too
        write!(f, "{}", "already exist".fg(Color::BLUE_100))
      }
      Self::MainHasInputs => {
        write!(f, "{} ", "`main`".fg(Color::GREEN_100))?;
        write!(f, "{}", "function defined with args".fg(Color::BLUE_100))
      }
      Self::MainNotFound => {
        write!(f, "{} ", "`main`".fg(Color::GREEN_100))?;
        write!(f, "{} ", "function not found".fg(Color::BLUE_100))
      }
      Self::MissingInputs => {
        write!(f, "{}", "missing input arguments".fg(Color::BLUE_100))
      }
      Self::NameClash => write!(f, "{}", "name clash".fg(Color::BLUE_100)),
      Self::NamingConvention(name, convention) => {
        write!(f, "{}", "variable".fg(Color::BLUE_100))?;
        write!(f, " {} ", format!("`{name}`").fg(Color::GREEN_100))?;
        write!(f, "{}", "should have a".fg(Color::BLUE_100))?;
        write!(f, " {} ", convention.fg(Color::BLUE_100))
      }
      Self::OutOfLoop(behavior) => {
        write!(f, "{} ", format!("`{behavior}`").fg(Color::GREEN_100)).ok();
        write!(f, "{}", "outside of the loop".fg(Color::BLUE_100))
      }
      Self::TypeMismatch => {
        write!(f, "{}", "type mismatch".fg(Color::BLUE_100))
      }
      Self::UndefinedName(name) => {
        write!(f, "{}", "the name".fg(Color::BLUE_100))?;
        write!(f, " {} ", format!("`{name}`").fg(Color::GREEN_100))?;
        write!(f, "{}", "does not exist in this scope".fg(Color::BLUE_100))
      }
      Self::WrongAssignOp => write!(
        f,
        "{}",
        "wrong assignment operator expression".fg(Color::BLUE_100)
      ),
      Self::WrongBinOp => write!(f, "wrong binary operation expression"),
      Self::WrongUnOp(op) => {
        write!(f, "{}", "wrong unary op expression".fg(Color::BLUE_100))?;
        write!(f, " {}", format!("`{op}`").fg(Color::GREEN_100))
      }
    }
  }
}

#[derive(Clone, Copy)]
pub struct ReportOffset(pub u32);

impl From<ReportOffset> for usize {
  fn from(offset: ReportOffset) -> Self {
    offset.0 as usize
  }
}

pub struct Label {
  kind: LabelKind,
  message: LabelMessage,
  source: Source,
  order: u32,
}

impl Label {
  pub fn new(kind: LabelKind, source: Source) -> Self {
    Self {
      kind,
      source,
      message: LabelMessage::UnrecognizedToken,
      order: 0,
    }
  }

  pub fn with_message(mut self, message: LabelMessage) -> Self {
    self.message = message;
    self
  }

  pub fn with_order(mut self, order: u32) -> Self {
    self.order = order;
    self
  }
}

impl From<Label> for ariadne::Label<Source> {
  fn from(label: Label) -> Self {
    ariadne::Label::new(label.source)
      .with_message(label.message)
      .with_color(label.kind.into())
  }
}

pub enum LabelKind {
  Error,
  Help,
  Hint,
  Note,
  Warning,
}

impl From<LabelKind> for ariadne::Color {
  fn from(kind: LabelKind) -> Self {
    match kind {
      LabelKind::Error => Color::RED_100,
      LabelKind::Help => Color::YELLOW_100,
      LabelKind::Hint => Color::BLUE_200,
      LabelKind::Note => Color::BLUE_100,
      LabelKind::Warning => Color::YELLOW_100,
    }
  }
}

pub enum LabelMessage {
  DuplicateDeclaration,
  MainHasInputs,
  MainNotFound(String),
  MissingInputs(String),
  NameClash,
  NamingConvention(String, String),
  OutOfLoop(String),
  TypeMismatch(String, String),
  TypeMismatchDefinedAs(String),
  UndefinedName,
  UnrecognizedToken,
  WrongAssignOp(String, String),
  WrongBinOp(String, String),
  WrongUnOp(String),
}

impl fmt::Display for LabelMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::DuplicateDeclaration => write!(
        f,
        "{}",
        "this name is already declared in the scope".fg(Color::RED_100)
      ),
      Self::MainHasInputs => write!(
        f,
        "{}",
        "`main` function should not take any arguments".fg(Color::RED_100)
      ),
      Self::MainNotFound(source_entry) => write!(
        f,
        "{}",
        format!(
          "you need to add a `{PROGRAM_ENTRY}` function to `{source_entry}`",
        )
        .fg(Color::RED_100)
      ),
      Self::MissingInputs(inputs) => write!(
        f,
        "{}",
        format!("the input argument(s) of type {inputs} are required")
          .fg(Color::RED_100)
      ),
      Self::NameClash => {
        write!(
          f,
          "{}",
          "this argument is defined multiple times".fg(Color::RED_100)
        )
      }
      Self::NamingConvention(name, convention) => {
        write!(
          f,
          "{}",
          format!(
            "change this identifier to {convention} convention: `{name}`",
          )
          .fg(Color::YELLOW_100)
        )
      }
      Self::OutOfLoop(behavior) => {
        write!(
          f,
          "{}",
          format!("cannot `{behavior}` out of the loop").fg(Color::RED_100)
        )
      }
      Self::TypeMismatch(t1, t2) => {
        write!(
          f,
          "{}",
          format!("expected `{t1}`, found `{t2}`").fg(Color::RED_100)
        )
      }
      Self::TypeMismatchDefinedAs(ty) => {
        write!(f, "{}", format!("defined as `{ty}`").fg(Color::BLUE_200))
      }
      Self::UndefinedName => write!(
        f,
        "{}",
        "i don't know this id. are your sure you defined it correctly?"
          .fg(Color::RED_100)
      ),
      Self::WrongAssignOp(t1, t2) => {
        write!(
          f,
          "{}",
          format!(
            "lhs and rhs should have the same type, got `{t1}` and `{t2}`",
          )
          .fg(Color::RED_100)
        )
      }
      Self::WrongBinOp(t1, t2) => {
        write!(
          f,
          "{}",
          format!(
            "lhs and rhs should have the same type, got `{t1}` and `{t2}`",
          )
          .fg(Color::RED_100)
        )
      }
      Self::WrongUnOp(ty) => {
        write!(
          f,
          "{}",
          format!("this unary operator expects a `{ty}` expression")
            .fg(Color::RED_100)
        )
      }
      Self::UnrecognizedToken => write!(f, ""),
    }
  }
}

pub struct Note {
  kind: NoteKind,
}

impl Note {
  pub fn new(kind: NoteKind) -> Self {
    Self { kind }
  }
}

impl fmt::Display for Note {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

pub enum NoteKind {
  MainHasInputs(String),
  MainNotFound,
  MissingInputs(usize, usize),
  NameClash,
  UnrecognizedToken,
}

impl fmt::Display for NoteKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::MainHasInputs(inputs) => write!(
        f,
        "expected `fun()` \n\t     actual `fun({})`",
        inputs.fg(Color::BLUE_100)
      ),
      Self::MainNotFound => write!(
        f,
        "add the following code {} to your entry file",
        "`fun main() {}`".fg(Color::GREEN_200)
      ),
      Self::MissingInputs(expected, actual) => write!(
        f,
        "this function takes {expected} argument but {actual} arguments were supplied",
      ),
      Self::NameClash => {
        write!(
          f,
          "i'm not sure which one you want to use? rename one of them!"
        )
      }
      Self::UnrecognizedToken => write!(f, ""),
    }
  }
}

pub struct Help {
  kind: HelpKind,
}

impl Help {
  pub fn new(kind: HelpKind) -> Self {
    Self { kind }
  }
}

impl fmt::Display for Help {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

pub enum HelpKind {
  MissingInputs(String),
}

impl fmt::Display for HelpKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::MissingInputs(callee) => write!(
        f,
        "{}",
        format!("This is how you should call this function: {callee}")
          .fg(Color::YELLOW_100)
      ),
    }
  }
}
