// run a command
#[inline]
pub fn run(args: Vec<String>) {
  command(args)
}

// launch a command to run with arguments
#[inline]
fn command(args: Vec<String>) {
  match args {
    arg if arg.contains(&"compile".into()) => crate::compile::run(arg),
    arg if arg.contains(&"repl".into()) => crate::repl::run(&arg),
    _ => crate::help::run(),
  }
}
