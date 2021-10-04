// return a c string pointer from a rust string
pub macro cstr($s:expr) {
  format!("{}{}", $s, "\0").as_ptr() as *const i8
}
