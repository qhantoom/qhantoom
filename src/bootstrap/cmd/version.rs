// run the version command
#[inline]
pub fn run() {
  print!("\nv{}\n", version());
}

// display the current version
#[inline]
pub fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}
