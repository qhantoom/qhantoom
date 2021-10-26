use crate::util;

// run the license command
#[inline]
pub fn run() {
  license();
}

// print the `LICENSE` if found
#[inline]
pub fn license() {
  match util::read_file("LICENSE") {
    Ok(s) => print!("\n{}\n", s),
    Err(_) => print!("License not found"),
  }
}
