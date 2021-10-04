#[inline]
pub fn run() {
  license();
}

#[inline]
pub fn license() {
  match crate::util::readfile("LICENSE") {
    Ok(s) => print!("{}\n", s),
    Err(_) => println!("License not found"),
  }
}
