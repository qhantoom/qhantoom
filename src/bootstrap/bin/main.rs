use bootstrap::cmd::Cmd;

fn main() {
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  Cmd::run(args);
}
