use crate::front::tokenizer::token::TokenKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Precedence {
  Lowest,
  Assignment,
  LOr,
  LAnd,
  Conditional,
  BitOr,
  BitAnd,
  Sum,
  Exponent,
  Unary,
  Calling,
  Index,
  Highest,
}

impl From<&TokenKind> for Precedence {
  #[inline]
  fn from(kind: &TokenKind) -> Precedence {
    match *kind {
      TokenKind::Dot => Precedence::Highest,

      TokenKind::OpenBracket => Self::Index,

      TokenKind::OpenParen => Self::Calling,

      TokenKind::Bang => Self::Unary,

      TokenKind::Mul | TokenKind::Div | TokenKind::Mod => Self::Exponent,

      TokenKind::Add | TokenKind::Sub => Self::Sum,

      TokenKind::And => Self::BitAnd,

      TokenKind::Pipe => Self::BitOr,

      TokenKind::Lt
      | TokenKind::Gt
      | TokenKind::Le
      | TokenKind::Ge
      | TokenKind::Equal
      | TokenKind::NotAssign => Self::Conditional,

      TokenKind::AndAnd => Self::LAnd,

      TokenKind::PipePipe => Self::LOr,

      TokenKind::Assign
      | TokenKind::AddAssign
      | TokenKind::SubAssign
      | TokenKind::MulAssign
      | TokenKind::ModAssign => Self::Assignment,

      _ => Self::Lowest,
    }
  }
}
