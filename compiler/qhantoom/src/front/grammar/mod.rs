// allow internal warnings related to the implementation of lalrpop
#![allow(clippy::clone_on_copy)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::unused_unit)]

lalrpop_mod!(grammar, "/front/grammar/grammar.rs");

pub use super::grammar::grammar::ProgramParser;
