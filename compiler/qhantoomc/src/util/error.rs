use std::fmt;
use std::io;
use std::num::ParseFloatError;
use std::result;

use cranelift::prelude::isa::LookupError;
use cranelift_codegen::CodegenError;
use cranelift_module::ModuleError;
use cranelift_object::object;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Custom(&'static str),
  CraneliftCodegen(CodegenError),
  CraneLiftIsa(LookupError),
  CraneliftModule(ModuleError),
  CraneliftObject(object::write::Error),
  FunctionRedef,
  FunctionRedefWithDifferentParams,
  Io(io::Error),
  ParseFloat(ParseFloatError),
  Undefined(&'static str),
  ExpectedExpr(&'static str, String),
  Unexpected(&'static str),
  UnexpectedCharacter(char),
  UnexpectedEscapeSequence(char),
  UnexpectedLiteralChar(String),
  UnexpectedLiteralNumber(char),
  WrongArgumentCount,
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Self::Custom(ref msg) => write!(f, "{}", msg),
      Self::CraneliftCodegen(ref error) => write!(f, "{}", error),
      Self::CraneLiftIsa(ref error) => write!(f, "{}", error),
      Self::CraneliftModule(ref error) => write!(f, "{}", error),
      Self::CraneliftObject(ref error) => write!(f, "{}", error),
      Self::FunctionRedef => write!(f, "redefinition of function"),
      Self::FunctionRedefWithDifferentParams => write!(
        f,
        "redefinition of function with different number of parameters"
      ),
      Self::Io(ref error) => write!(f, "{}", error),
      Self::ParseFloat(ref error) => write!(f, "{}", error),
      Self::Undefined(ref msg) => write!(f, "undefined {}", msg),
      Self::ExpectedExpr(ref expected, ref found) => write!(
        f,
        "expected {} literal expression, found: {}",
        expected, found
      ),
      Self::Unexpected(ref msg) => write!(f, "unexpected {}", msg),
      Self::UnexpectedCharacter(ref ch) => {
        write!(f, "unknown character: `{}`", ch)
      }
      Self::UnexpectedEscapeSequence(ref ch) => {
        write!(f, "unexpected escape sequence: {}", ch)
      }
      Self::UnexpectedLiteralChar(ref buf) => {
        write!(f, "unexpected char literal: {}", buf)
      }
      Self::UnexpectedLiteralNumber(ref buf) => {
        write!(f, "unexpected number literal (leading zero): {}", buf)
      }
      Self::WrongArgumentCount => write!(f, "wrong argument count"),
    }
  }
}

impl From<CodegenError> for Error {
  #[inline]
  fn from(error: CodegenError) -> Self {
    Self::CraneliftCodegen(error)
  }
}

impl From<io::Error> for Error {
  #[inline]
  fn from(error: io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<LookupError> for Error {
  #[inline]
  fn from(error: LookupError) -> Self {
    Self::CraneLiftIsa(error)
  }
}

impl From<ModuleError> for Error {
  #[inline]
  fn from(error: ModuleError) -> Self {
    Self::CraneliftModule(error)
  }
}

impl From<object::write::Error> for Error {
  #[inline]
  fn from(error: object::write::Error) -> Self {
    Self::CraneliftObject(error)
  }
}

impl From<ParseFloatError> for Error {
  #[inline]
  fn from(error: ParseFloatError) -> Self {
    Self::ParseFloat(error)
  }
}
