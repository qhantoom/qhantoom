use bootstrap::cmd::cmd;

#[inline]
fn main() {
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  cmd::run(args);
}
