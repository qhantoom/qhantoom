use std::process::Command;

fn main() {
  std::thread::spawn(|| {
    Command::new("wasm-pack")
      .args(&["build", "compiler/qhantoom-lab"])
      .args(&["--out-dir", "src/runtime/pkg"])
      .output()
      .expect("failed to execute process");

    // Command::new("yarn")
    //   .args(&["--cwd", "compiler/qhantoom-lab"])
    //   .args(&["build"])
    //   .output()
    //   .expect("failed to execute process");
  }).join().unwrap();
}
