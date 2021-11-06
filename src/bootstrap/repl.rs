use std::fmt::Write;

use super::cli::DEFAULT_VERSION;
use super::compile::EXIT_SUCCESS;
use super::help;

use crate::util;

use qhantoomc::back;
use qhantoomc::back::codegen::jit::Jit;

use clap::ArgMatches;
use platform_info::Uname;
use qute::prelude::*;

// run the `repl` command
#[inline]
pub fn run(args: ArgMatches<'static>) {
  match repl(args) {
    Ok(_) => return,
    Err(e) => panic!("{}", e),
  }
}

// read a line of input from stdin
#[inline]
fn repl(args: ArgMatches<'static>) -> Result<(), String> {
  let mut jit = Jit::new();

  banner();

  loop {
    match util::read_line("📡") {
      Ok(ref line) => processing(&mut jit, &args, line),
      Err(e) => Err(format!("{}", e)),
    }?;
  }
}

// process the line of input from stdin
#[inline]
fn processing(
  jit: &mut Jit,
  _args: &ArgMatches,
  line: &str,
) -> Result<(), String> {
  if line.is_empty() {
    return Ok(());
  }

  match line {
    l if l.starts_with("exit") => Ok(self::exit()),
    l if l.starts_with("help") => Ok(self::help()),
    l if l.starts_with("copyright") => Ok(self::copyright()),
    l if l.starts_with("license") => Ok(self::license()),
    _ => match back::codegen::jit::compile::<i64>(jit, line) {
      Ok(v) => Ok(print!("🛰️  {}\n", v)),
      Err(e) => Err(format!("{}", e)),
    },
  }
}

// print the repl banner
// the design banner should looks like this:
// qhantoomc v0.1.0 (Oct 03 2021, 20:05:01)
// welcome {user} to qhantoom version 0.1.0 Darwin/x86_64
// use "help", "copyright" or "license" for more information.
// 📡
#[inline]
pub fn banner() {
  let help_fmt = format!("{}", "\"help\"");
  let help_styled = qute!(&help_fmt).cyan().italic();
  let copyright_fmt = format!("{}", "\"copyright\"");
  let copyright_styled = qute!(&copyright_fmt).cyan().italic();
  let license_fmt = format!("{}", "\"license\"");
  let license_styled = qute!(&license_fmt).cyan().italic();
  let username_styled = qute!(&util::username()).underline();
  let date_time_styled = qute!(&util::date_time()).italic();
  let os = platform_info::PlatformInfo::new().unwrap();
  let mut buf = String::new();

  write!(
    buf,
    "{}",
    &format!("\nqhantoomc v{} ({})\n", self::version(), date_time_styled,)
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

  print!("{}", buf);
}

// abort the program
#[inline]
pub fn exit() {
  print!("\nTriForce.. 👋\n");
  std::process::exit(EXIT_SUCCESS);
}

// print the copyright
#[inline]
pub fn copyright() {
  print!("\nnot implemented yet\n\n");
}

// display the usage of the help command
#[inline]
pub fn help() {
  help::run()
}

// get the current version from `cargo.toml`
#[inline]
pub fn version() -> &'static str {
  DEFAULT_VERSION
}

// print the `LICENSE` if found
#[inline]
pub fn license() {
  match util::read_file("LICENSE") {
    Ok(s) => print!("\n{}\n", s),
    Err(_) => print!("License not found"),
  }
}
