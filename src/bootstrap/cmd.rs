pub struct Cmd;

impl Cmd {
  pub fn run(args: Vec<String>) {
    let cmds = args.clone();

    if cmds.contains(&"repl".into()) {
      match crate::repl::run(&cmds) {
        Ok(_) => return,
        Err(_) => panic!("repl error"),
      }
    }

    if cmds.contains(&"compile".into()) {
      match crate::compiler::run(cmds) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(2),
      }
    }
  }
}
