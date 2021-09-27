mod interpreter;
pub mod runtime;
pub mod value;

#[cfg(test)]
mod tests;

pub use interpreter::{interpret, Interpreter};
