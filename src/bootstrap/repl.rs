use qhantoomc::front::interpreter::{self, runtime::Runtime};

// run the repl
#[inline]
pub fn run(args: &[String]) {
  match repl(args) {
    Ok(_) => return,
    Err(_) => panic!("repl error"),
  }
}

// read a line of input from stdin
#[inline]
fn repl(args: &[String]) -> Result<(), String> {
  let mut runtime = Runtime::new();

  loop {
    match crate::util::readline("📡") {
      Ok(ref line) => processing(&mut runtime, args, line),
      Err(e) => Err(format!("{}", e)),
    }?;
  }
}

// process the line of input from stdin
#[inline]
fn processing(
  runtime: &mut Runtime,
  _args: &[String],
  line: &str,
) -> Result<(), String> {
  if line.is_empty() {
    return Ok(());
  }

  exit_if_possible(line);

  match interpreter::interpret(runtime, line) {
    Ok(value) => Ok(print!("🛰️  {}\n", value)),
    Err(e) => Err(format!("{}", e)),
  }
}

// abort the program if the line of input is ".exit"
#[inline]
pub fn exit_if_possible(line: &str) {
  if line.starts_with(".exit") {
    println!("\nTriForce.. 👋\n");
    std::process::exit(0);
  }
}
