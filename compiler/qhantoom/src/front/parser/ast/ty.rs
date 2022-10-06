use super::ast::{Expr, ExprKind};
use super::pbox::PBox;

use crate::util::span::Span;

pub trait AsTy: Sized {
  fn as_ty(&self) -> PBox<Ty>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  pub kind: TyKind,
  pub span: Span,
}

impl Ty {
  pub const STR: Self = Self::new(TyKind::Str, Span::ZERO);

  pub const VOID: Self = Self::new(TyKind::Void, Span::ZERO);

  pub const fn new(kind: TyKind, span: Span) -> Self {
    Self { kind, span }
  }

  pub const fn with_void(span: Span) -> Self {
    Self::new(TyKind::Void, span)
  }

  pub const fn with_bool(span: Span) -> Self {
    Self::new(TyKind::Bool, span)
  }

  pub const fn with_uint(span: Span) -> Self {
    Self::new(TyKind::UInt, span)
  }

  pub const fn with_f64(span: Span) -> Self {
    Self::new(TyKind::F64, span)
  }

  pub const fn with_str(span: Span) -> Self {
    Self::new(TyKind::Str, span)
  }

  pub const fn with_fn(
    args: Vec<PBox<Ty>>,
    return_ty: PBox<Ty>,
    span: Span,
  ) -> Self {
    Self::new(TyKind::Fn(args, return_ty), span)
  }
}

impl From<PBox<Expr>> for Ty {
  fn from(expr: PBox<Expr>) -> Self {
    let kind = if let ExprKind::Identifier(identifier) = &expr.kind {
      match identifier.as_str() {
        "uint" => TyKind::UInt,
        _ => panic!("from ty error"),
      }
    } else {
      TyKind::Void
    };

    Ty::new(kind, expr.span)
  }
}

#[derive(Clone, Debug)]
pub enum TyKind {
  Void,
  Bool,
  U8,
  U16,
  U32,
  U64,
  UInt,
  S8,
  S16,
  S32,
  S64,
  SInt,
  F32,
  F64,
  Str,
  Fn(Vec<PBox<Ty>>, PBox<Ty>),
}

impl PartialEq for TyKind {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Void, Self::Void)
      | (Self::Bool, Self::Bool)
      | (Self::U8, Self::U8)
      | (Self::U16, Self::U16)
      | (Self::U32, Self::U32)
      | (Self::U64, Self::U64)
      | (Self::UInt, Self::UInt)
      | (Self::S8, Self::S8)
      | (Self::S16, Self::S16)
      | (Self::S32, Self::S32)
      | (Self::S64, Self::S64)
      | (Self::SInt, Self::SInt)
      | (Self::F32, Self::F32)
      | (Self::F64, Self::F64)
      | (Self::Str, Self::Str) => true,
      (Self::Fn(_, lhs_return_ty), Self::Fn(_, rhs_return_ty)) => {
        lhs_return_ty == rhs_return_ty
      }
      _ => false,
    }
  }
}
