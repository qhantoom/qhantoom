use std::fs::File;
use std::io::Error;
use std::io::Write;

#[inline]
pub fn write(filename: &str, buf: Vec<u8>) -> Result<(), Error> {
  let mut file = File::create(filename)?;

  file.write_all(&buf)
}
