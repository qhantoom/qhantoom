use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

#[inline]
pub fn read_file(pathname: &str) -> Result<String, Error> {
  let path = Path::new(pathname);
  let mut file = File::open(&path)?;
  let mut source = String::new();

  file.read_to_string(&mut source).ok();

  Ok(source)
}
