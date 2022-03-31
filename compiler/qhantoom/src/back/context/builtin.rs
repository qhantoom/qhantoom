use std::ffi;

pub fn print_builtin(value: isize) {
  print!("{}\n", value);
}

pub fn print_str_builtin(s: *const i8) {
  print!("{}", cstr_from_ptr(s));
}

fn cstr_from_ptr(s: *const i8) -> &'static str {
  let s = unsafe { ffi::CStr::from_ptr(s) };
  s.to_str().unwrap()
}
