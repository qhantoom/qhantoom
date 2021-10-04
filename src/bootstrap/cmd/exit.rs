// run the `exit` command
#[inline]
pub fn run() {
  exit();
}

// abort the program
#[inline]
pub fn exit() -> ! {
  println!("\nTriForce.. 👋\n");
  std::process::exit(0);
}
