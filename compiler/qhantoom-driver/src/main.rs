use clap::Parser;
use qhantoom_driver::Cmd;

fn main() {
  Cmd::parse().run();
  // return 0;
}
