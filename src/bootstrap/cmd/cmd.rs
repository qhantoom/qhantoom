use super::{compile, help, repl, version};

pub fn run() {
  cmd()
}

fn cmd() {
  let args = std::env::args().skip(1).collect::<Vec<String>>();

  if args.is_empty() {
    return help::run();
  }

  match args[0].as_str() {
    "compile" => compile::run(args),
    "repl" => repl::run(),
    "version" => version::run(),
    _ => help::run(),
  }
}
