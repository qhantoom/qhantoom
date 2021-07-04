use qhantoom::front;
use qhantoom::front::interpreter::Interpreter;
use qhantoom::util;

pub fn run(_args: &[String]) -> Result<(), String> {
  let mut interpreter = Interpreter::new();

  loop {
    match util::reader::readline("📡") {
      Ok(ref line) => processing(line, &mut interpreter),
      Err(e) => Err(format!("{}", e)),
    }?;
  }
}

fn processing(
  line: &str,
  interpreter: &mut Interpreter,
) -> Result<(), String> {
  exit_if_possible(line);

  // tmp
  let _ = tokenize(line);
  let _ = parse(line);
  let _ = analyze(line);
  let _ = interpret(line, interpreter);

  Ok(())
}

#[inline]
pub fn exit_if_possible(line: &str) {
  if line.starts_with(".exit") {
    println!("\nTriForce..\n");
    std::process::exit(0);
  }
}

#[inline]
fn tokenize(src: &str) {
  match front::tokenizer::tokenize_capsule_from_source(src) {
    Ok(tokens) => print!("tokenize \\-> tokens: {:?}\n\n", tokens),
    Err(e) => print!("\n{}", e),
  }
}

#[inline]
fn parse(src: &str) {
  match front::parser::parse_capsule_from_source(src) {
    Ok(ast) => print!("parse \\-> ast: {:?}\n\n", ast),
    Err(e) => print!("\n{}", e),
  }
}

#[inline]
fn analyze(src: &str) {
  match front::analyzer::analyze_capsule_from_source(src) {
    Ok(result) => print!("analyze \\-> {:?}\n\n", result),
    Err(e) => print!("\n{}", e),
  }
}

#[inline]
fn interpret(src: &str, mut interpreter: &mut Interpreter) {
  match front::interpreter::interpret_capsule_from_source(
    src,
    &mut interpreter,
  ) {
    Ok(value) => print!("interpret \\-> value: {:?}\n\n", value),
    Err(e) => print!("\n{}", e),
  }
}
