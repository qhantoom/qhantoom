use super::compile;
use super::help;
use super::repl;

use clap::{App, Arg, ArgMatches, SubCommand};

// run a command
#[inline]
pub fn run() {
  let matches = App::new("qc")
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .bin_name("qc")
    .version(env!("CARGO_PKG_VERSION"))
    .before_help("") // use to print a new line before help
    .after_help("") // use to print a new line after help
    .subcommand(
      SubCommand::with_name("repl")
        .about("run the jit compiler")
        .arg(Arg::with_name("token")),
    )
    .subcommand(
      SubCommand::with_name("compile")
        .about("run the aot compiler")
        .arg(
          Arg::with_name("file")
            .multiple(true)
            .required(true)
            .takes_value(true),
        ),
    )
    .get_matches();

  command(matches)
}

// launch a command to run with arguments
#[inline]
fn command(args: ArgMatches<'static>) {
  match args.subcommand_name() {
    Some("compile") => compile::run(args),
    Some("repl") => repl::run(args),
    _ => help::run(),
  }
}
