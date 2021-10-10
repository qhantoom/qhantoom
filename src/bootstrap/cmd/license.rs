// run the `license` command
#[inline]
pub fn run() {
  license();
}

// print the license if found
#[inline]
pub fn license() {
  match crate::util::readfile("LICENSE") {
    Ok(s) => print!("\n{}\n", s),
    Err(_) => print!("License not found"),
  }
}
