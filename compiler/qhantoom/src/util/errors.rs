use std::io;
use std::io::Write;
use std::process;

use super::span::Span;

#[inline]
pub fn abort() -> ! {
  process::exit(1)
}

#[inline]
pub fn error(msg: &str) {
  let mut stderr = io::stderr();

  print_error(&mut stderr);
  write!(&mut stderr, " : {}", msg).ok();
}

#[inline]
pub fn error_at(msg: &str, span: Span) {
  let mut stderr = io::stderr();

  print_error(&mut stderr);
  write!(&mut stderr, "\n at line {}:{}: {}", span.lo, span.hi, msg).ok();
}

#[inline]
fn print_error(stderr: &mut io::Stderr) {
  write!(stderr, "error").ok();
}

// #[inline]
// pub fn info(msg) {
//   console.log(msg)
// }

// #[inline]
// pub fn success(msg) {
//   console.log(msg)
// }

// #[inline]
// pub fn warn(msg) {
//   console.warn(msg)
// }

// #[inline]
// pub fn error(msg) {
//   console.error(msg)
// }
