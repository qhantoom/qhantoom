pub mod ast;
mod interface;
mod parser;

#[cfg(test)]
mod tests;

pub use parser::{parse, Parser};
