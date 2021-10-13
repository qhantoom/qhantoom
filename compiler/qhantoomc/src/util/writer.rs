use std::fs::File;
use std::io::Error;
use std::io::Write;

// write buffer to file
#[inline]
pub fn write(filename: &str, buffer: Vec<u8>) -> Result<(), Error> {
  let mut file = File::create(filename)?;

  file.write_all(&buffer)
}
