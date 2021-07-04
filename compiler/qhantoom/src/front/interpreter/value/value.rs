use crate::front::parser::ast;

use crate::util::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
  kind: ValueKind,
}

impl Value {
  #[inline]
  pub const fn new(kind: ValueKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub fn kind(&self) -> &ValueKind {
    &self.kind
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueKind {
  Void,
  Shebang(String),
  Comment(CommentKind, String),
  Capsule(Box<Value>),
  Mod(Box<Value>),
  Fun(Box<Fun>),
  Local(Box<Local>),
  Ret(Box<Value>),
  Bool(bool),
  Char(char),
  Float(f32),
  Int(i32),
  Str(String),
  Closure(Box<Fun>),
  Array(Vec<Box<Value>>),
  Ident(Symbol),
  Loop(LoopKind),
  Binop {
    lhs: Box<Value>,
    op: BinopKind,
    rhs: Box<Value>,
  },
  Unop {
    op: UnopKind,
    rhs: Box<Value>,
  },
  Index {
    rhs: Box<Value>,
    lhs: Box<Value>,
  },
  If {
    condition: Box<Value>,
    consequence: Box<Block>,
    alternative: Option<Box<Block>>,
  },
  Call {
    callee: Box<Value>,
    args: Vec<Box<Value>>,
  },
  Hash {
    data: Vec<(Box<HashKind>, Box<Value>)>,
  },
  Ty(TyKind),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Capsule {
  pub module: Mod,
}

impl Capsule {
  #[inline]
  pub fn new(module: Mod) -> Self {
    Self { module }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mod {
  values: Vec<Box<Value>>,
}

impl Mod {
  #[inline]
  pub fn new(values: Vec<Box<Value>>) -> Self {
    Self { values }
  }

  #[inline]
  pub fn values(&mut self) -> &Vec<Box<Value>> {
    &self.values
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub stmts: Vec<Box<Value>>,
}

impl Block {
  #[inline]
  pub fn new(stmts: Vec<Box<Value>>) -> Self {
    Self { stmts }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
  name: Box<Value>,
  pub args: Vec<(Box<Value>, Box<Ty>)>,
  pub ty: Box<Ty>,
  pub block: Box<Value>, // box Block
}

impl Fun {
  #[inline]
  pub fn new(
    name: Box<Value>,
    args: Vec<(Box<Value>, Box<Ty>)>,
    ty: Box<Ty>,
    block: Box<Value>,
  ) -> Self {
    Self {
      name,
      args,
      ty,
      block,
    }
  }

  #[inline]
  pub fn name(&self) -> String {
    match self.name.kind() {
      ValueKind::Ident(ref sym) => sym.to_string(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
  name: Box<Value>,
  ty: Box<Ty>,
  value: Box<Value>,
}

impl Local {
  #[inline]
  pub fn new(
    name: Box<Value>,
    ty: Box<Ty>,
    value: Box<Value>,
  ) -> Self {
    Self {
      name,
      ty,
      value,
    }
  }

  #[inline]
  pub fn name(&self) -> String {
    match self.name.kind() {
      ValueKind::Ident(ref sym) => sym.to_string(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommentKind {}

#[derive(Clone, Debug, PartialEq)]
pub enum HashKind {}

#[derive(Clone, Debug, PartialEq)]
pub enum LoopKind {}

#[derive(Clone, Debug, PartialEq)]
pub enum BinopKind {}

#[derive(Clone, Debug, PartialEq)]
pub enum UnopKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  kind: TyKind,
}

impl From<&Box<ast::Ty>> for Ty {
  fn from(rhs: &Box<ast::Ty>) -> Self {
    // use crate::front::parser::ast::TyKind::*;

    let kind = match rhs.kind() {
      _ => TyKind::Void,
    };

    Self { kind }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
  Void,
}

#[inline]
pub fn make_capsule(capsule: Box<Value>) -> Box<Value> {
  box Value::new(ValueKind::Capsule(capsule))
}

#[inline]
pub fn make_mod(value: Box<Value>) -> Box<Value> {
  box Value::new(ValueKind::Mod(value))
}

#[inline]
pub fn make_void_obj() -> Box<Value> {
  box Value::new(ValueKind::Void)
}

#[inline]
pub fn make_obj(kind: ValueKind) -> Box<Value> {
  box Value::new(kind)
}

#[inline]
pub fn make_binop(
  rhs: Box<Value>,
  op: BinopKind,
  lhs: Box<Value>,
) -> Box<Value> {
  box Value::new(ValueKind::Binop { rhs, op, lhs })
}

#[inline]
pub fn make_ret(value: Box<Value>) -> Box<Value> {
  box Value::new(ValueKind::Ret(value))
}

#[inline]
pub fn make_fun(fun: Box<Fun>) -> Box<Value> {
  box Value::new(ValueKind::Fun(fun))
}

#[inline]
pub fn make_local(local: Box<Local>) -> Box<Value> {
  box Value::new(ValueKind::Local(local))
}

#[inline]
pub fn make_ident(sym: Symbol) -> Box<Value> {
  box Value::new(ValueKind::Ident(sym))
}

#[inline]
pub fn make_array(value: Vec<Box<Value>>) -> Box<Value> {
  box Value::new(ValueKind::Array(value))
}

#[inline]
pub fn make_int(int: i32) -> Box<Value> {
  box Value::new(ValueKind::Int(int))
}

#[inline]
pub fn make_ty(ty: &Box<ast::Ty>) -> Box<Ty> {
  box ty.into()
}

#[inline]
pub fn unwrap(value: &Box<Value>) -> Box<Value> {
  match value.kind() {
    ValueKind::Ret(ref v) => v.to_owned(),
    k => make_obj(k.to_owned()),
  }
}
