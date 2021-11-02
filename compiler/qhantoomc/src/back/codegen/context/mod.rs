mod builtin;
mod scope;

pub use builtin::{print_builtin, print_char_builtin, print_str_builtin};
pub use scope::ScopeMap;
