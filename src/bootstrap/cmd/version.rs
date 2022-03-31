use crate::util::interface::VERSION;

pub fn run() {
  version()
}

fn version() {
  print!("\nv{}\n", VERSION);
}
