mod codegen;
pub mod cranelift;
mod interface;
pub mod llvm;

#[cfg(test)]
mod tests;

pub use codegen::{codegen_with_cranelift, codegen_with_llvm, Codegen};
pub use interface::CodeGenerator;
