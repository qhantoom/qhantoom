pub use self::Precedence::*;

use crate::front::tokenizer::token::TokenKind;

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
      TokenKind::Mul | TokenKind::Div | TokenKind::Mod => Self::Exponent,
      TokenKind::Add | TokenKind::Sub => Self::Sum,
      TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => {
        Self::Conditional
      }
      TokenKind::Assign
      | TokenKind::Equal
      | TokenKind::Bang
      | TokenKind::DotDot // no sure about this precedence for range
      | TokenKind::NotAssign => Self::Assignement,
      TokenKind::OpenParen => Self::Calling,
      TokenKind::OpenBracket => Self::Index,
      _ => Self::Lowest,
    }
  }
}
