pub mod runtime;
pub mod value;
mod interpreter;

#[cfg(test)]
mod tests;

pub use interpreter::{interpret, Interpreter};
