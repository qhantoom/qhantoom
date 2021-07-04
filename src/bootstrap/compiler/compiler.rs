use qhantoom::front;

use std::any::Any;
use std::path::Path;

pub fn run(args: Vec<String>) -> Result<(), Box<(dyn Any + Send + 'static)>> {
  use std::thread::Builder;

  const STACK_SIZE: usize = 128 * 1024 * 1024;

  print!("\nrunning..\n");
  print!("\nargs: {:?}\n", args);

  if args.len() == 0 {
    print!("\nhelp\n");
    return Ok(());
  }

  Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(move || compile(args))
    .unwrap()
    .join()
}

fn compile(args: Vec<String>) {
  let cmds = args.clone();

  print!("\ncompiling..\n");

  let mut ast = {
    match front::parser::parse_capsule_from_file(Path::new(&cmds[1])) {
      Ok(ast) => ast,
      Err(e) => panic!("io error: {}", e),
    }
  };

  print!("\nAST: {:?}\n", ast);

  let mut value = {
    let mut interpreter = front::interpreter::Interpreter::new();
    match front::interpreter::interpret_capsule_from_file(
      Path::new(&cmds[1]),
      &mut interpreter,
    ) {
      Ok(value) => value,
      Err(e) => panic!("io error: {}", e),
    }
  };

  print!("\nVALUE: {:?}\n", value);
  print!("\nfinished!\n");
}
