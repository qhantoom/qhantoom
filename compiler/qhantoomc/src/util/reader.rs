use std::fs;
use std::io::Error;
use std::path::Path;

// read a file into a string
#[inline]
pub fn readfile(pathname: &str) -> Result<String, Error> {
  let path = Path::new(pathname);

  fs::read_to_string(path)
}
