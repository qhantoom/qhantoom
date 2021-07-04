pub use self::Precedence::*;

use crate::front::tokenizer::TokenKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Precedence {
  Lowest,
  Assignement,
  Conditional,
  Sum,
  Exponent,
  Unary,
  Calling,
  Index,
  Highest,
}

impl From<&TokenKind> for Precedence {
  fn from(kind: &TokenKind) -> Precedence {
    match *kind {
      TokenKind::Mul | TokenKind::Div => Self::Exponent,
      TokenKind::Add | TokenKind::Sub => Self::Sum,
      TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => {
        Self::Conditional
      }
      TokenKind::Eq | TokenKind::Bang => Self::Assignement,
      TokenKind::OpenParen => Self::Calling,
      TokenKind::OpenBracket => Self::Index,
      _ => Self::Lowest,
    }
  }
}
