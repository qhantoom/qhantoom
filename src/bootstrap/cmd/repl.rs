use std::fmt::Write;
use std::mem;

use super::help;

use crate::util;
use crate::util::interface::{EXIT_FAILURE, EXIT_SUCCESS, VERSION};

use platform_info::Uname;
use qhantoom::back::cranelift::jit::Codegen;
use qhantoom::front::analyzer::Analyzer;
use qhantoom::front::parser::Parser;
use qhantoom::front::tokenizer::Tokenizer;
use qhantoom::util::symbol::Symbols;
use qute::prelude::*;
use slowprint::slow_print;

pub fn run() {
  match repl() {
    Ok(_) => (),
    Err(e) => {
      print!("{}\n", e);
      std::process::exit(EXIT_FAILURE);
    }
  }
}

fn repl() -> Result<(), String> {
  let mut jit = Codegen::new();

  banner();

  loop {
    match util::reader::read_line("📡") {
      Ok(ref line) => processing(&mut jit, line),
      Err(e) => Err(format!("{}", e)),
    }?;
  }
}

fn banner() {
  let help_fmt = format!("{}", "\"help\"");
  let help_styled = qute!(&help_fmt).cyan().italic();
  let copyright_fmt = format!("{}", "\"copyright\"");
  let copyright_styled = qute!(&copyright_fmt).cyan().italic();
  let license_fmt = format!("{}", "\"license\"");
  let license_styled = qute!(&license_fmt).cyan().italic();
  let username_styled = qute!(&util::user::username()).underline();
  let date_time_styled = qute!(&util::timer::date_time()).italic();
  let os = platform_info::PlatformInfo::new().unwrap();
  let mut buf = String::new();

  write!(
    buf,
    "{}",
    &format!("\nqhantoomc v{} ({})\n", self::version(), date_time_styled)
  )
  .ok();

  write!(
    buf,
    "{}",
    &format!(
      "welcome {} to qhantoom version {} {}/{}\n",
      username_styled,
      self::version(),
      os.sysname(),
      os.machine(),
    )
  )
  .ok();

  write!(
    buf,
    "{}",
    &format!(
      "use {}, {} or {} for more information.\n",
      help_styled, copyright_styled, license_styled,
    )
  )
  .ok();

  let delay = std::time::Duration::from_millis(6);

  slow_print(&buf, delay);
}

fn processing(codegen: &mut Codegen, line: &str) -> Result<(), String> {
  if line.is_empty() {
    return Ok(());
  }

  match line {
    l if l.starts_with("exit") => self::exit(),
    l if l.starts_with("help") => self::help(),
    l if l.starts_with("copyright") => self::copyright(),
    l if l.starts_with("license") => self::license(),
    _ => self::compile(codegen, line),
  }

  Ok(())
}

fn compile(codegen: &mut Codegen, line: &str) {
  // create symbol table
  let mut syms = Symbols::new();

  // tokenizer: lexical analysis
  let mut tokenizer = Tokenizer::new(line, &mut syms);
  let tokens = tokenizer.tokenize();

  // parser: syntactical analysis
  let mut parser = Parser::new(tokens);
  let ast = parser.parse();

  // store symbols
  syms.store();

  // analyzer: semantic analysis
  let mut analyzer = Analyzer::new(&ast);
  let _checked = analyzer.analyze();

  // codegen: code generation
  let code = codegen.generate(&ast);
  let code_fn = unsafe { mem::transmute::<_, fn() -> i64>(code) };

  print!("{}\n", code_fn())
}

fn copyright() {
  print!("\nnot implemented yet\n\n");
}

fn exit() {
  print!("\nTriForce.. 👋\n");
  std::process::exit(EXIT_SUCCESS);
}

fn help() {
  help::run()
}

fn license() {
  match util::reader::read_file("LICENSE") {
    Ok(s) => print!("\n{}\n", s),
    Err(_) => print!("License not found"),
  }
}

fn version() -> &'static str {
  VERSION
}
