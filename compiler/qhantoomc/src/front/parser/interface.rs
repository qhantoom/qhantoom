use crate::front::tokenizer::token::TokenKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Precedence {
  Lowest,
  Assignment,
  LOr,
  LAnd,
  Conditional,
  Sum,
  Exponent,
  Unary,
  Calling,
  Index,
  // Highest,
}

impl From<&TokenKind> for Precedence {
  #[inline]
  fn from(kind: &TokenKind) -> Precedence {
    match *kind {
      TokenKind::OpenBracket => Self::Index,

      TokenKind::OpenParen => Self::Calling,

      TokenKind::Add | TokenKind::Sub => Self::Sum,

      TokenKind::Lt
      | TokenKind::Gt
      | TokenKind::Le
      | TokenKind::Ge
      | TokenKind::Equal
      | TokenKind::NotAssign => Self::Conditional,

      TokenKind::Mul | TokenKind::Div | TokenKind::Mod => Self::Exponent,

      TokenKind::AndAnd => Self::LAnd,

      TokenKind::PipePipe => Self::LOr,

      TokenKind::Assign | TokenKind::Bang => Self::Assignment,

      _ => Self::Lowest,
    }
  }
}
