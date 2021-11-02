use std::any::Any;

use super::cli::SUBCOMMAND_COMPILE_NAME;

use qhantoomc::back;
use qhantoomc::front;
use qhantoomc::util;

use clap::ArgMatches;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

// run the `compile` command
#[inline]
pub fn cmd(args: ArgMatches<'static>) {
  match compile(args) {
    Ok(_) => std::process::exit(EXIT_SUCCESS),
    Err(_) => std::process::exit(EXIT_FAILURE),
  }
}

// compile the program in a separate thread
#[inline]
fn compile(
  args: ArgMatches<'static>,
) -> Result<(), Box<(dyn Any + Send + 'static)>> {
  use std::thread::Builder;

  const STACK_SIZE: usize = 128 * 1024 * 1024;

  Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(move || compiling(args))
    .unwrap()
    .join()
}

// compile the program
#[inline]
fn compiling(args: ArgMatches) {
  print!("compiling..\n");

  if let Some(matches) = args.subcommand_matches(SUBCOMMAND_COMPILE_NAME) {
    let pathname = matches.value_of("file").unwrap();

    // read the file from the path
    let file = {
      match crate::util::read_file(&pathname) {
        Ok(f) => f,
        Err(e) => panic!("io error: {}", e),
      }
    };

    print!("\nfile: {}\n", file);

    // TMP: this is used just for printing the tokenize output
    // transform source code into tokens
    let tokens = {
      match front::tokenizer::tokenize(&file) {
        Ok(t) => t,
        Err(e) => panic!("tokenizer error: {}", e),
      }
    };

    print!("\ntokens: {:#?}\n", tokens);

    // transform source code into AST
    let ast = {
      match front::parser::parse(&file) {
        Ok(a) => a,
        Err(e) => panic!("io error: {}", e),
      }
    };

    print!("\nast: {:#?}\n", ast);

    // type checking the AST
    front::analyzer::maincheck::analyze(&ast);
    front::analyzer::typecheck::analyze(&ast);

    // code generation from an AST to machine code
    let code = {
      match back::codegen::aot::generate(&ast) {
        Ok(c) => c,
        Err(e) => panic!("codegen error: {}", e),
      }
    };

    print!("code: {:?}\n", code);

    // write machine code to file
    let _ = util::writer::write("test.o", code);

    // print success message
    print!("\ncompiled successfully..\n");
  }
}
