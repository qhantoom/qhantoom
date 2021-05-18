use crate::util::span::{Span, SPAN_ZERO};

use super::interface::TokenKind;

pub const TOKEN_ZERO: Token = Token::zero();

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  kind: TokenKind,
  span: Span,
}

impl Token {
  #[inline]
  pub const fn zero() -> Self {
    Self {
      kind: TokenKind::EOF,
      span: SPAN_ZERO,
    }
  }

  #[inline]
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &TokenKind {
    &self.kind
  }

  #[inline]
  pub fn span(&self) -> Span {
    self.span
  }

  #[inline]
  pub fn is_eof(&self) -> bool {
    self.kind.is_eof()
  }

  #[inline]
  pub fn is(&self, kind: TokenKind) -> bool {
    self.kind.is(kind)
  }
}
