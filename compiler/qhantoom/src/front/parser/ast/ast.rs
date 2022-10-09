use super::pbox::{pbox, PBox};
use super::ty::{AsTy, Ty};

use crate::util::error::Reporter;
use crate::util::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum Public {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Async {
  Yes(Span),
  No,
}

#[derive(Debug)]
pub enum Mutability {
  Not,
  Yes,
}

#[derive(Debug)]
pub struct BindingAnnotation(pub Mutability);

#[derive(Debug)]
pub struct Program {
  pub items: Vec<PBox<Item>>,
  pub reporter: Reporter,
}

impl Program {
  pub fn new(items: Vec<PBox<Item>>, reporter: Reporter) -> Self {
    Self { items, reporter }
  }
}

#[derive(Debug)]
pub struct Load {
  pub path_view: PathView,
  pub span: Span,
}

impl Load {
  pub fn new(path_view: PathView, span: Span) -> Self {
    Self { path_view, span }
  }
}

#[derive(Debug)]
pub struct PathView {
  pub kind: PathViewKind,
  pub span: Span,
}

impl PathView {
  pub fn new(kind: PathViewKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum PathViewKind {
  Identifier(PBox<Expr>),
  Path(PBox<PathView>, Vec<PBox<Expr>>),
}

#[derive(Debug)]
pub struct Pattern {
  pub kind: PatternKind,
  pub span: Span,
}

impl Pattern {
  pub fn new(kind: PatternKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum PatternKind {
  Underscore,
  Identifier(BindingAnnotation, PBox<Expr>),
  Lit(PBox<Expr>),
}

#[derive(Debug)]
pub struct Item {
  pub kind: ItemKind,
  pub span: Span,
}

impl Item {
  pub fn new(kind: ItemKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum ItemKind {
  Load(PBox<Load>),
  Ext(PBox<Ext>),
  Val(PBox<Decl>),
  Fun(PBox<Fun>),
}

#[derive(Debug)]
pub struct Ext {
  pub public: Public,
  pub prototype: Prototype,
  pub body: Option<PBox<Block>>,
  pub span: Span,
}

impl Ext {
  pub fn new(
    public: Public,
    prototype: Prototype,
    body: Option<PBox<Block>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      prototype,
      body,
      span,
    }
  }
}

#[derive(Debug)]
pub struct Decl {
  pub mutability: Mutability,
  pub kind: DeclKind,
  pub pattern: Pattern,
  pub ty: PBox<Ty>,
  pub value: PBox<Expr>,
  pub span: Span,
}

impl Decl {
  pub fn new(
    mutability: Mutability,
    kind: DeclKind,
    pattern: Pattern,
    ty: PBox<Ty>,
    value: PBox<Expr>,
    span: Span,
  ) -> Self {
    Self {
      mutability,
      kind,
      pattern,
      ty,
      value,
      span,
    }
  }
}

#[derive(Debug)]
pub enum DeclKind {
  Val,
  Imu,
  Mut,
}

#[derive(Debug)]
pub struct Fun {
  pub prototype: Prototype,
  pub body: PBox<Block>,
  pub span: Span,
}

impl Fun {
  pub fn new(prototype: Prototype, body: PBox<Block>, span: Span) -> Self {
    Self {
      prototype,
      body,
      span,
    }
  }

  pub fn as_inputs_tys(&self) -> Vec<PBox<Ty>> {
    self.prototype.as_inputs_tys()
  }
}

impl AsTy for Fun {
  fn as_ty(&self) -> PBox<Ty> {
    self.prototype.as_ty()
  }
}

#[derive(Debug)]
pub struct Prototype {
  pub name: PBox<Expr>,
  pub inputs: Vec<PBox<Arg>>,
  pub output: ReturnTy,
}

impl Prototype {
  pub fn new(
    name: PBox<Expr>,
    inputs: Vec<PBox<Arg>>,
    output: ReturnTy,
  ) -> Self {
    Self {
      name,
      inputs,
      output,
    }
  }

  pub fn as_inputs_tys(&self) -> Vec<PBox<Ty>> {
    self
      .inputs
      .iter()
      .map(|input| input.ty.to_owned())
      .collect::<Vec<_>>()
  }
}

impl AsTy for Prototype {
  fn as_ty(&self) -> PBox<Ty> {
    self.output.as_ty()
  }
}

#[derive(Debug)]
pub struct Arg {
  pub pattern: Pattern,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl Arg {
  pub fn new(pattern: Pattern, ty: PBox<Ty>, span: Span) -> Self {
    Self { pattern, ty, span }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnTy {
  Default(Span),
  Ty(PBox<Ty>),
}

impl AsTy for ReturnTy {
  fn as_ty(&self) -> PBox<Ty> {
    match self {
      Self::Ty(ty) => ty.clone(),
      Self::Default(span) => pbox(Ty::with_void(*span)),
    }
  }
}

#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<PBox<Stmt>>,
  pub span: Span,
}

impl Block {
  pub fn new(stmts: Vec<PBox<Stmt>>, span: Span) -> Self {
    Self { stmts, span }
  }
}

#[derive(Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl Stmt {
  pub fn new(kind: StmtKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum StmtKind {
  Item(PBox<Item>),
  Decl(PBox<Decl>),
  Expr(PBox<Expr>),
}

#[derive(Debug)]
pub struct Expr {
  pub kind: ExprKind,
  pub span: Span,
}

impl Expr {
  pub fn new(kind: ExprKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum ExprKind {
  Lit(PBox<Lit>),
  Identifier(String),
  Call(PBox<Expr>, Vec<PBox<Expr>>),
  UnOp(UnOp, PBox<Expr>),
  BinOp(PBox<Expr>, BinOp, PBox<Expr>),
  Assign(PBox<Expr>, BinOp, PBox<Expr>),
  AssignOp(PBox<Expr>, BinOp, PBox<Expr>),
  Return(Option<PBox<Expr>>),
  Block(PBox<Block>),
  Loop(PBox<Block>),
  While(PBox<Expr>, PBox<Block>),
}

#[derive(Debug)]
pub struct Lit {
  pub kind: LitKind,
  pub span: Span,
}

impl Lit {
  pub fn new(kind: LitKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug)]
pub enum LitKind {
  Bool(bool),
  Int(i64),
  Float(f64),
  Str(String),
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Debug)]
pub enum BinOpKind {
  Add,    // +
  Sub,    // -
  Mul,    // *
  Div,    // /
  Rem,    // %
  And,    // &&
  Or,     // ||
  Lt,     // <
  Gt,     // >
  Le,     // <=
  Ge,     // >=
  Eq,     // ==
  Ne,     // !=
  Shl,    // <<
  Shr,    // >>
  BitAnd, // &
  BitOr,  // |
  BitXor, // ^
  As,     // as
  Range,  // ..
}

impl BinOpKind {
  pub fn is_assign_op(&self) -> bool {
    matches!(
      self,
      Self::Add
        | Self::Sub
        | Self::Mul
        | Self::Div
        | Self::Rem
        | Self::BitXor
        | Self::BitAnd
        | Self::BitOr
    )
  }
}

pub type UnOp = Spanned<UnOpKind>;

#[derive(Debug)]
pub enum UnOpKind {
  Not,
  Neg,
}
