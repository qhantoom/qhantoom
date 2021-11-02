#[no_mangle]
#[inline]
pub extern "C" fn print_builtin(value: isize) {
  print!("{}\n", value);
}

use std::ffi;

#[no_mangle]
pub fn print_str_builtin(s: *const i8) {
  let s = unsafe { ffi::CStr::from_ptr(s) };
  print!("{}", s.to_str().unwrap());
}

/*
let str_ptr = if let Some(str_ptr) = self.global_env.get(&s) {
    *str_ptr
} else {
    let label = format!(".LS{}", s.as_usize());
    let data_id = self
        .module
        .declare_data(&label, Linkage::Local, false, None)
        .unwrap();
    let mut data_ctx = DataContext::new();
    data_ctx.define(s.as_str().to_owned().into_boxed_str().into());
    self.module.define_data(data_id, &data_ctx).unwrap();
    let str_ptr = self.module.declare_data_in_func(data_id, self.builder.func);
    self.global_env.insert(s, str_ptr);
    str_ptr
};
self.builder.ins().global_value(self.ptr_type, str_ptr)
*/
