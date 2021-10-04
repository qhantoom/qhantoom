// run the `version` command
#[inline]
pub fn run() {
  print!("\nv{}\n", version());
}

// get the current version from cargo.toml
#[inline]
pub fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}
