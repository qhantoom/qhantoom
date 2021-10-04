use super::copyright;
use super::exit;
use super::help;
use super::license;
use super::version;

use qhantoomc::front::interpreter::{self, runtime::Runtime};

use qute::prelude::*;

// run the repl
#[inline]
pub fn run(args: &[String]) {
  match repl(args) {
    Ok(_) => return,
    Err(_) => panic!("repl error"),
  }
}

// read a line of input from stdin
#[inline]
fn repl(args: &[String]) -> Result<(), String> {
  let mut runtime = Runtime::new();

  banner();

  loop {
    match crate::util::readline("📡") {
      Ok(ref line) => processing(&mut runtime, args, line),
      Err(e) => Err(format!("{}", e)),
    }?;
  }
}

// process the line of input from stdin
#[inline]
fn processing(
  runtime: &mut Runtime,
  _args: &[String],
  line: &str,
) -> Result<(), String> {
  if line.is_empty() {
    return Ok(());
  }

  match line {
    l if l.starts_with(".exit") => Ok(exit::run()),
    l if l.starts_with("help") => Ok(help::run()),
    l if l.starts_with("copyright") => Ok(copyright::run()),
    l if l.starts_with("license") => Ok(license::run()),
    _ => match interpreter::interpret(runtime, line) {
      Ok(value) => Ok(print!("🛰️  {}\n", value)),
      Err(e) => Err(format!("{}", e)),
    }
  }
}

// print the repl banner
// design:
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

  let username_styled = qute!(&crate::util::username()).underline();
  let datetime_styled = qute!(&crate::util::datetime()).italic();

  use platform_info::Uname;
  let os = platform_info::PlatformInfo::new().unwrap();

  print!("\n");

  print!(
    "qhantoomc v{} ({})\n",
    version::version(),
    datetime_styled,
  );

  print!(
    "welcome {} to qhantoom version {} {}/{}\n",
    username_styled,
    version::version(),
    os.sysname(),
    os.machine(),
  );

  print!(
    "use {}, {} or {} for more information.\n",
    help_styled,
    copyright_styled,
    license_styled,
  );
}
