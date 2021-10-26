#[no_mangle]
#[inline]
pub extern "C" fn print_builtin(value: isize) {
  print!("{}\n", value);
}
