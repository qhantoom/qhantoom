use std::fmt::Write;

use super::copyright;
use super::exit;
use super::help;
use super::license;
use super::version;

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
    l if l.starts_with("exit") => Ok(exit::run()),
    l if l.starts_with("help") => Ok(help::run()),
    l if l.starts_with("copyright") => Ok(copyright::run()),
    l if l.starts_with("license") => Ok(license::run()),
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
    &format!(
      "\nqhantoomc v{} ({})\n",
      version::version(),
      date_time_styled,
    )
  )
  .unwrap();

  write!(
    buf,
    "{}",
    &format!(
      "welcome {} to qhantoom version {} {}/{}\n",
      username_styled,
      version::version(),
      os.sysname(),
      os.machine(),
    )
  )
  .unwrap();

  write!(
    buf,
    "{}",
    &format!(
      "use {}, {} or {} for more information.\n",
      help_styled, copyright_styled, license_styled,
    )
  )
  .unwrap();

  print!("{}", buf);
}
