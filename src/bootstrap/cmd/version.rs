use crate::util::interface::VERSION;

#[inline]
pub fn run() {
  version()
}

#[inline]
fn version() {
  print!("\nv{}\n", VERSION);
}
