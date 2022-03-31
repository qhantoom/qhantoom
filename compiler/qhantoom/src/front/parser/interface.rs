use crate::front::tokenizer::token::TokenKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Precedence {
  Lowest,
  Assignment,
  LOr,
  LAnd,
  Conditional,
  BitOr,
  BitXor,
  BitAnd,
  Shift,
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
      TokenKind::Dot => Precedence::Highest,

      TokenKind::OpenBracket => Self::Index,

      TokenKind::OpenParen => Self::Calling,

      TokenKind::Not => Self::Unary,

      TokenKind::Mul | TokenKind::Div | TokenKind::Mod => Self::Exponent,

      TokenKind::Add | TokenKind::Sub => Self::Sum,

      TokenKind::Shl | TokenKind::Shr => Self::Shift,

      TokenKind::And => Self::BitAnd,

      TokenKind::Caret => Self::BitXor,

      TokenKind::Or => Self::BitOr,

      TokenKind::Lt
      | TokenKind::Gt
      | TokenKind::Le
      | TokenKind::Ge
      | TokenKind::Eq
      | TokenKind::Ne => Self::Conditional,

      TokenKind::AndAnd => Self::LAnd,

      TokenKind::OrOr => Self::LOr,

      TokenKind::Assign
      | TokenKind::AddAssign
      | TokenKind::SubAssign
      | TokenKind::MulAssign
      | TokenKind::RemAssign
      | TokenKind::CaretAssign
      | TokenKind::BitAndAssign
      | TokenKind::BitOrAssign => Self::Assignment,

      _ => Self::Lowest,
    }
  }
}
