use crate::front::grammar::ProgramParser;
use crate::front::parser::ast::*;
use crate::util::error::Reporter;

use std::path::PathBuf;

pub fn parse<P: Into<PathBuf>>(pathname: P) -> Program {
  let mut reporter = Reporter::default();
  let source_id = reporter.add_source(pathname.into()).unwrap();
  let source_code = reporter.code(source_id);
  let parser = ProgramParser::new();

  match parser.parse(source_code) {
    Ok(items) => Program::new(items, reporter),
    Err(error) => panic!("{error}"),
  }
}
