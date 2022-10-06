use super::report::Report;

use crate::util::constant::EXIT_FAILURE;
use crate::util::source::SourceMap;
use crate::util::span::Span;

use ariadne::sources;

use std::cell::Cell;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::{io, process};

#[derive(Debug)]
pub struct Reporter {
  has_errors: Cell<bool>,
  pub source_map: SourceMap,
}

impl Reporter {
  pub fn add_source<P: Into<PathBuf>>(&mut self, path: P) -> io::Result<u32> {
    self.source_map.add(path.into())
  }

  pub fn code(&self, source_id: u32) -> &str {
    self.source_map.code(source_id)
  }

  pub fn source(&self, span: Span) -> u32 {
    self.source_map.source(span)
  }

  pub fn path(&self, span: Span) -> &Path {
    self.source_map.path(span)
  }

  pub fn add_report(&self, report: Report, pathname: String, code: &str) {
    let stream = io::stderr();

    eprintln!();
    ariadne::Report::from(report)
      .write(sources(vec![(pathname, code)]), stream)
      .unwrap();

    self.has_errors.set(true);
  }

  pub fn raise(&self, report: Report, pathname: String, code: &str) -> ! {
    self.add_report(report, pathname, code);
    self.abort()
  }

  pub fn abort_if_has_error(&self) {
    if self.has_errors.get() {
      self.abort();
    }
  }

  pub fn abort(&self) -> ! {
    process::exit(EXIT_FAILURE)
  }
}

impl Default for Reporter {
  fn default() -> Self {
    Self {
      has_errors: Cell::new(false),
      source_map: SourceMap::default(),
    }
  }
}
