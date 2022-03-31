use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use super::interface::{LOG_INCOGNITO, LOG_NAME};

pub fn read_line(icon: &str) -> Result<String, String> {
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut buf = String::new();

  print!("{} ", icon);
  stdout.flush().unwrap();
  buf.clear();

  match stdin.read_line(&mut buf) {
    Ok(_) => Ok(buf),
    Err(e) => Err(format!("{}", e)),
  }
}

pub fn read_file(pathname: &str) -> Result<String, io::Error> {
  let path = Path::new(pathname);

  fs::read_to_string(path)
}

pub fn username() -> String {
  std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
}
