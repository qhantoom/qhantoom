use super::interface::TokenKind;

pub const TOKEN_EOF: Token = Token::eof();

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  kind: TokenKind,
}

impl Token {
  #[inline]
  pub const fn eof() -> Self {
    Self {
      kind: TokenKind::EOF,
    }
  }

  #[inline]
  pub const fn new(kind: TokenKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub fn is(&self, k: &TokenKind) -> bool {
    self.kind.is(k)
  }

  #[inline]
  pub fn is_eof(&self) -> bool {
    self.kind.is_eof()
  }

  #[inline]
  pub fn kind(&self) -> &TokenKind {
    &self.kind
  }
}
