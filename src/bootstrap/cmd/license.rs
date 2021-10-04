// run the `license` command
#[inline]
pub fn run() {
  license();
}

// print the license if found
#[inline]
pub fn license() {
  match crate::util::readfile("LICENSE") {
    Ok(s) => print!("{}\n", s),
    Err(_) => println!("License not found"),
  }
}
