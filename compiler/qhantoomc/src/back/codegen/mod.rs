mod codegen;
pub mod cranelift;
mod interface;
pub mod llvm;

#[cfg(test)]
mod tests;

pub use codegen::{codegen_with_llvm, codegen_with_cranelift, Codegen};
pub use interface::CodeGenerator;
