use std::any::Any;
use std::process::{self, Command};
use std::thread;

use crate::util;
use crate::util::interface::{EXIT_FAILURE, EXIT_SUCCESS};

use qhantoom::back::cranelift::aot::Codegen;
use qhantoom::front::analyzer::Analyzer;
use qhantoom::front::parser::Parser;
use qhantoom::front::tokenizer::Tokenizer;
use qhantoom::util::symbol::Symbols;

pub fn run(args: Vec<String>) {
  match compile(args) {
    Ok(_) => process::exit(EXIT_SUCCESS),
    Err(_) => process::exit(EXIT_FAILURE),
  }
}

fn compile(args: Vec<String>) -> Result<(), Box<(dyn Any + Send + 'static)>> {
  thread::spawn(move || compiling(args)).join()
}

fn compiling(args: Vec<String>) {
  print!("compiling...\n");

  let file = match util::reader::read_file(&args[1]) {
    Ok(f) => f,
    Err(e) => return print!("\n{}\n", e),
  };

  // create symbol table
  let mut syms = Symbols::new();

  // tokenizer: lexical analysis
  let mut tokenizer = Tokenizer::new(&file, &mut syms);
  let tokens = tokenizer.tokenize();

  print!("\ntokens: {:#?}\n", tokens);

  // parser: syntactical analysis
  let mut parser = Parser::new(tokens);
  let ast = parser.parse();

  print!("\nast: {:#?}\n", ast);

  // store symbols
  syms.store();

  // analyzer: semantic analysis
  let mut analyzer = Analyzer::new(&ast);
  let checked = analyzer.analyze();

  print!("\nchecked: {}\n", checked);

  // codegen: code generation
  let codegen = Codegen::new();
  let code = codegen.generate(&ast);

  print!("\ncode: {:?}\n", code);

  // create object file
  let _ = util::writer::write_file("test.o", code);

  // create executable file
  let _ = Command::new("gcc")
    .arg("-o")
    .arg("test")
    .arg("test.o")
    .output();

  print!("\ncompiled succesfully...\n");
}
