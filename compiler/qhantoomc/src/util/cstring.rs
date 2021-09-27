pub macro cstring($s:expr) {
  format!("{}{}", $s, "\0").as_ptr() as *const i8
}

pub macro cstring_mut($s:expr) {
  format!("{}{}", $s, "\0").as_ptr() as *mut i8
}

pub macro cstring_mut_mut($s:expr) {
  format!("{}{}", $s, "\0").as_ptr() as *mut *mut i8
}