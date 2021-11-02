use crate::front::tokenizer::token::TokenKind;

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  pub kind: TyKind,
}

impl Ty {
  #[inline]
  pub const fn new(kind: TyKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub fn kind(&self) -> &TyKind {
    &self.kind
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
  S8,
  S16,
  S32,
  S64,
  SInt,
  U8,
  U16,
  U32,
  U64,
  UInt,
  F32,
  F64,
  Bool,
  Str,
  Char,
  Void,
  Dynamic,
  Array(Box<Ty>),
  Fun(Vec<Box<Ty>>, Box<Ty>),
}

impl From<&TokenKind> for TyKind {
  #[inline]
  fn from(kind: &TokenKind) -> Self {
    match kind {
      TokenKind::U8 => Self::U8,
      TokenKind::U16 => Self::U16,
      TokenKind::U32 => Self::U32,
      TokenKind::U64 => Self::U64,
      TokenKind::UInt => Self::UInt,
      TokenKind::S8 => Self::S8,
      TokenKind::S16 => Self::S16,
      TokenKind::S32 => Self::S32,
      TokenKind::S64 => Self::S64,
      TokenKind::SInt => Self::SInt,
      TokenKind::F32 => Self::F32,
      TokenKind::F64 => Self::F64,
      TokenKind::Bool => Self::Bool,
      TokenKind::Char => Self::Char,
      TokenKind::Str => Self::Str,
      TokenKind::Void => Self::Void,
      _ => unreachable!(),
    }
  }
}

#[inline]
pub const fn mk_ty(kind: TyKind) -> Ty {
  Ty::new(kind)
}
