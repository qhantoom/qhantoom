use std::fs;
use std::io;
use std::path::Path;

// read the file at the given path and return the contents as a string
#[inline]
pub fn readfile(pathname: &str) -> Result<String, String> {
  let path = Path::new(pathname);

  match fs::read_to_string(path) {
    Ok(file) => Ok(file),
    Err(e) => Err(format!("{}", e)),
  }
}

// read the lines from stdin at the given path and return the contents as a string
#[inline]
pub fn readline<'a>(icon: &str) -> Result<String, String> {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut buf = String::new();

  print!("{} ", icon);

  io::Write::flush(&mut stdout).expect("flush failed!");
  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  buf.truncate(buf.trim_end().len());

  Ok(buf)
}
