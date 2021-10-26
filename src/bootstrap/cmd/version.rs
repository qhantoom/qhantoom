use super::cmd::DEFAULT_VERSION;

// run the `version` command
#[inline]
pub fn run() {
  print!("\nv{}\n", version());
}

// get the current version from `cargo.toml`
#[inline]
pub fn version() -> &'static str {
  DEFAULT_VERSION
}
