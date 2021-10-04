use std::fs;
use std::path::Path;

// read a file into a string
#[inline]
pub fn readfile(pathname: &str) -> Result<String, String> {
  let path = Path::new(pathname);

  match fs::read_to_string(path) {
    Ok(file) => Ok(file),
    Err(e) => Err(format!("{}", e)),
  }
}
