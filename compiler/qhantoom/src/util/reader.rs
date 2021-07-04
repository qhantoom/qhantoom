use std::fs;
use std::io;
use std::path::Path;

#[inline]
pub fn readfile(path: &Path) -> Result<String, String> {
  // let mut file = fs::File::open(path)?;
  // let length = file.metadata()?.len() as usize;
  // let mut s = String::with_capacity(length + 1);

  // match file.read_to_string(&mut s) {
  match fs::read_to_string(path) {
    Ok(file) => Ok(file),
    Err(e) => Err(format!("{}", e)),
  }
}

#[inline]
pub fn readline<'a>(icon: &str) -> Result<String, String> {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut buf = String::new();

  print!("\n{} ", icon);

  io::Write::flush(&mut stdout).expect("flush failed!");
  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  buf.truncate(buf.trim_end().len());

  Ok(buf)
}
