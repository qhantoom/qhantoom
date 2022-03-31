use super::interface::TokenKind;

pub const TOKEN_EOF: Token = Token::eof();

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  kind: TokenKind,
}

impl Token {
  pub const fn eof() -> Self {
    Self {
      kind: TokenKind::EOF,
    }
  }

  pub const fn new(kind: TokenKind) -> Self {
    Self { kind }
  }

  pub fn is(&self, k: &TokenKind) -> bool {
    self.kind.is(k)
  }

  pub fn is_eof(&self) -> bool {
    self.kind.is_eof()
  }

  pub fn kind(&self) -> &TokenKind {
    &self.kind
  }
}
