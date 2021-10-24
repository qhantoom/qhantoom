#[no_mangle]
#[inline]
pub extern "C" fn print_builtin(value: isize) -> isize {
  print!("\n{}\n", value);
  print!("hello, world");
  0
}
