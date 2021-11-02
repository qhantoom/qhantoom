use std::ffi;

#[no_mangle]
#[inline]
pub extern "C" fn print_builtin(value: isize) {
  print!("{}\n", value);
}

#[inline]
#[no_mangle]
pub extern "C" fn print_str_builtin(s: *const i8) {
  print!("{}", cstr_from_ptr(s));
}

#[inline]
#[no_mangle]
pub extern "C" fn print_char_builtin(s: *const i8) {
  print!("{}", cstr_from_ptr(s));
}

#[inline]
fn cstr_from_ptr(s: *const i8) -> &'static str {
  let s = unsafe { ffi::CStr::from_ptr(s) };
  s.to_str().unwrap()
}
