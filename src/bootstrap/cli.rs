use super::compile;
use super::help;
use super::repl;

use clap::{App, Arg, SubCommand};

pub const DEFAULT_APP_NAME: &str = "qc";
pub const DEFAULT_BIN_NAME: &str = env!("CARGO_CRATE_NAME");
pub const DEFAULT_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DEFAULT_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const DEFAULT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SUBCOMMAND_COMPILE_NAME: &str = "compile";
pub const SUBCOMMAND_COMPILE_DESCRIPTION: &str = "Compile a qhantoom program";
pub const SUBCOMMAND_REPL_NAME: &str = "repl";
pub const SUBCOMMAND_REPL_DESCRIPTION: &str = "Run the qhantoom REPL";

// run a command
#[inline]
pub fn run() {
  cmd()
}

// launch a command to run with arguments
#[inline]
fn cmd() {
  let args = App::new(DEFAULT_APP_NAME)
    .about(DEFAULT_DESCRIPTION)
    .author(DEFAULT_AUTHORS)
    .bin_name(DEFAULT_BIN_NAME)
    .version(DEFAULT_VERSION)
    .before_help("") // use to print a new line before help
    .after_help("") // use to print a new line after help
    .subcommand(
      SubCommand::with_name(SUBCOMMAND_REPL_NAME)
        .about(SUBCOMMAND_COMPILE_DESCRIPTION)
        .arg(Arg::with_name("token")),
    )
    .subcommand(
      SubCommand::with_name(SUBCOMMAND_COMPILE_NAME)
        .about(SUBCOMMAND_REPL_DESCRIPTION)
        .arg(
          Arg::with_name("file")
            .multiple(true)
            .required(true)
            .takes_value(true),
        ),
    )
    .get_matches();

  match args.subcommand_name() {
    Some(SUBCOMMAND_COMPILE_NAME) => compile::cmd(args),
    Some(SUBCOMMAND_REPL_NAME) => repl::cmd(args),
    _ => help::cmd(),
  }
}