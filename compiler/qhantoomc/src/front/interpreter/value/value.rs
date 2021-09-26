use std::fmt;

use crate::front::interpreter::runtime::Scope;
use crate::front::parser::ast;

pub const VALUE_VOID: Value = Value::void();

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Void,
  Bool(bool),
  Int(i32),
  Float(f32),
  Char(char),
  Str(String),
  Ident(String),
  Return(Box<Value>),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Void => write!(f, ""),
      Self::Bool(ref expr) => write!(f, "{}", expr),
      Self::Int(ref expr) => write!(f, "{}", expr),
      Self::Float(ref expr) => write!(f, "{}", expr),
      Self::Char(ref expr) => write!(f, "{}", expr),
      Self::Str(ref expr) => write!(f, "\"{}\"", expr),
      Self::Ident(ref expr) => write!(f, "{}", expr),
      Self::Return(ref expr) => write!(f, "{}", expr),
    }
  }
}

impl Value {
  #[inline]
  pub const fn void() -> Self {
    Self::Void
  }

  #[inline]
  pub fn kind(&self) -> &Self {
    &self
  }

  #[inline]
  pub fn as_bool(&self) -> bool {
    match self {
      Self::Bool(ref value) => *value,
      _ => true,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
  pub name: Box<Value>,
  pub args: Vec<Box<Value>>,
  pub ty: Box<Ty>,
  pub block: Vec<Box<Value>>,
  pub scope: Option<Scope>,
}

impl Fun {
  #[inline]
  pub fn name(&self) -> String {
    match *self.name {
      Value::Ident(ref expr) => expr.into(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
  pub name: Box<Value>,
  pub ty: Box<Ty>,
  pub value: Box<Value>,
}

impl Local {
  #[inline]
  pub fn name(&self) -> String {
    match *self.name {
      Value::Ident(ref expr) => expr.into(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
  Void,
  S8,
  S16,
  S32,
  S64,
  SInt,
  U8,
  U16,
  U32,
  U64,
  F32,
  F64,
  UInt,
  Bool,
  Char,
  Str,
}

impl fmt::Display for Ty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::S8 => write!(f, "s8"),
      Self::S16 => write!(f, "s16"),
      Self::S32 => write!(f, "s32"),
      Self::S64 => write!(f, "s64"),
      Self::SInt => write!(f, "sint"),
      Self::U8 => write!(f, "u8"),
      Self::U16 => write!(f, "u16"),
      Self::U32 => write!(f, "u32"),
      Self::U64 => write!(f, "u64"),
      Self::UInt => write!(f, "uint"),
      Self::F32 => write!(f, "f32"),
      Self::F64 => write!(f, "f64"),
      Self::Bool => write!(f, "bool"),
      Self::Char => write!(f, "char"),
      Self::Str => write!(f, "str"),
    }
  }
}

impl From<&Box<ast::Ty>> for Ty {
  fn from(rhs: &Box<ast::Ty>) -> Self {
    match rhs.kind() {
      ast::TyKind::Void => Ty::Void,
      ast::TyKind::S8 => Ty::S8,
      ast::TyKind::S16 => Ty::S16,
      ast::TyKind::S32 => Ty::S32,
      ast::TyKind::S64 => Ty::S64,
      ast::TyKind::SInt => Ty::SInt,
      ast::TyKind::U8 => Ty::U8,
      ast::TyKind::U16 => Ty::U16,
      ast::TyKind::U32 => Ty::U32,
      ast::TyKind::U64 => Ty::U64,
      ast::TyKind::UInt => Ty::UInt,
      ast::TyKind::F32 => Ty::F32,
      ast::TyKind::F64 => Ty::F64,
      ast::TyKind::Bool => Ty::Bool,
      ast::TyKind::Char => Ty::Char,
      ast::TyKind::Str => Ty::Str,
      _ => todo!(),
    }
  }
}
