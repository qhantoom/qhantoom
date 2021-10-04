use std::any::Any;

use qhantoomc::front;

// run the compiler
#[inline]
pub fn run(args: Vec<String>) {
  match compile(args) {
    Ok(_) => std::process::exit(0),
    Err(_) => std::process::exit(2),
  }
}

// compile the program in a separate thread
#[inline]
fn compile(args: Vec<String>) -> Result<(), Box<(dyn Any + Send + 'static)>> {
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
fn compiling(args: Vec<String>) {
  print!("compiling..\n");

  let args = &args[1..];

  // read the file from the path
  let file = match crate::util::readfile(&args[0]) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
  };

  print!("\nfile: {}\n", file);

  // transform source code into tokens
  // let tokens = match front::tokenizer::tokenize(&file) {
  //   Ok(t) => t,
  //   Err(e) => panic!("tokenizer error: {}", e),
  // };

  // print!("\ntokens: {:#?}\n", tokens);

  // transform tokens into AST
  let ast = {
    match front::parser::parse(&file) {
      Ok(ast) => ast,
      Err(e) => panic!("io error: {}", e),
    }
  };

  print!("\nast: {:#?}\n", ast);

  // check the AST
  front::analyzer::maincheck::check(&ast);

  // transform AST into bytecode
  // write bytecode to file
  unsafe {
    qhantoomc::back::codegen::codegen_with_llvm(&ast);
  }

  // print success message
  print!("\ncompiled successfully..\n");
}
