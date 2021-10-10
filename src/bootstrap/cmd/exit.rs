// run the `exit` command
#[inline]
pub fn run() {
  exit();
}

// abort the program
#[inline]
pub fn exit() -> ! {
  print!("\nTriForce.. 👋\n");
  std::process::exit(0);
}
