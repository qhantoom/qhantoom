use super::compile;
use super::help;
use super::repl;

// run a command
#[inline]
pub fn run(args: Vec<String>) {
  command(args)
}

// launch a command to run with arguments
#[inline]
fn command(args: Vec<String>) {
  match args {
    arg if arg.contains(&"compile".into()) => compile::run(arg),
    arg if arg.contains(&"repl".into()) => repl::run(&arg),
    _ => help::run(),
  }
}
