use crate::front::tokenizer::{Token, TokenKind};
use crate::util::span::{Span, SPAN_ZERO};
use crate::util::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub struct Capsule {
  pub module: Mod,
  span: Span,
}

impl Capsule {
  #[inline]
  pub const fn new(module: Mod, span: Span) -> Self {
    Self { module, span }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mod {
  items: Vec<Box<Item>>,
  span: Span,
}

impl Mod {
  #[inline]
  pub const fn new(items: Vec<Box<Item>>, span: Span) -> Self {
    Self { items, span }
  }

  #[inline]
  pub fn items(&self) -> &Vec<Box<Item>> {
    &self.items
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.items.len()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub stmts: Vec<Box<Stmt>>,
}

impl Block {
  #[inline]
  pub const fn new(stmts: Vec<Box<Stmt>>) -> Self {
    Self { stmts }
  }

  #[inline]
  pub fn stmts(&self) -> &Vec<Box<Stmt>> {
    &self.stmts
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.stmts.len()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
  pub name: Symbol,
  pub span: Span,
}

impl Ident {
  #[inline]
  pub const fn new(name: Symbol, span: Span) -> Self {
    Self { name, span }
  }

  #[inline]
  pub fn name(&self) -> &Symbol {
    &self.name
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
  pub kind: ItemKind,
  pub span: Span,
}

impl Item {
  #[inline]
  pub const fn new(kind: ItemKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &ItemKind {
    &self.kind
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
  Empty,
  Mod(Box<Mod>),
  Fun(Box<FunDecl>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl Stmt {
  #[inline]
  pub const fn new(kind: StmtKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &StmtKind {
    &self.kind
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
  Empty,
  Break,
  Continue,
  Block { kind: Vec<Box<Stmt>> },
  Fun(Box<FunDecl>),
  Local(Box<Local>),
  Ret(Box<Expr>),
  Expr(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunDecl {
  pub ident: Box<Expr>,
  pub ty: Box<Ty>,
  pub args: Vec<(Box<Expr>, Box<Ty>)>,
  pub block: Box<Stmt>,
}

// impl From<&Box<FunDecl>> for FunDecl {
//   fn from(rhs: &Box<FunDecl>) -> Self {
//     Self {
//       ident: rhs.ident.to_owned(),
//       args: rhs.args.to_owned(),
//       ty: rhs.ty.to_owned(),
//       block: rhs.block.to_owned(),
//     }
//   }
// }

impl FunDecl {
  #[inline]
  pub fn new(
    ident: Box<Expr>,
    args: Vec<(Box<Expr>, Box<Ty>)>,
    ty: Box<Ty>,
    block: Box<Stmt>,
  ) -> Self {
    Self {
      ident,
      args,
      ty,
      block,
    }
  }

  #[inline]
  pub fn name(&self) -> String {
    match self.ident.kind() {
      ExprKind::Ident(ref sym) => sym.to_string(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
  pub ident: Box<Expr>,
  pub immutable: bool,
  pub ty: Box<Ty>,
  pub value: Box<Expr>,
}

impl Local {
  #[inline]
  pub fn name(&self) -> String {
    match self.ident.kind() {
      ExprKind::Ident(ref sym) => sym.to_string(),
      _ => unreachable!(),
    }
  }

  #[inline]
  pub fn value(&self) -> &Box<Expr> {
    &self.value
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
  pub kind: ExprKind,
  pub span: Span,
}

impl Expr {
  #[inline]
  pub const fn new(kind: ExprKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &ExprKind {
    &self.kind
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
  Empty,
  Void,
  Binop {
    lhs: Box<Expr>,
    op: BinopKind,
    rhs: Box<Expr>,
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Box<Expr>>,
  },
  If {
    condition: Vec<Box<Expr>>,
    consequence: Option<Box<Expr>>,
    alternative: Option<Box<Stmt>>,
  },
  Index {
    index: Box<Expr>,
    data: Box<Expr>,
  },
  For {
    iterable: Box<Expr>,
    iterator: Box<Expr>,
    block: Box<Stmt>,
  },
  Loop {
    block: Box<Stmt>,
  },
  While {
    condition: Box<Expr>,
    block: Box<Stmt>,
  },
  Unop {
    operand: UnopKind,
    rhs: Box<Expr>,
  },
  Array(Vec<Box<Expr>>),
  Hash(Vec<(Box<HashKind>, Box<Expr>)>),
  Closure(Box<FunDecl>),
  Ident(Symbol),
  Bool(bool),
  Int(i32),
  Float(f32),
  Char(char),
  Str(String),
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct FunArg {
//   pub name: Box<Expr>,
//   pub immutable: bool,
//   pub expr: Box<Expr>,
//   pub span: Span,
//   pub ty: Ty,
// }

#[derive(Clone, Debug, PartialEq)]
pub enum HashKind {
  Bool(bool),
  Int(i32),
  Str(String),
}

impl From<Box<Expr>> for HashKind {
  fn from(expr: Box<Expr>) -> Self {
    match expr.kind() {
      ExprKind::Bool(ref value) => Self::Bool(*value),
      ExprKind::Int(ref value) => Self::Int(*value),
      ExprKind::Str(ref value) => Self::Str(value.into()),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinopKind {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Lt,
  Le,
  Gt,
  Ge,
  Eq,
  Ne,
}

impl BinopKind {
  pub fn from(token: &Token) -> Self {
    match token.kind() {
      TokenKind::Add => Self::Add,
      TokenKind::Sub => Self::Sub,
      TokenKind::Mul => Self::Mul,
      TokenKind::Div => Self::Div,
      TokenKind::Mod => Self::Mod,
      TokenKind::Lt => Self::Lt,
      TokenKind::Le => Self::Le,
      TokenKind::Gt => Self::Gt,
      TokenKind::Ge => Self::Ge,
      TokenKind::Eq => Self::Eq,
      TokenKind::Ne => Self::Ne,
      k => unreachable!("{:?} is not a operator", k),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  pub kind: TyKind,
  pub span: Span,
}

impl Ty {
  #[inline]
  pub const fn new(kind: TyKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &TyKind {
    &self.kind
  }

  #[inline]
  pub fn span(&self) -> &Span {
    &self.span
  }
}

#[derive(Clone, Debug)]
pub enum TyKind {
  F32,
  F64,
  S8,
  S16,
  S32,
  S64,
  Sint,
  U8,
  U16,
  U32,
  U64,
  Uint,
  Int,
  Bool,
  Str,
  Char,
  Void,
  Array(Box<Ty>),
  Fun(Box<Ty>, Vec<Ty>),
  Struct(Box<Expr>),
}

impl TyKind {
  pub fn as_bool(&self) -> bool {
    match *self {
      Self::Struct(_) => false,
      _ => true,
    }
  }
}

impl PartialEq for TyKind {
  fn eq(&self, rhs: &TyKind) -> bool {
    match (self, rhs) {
      (&Self::Bool, &Self::Bool) | (&Self::Int, &Self::Int) => true,
      (&Self::Array(ref t1), &Self::Array(ref t2)) => *t1 == *t2,
      (&Self::Struct(ref s1), &Self::Struct(ref s2)) => s1 == s2,
      (&Self::Fun(ref t1, ref l1), &Self::Fun(ref t2, ref l2)) => {
        *t1 == *t2 && *l1 == *l2
      }
      _ => false,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnopKind {
  Neg,
  Not,
  Deref,
}

impl From<&TokenKind> for UnopKind {
  fn from(op: &TokenKind) -> Self {
    match op {
      TokenKind::Sub => Self::Neg,
      TokenKind::Bang => Self::Not,
      TokenKind::Mul => Self::Deref,
      _ => unreachable!(),
    }
  }
}

impl UnopKind {
  pub fn as_bool(&self) -> bool {
    match *self {
      Self::Neg | Self::Not => true,
      _ => false,
    }
  }
}

#[inline]
pub fn make_capsule(module: Mod, span: Span) -> Box<Capsule> {
  box Capsule::new(module, span)
}

#[inline]
pub fn make_mod(items: Vec<Box<Item>>, span: Span) -> Mod {
  Mod::new(items, span)
}

#[inline]
pub fn make_item(kind: ItemKind, span: Span) -> Box<Item> {
  box Item::new(kind, span)
}

#[inline]
pub fn make_fun_decl_item(
  ident: Box<Expr>,
  ty: Box<Ty>,
  args: Vec<(Box<Expr>, Box<Ty>)>,
  block: Box<Stmt>,
  span: Span,
) -> Box<Item> {
  box Item::new(
    ItemKind::Fun(box FunDecl {
      ident,
      ty,
      args,
      block,
    }),
    span,
  )
}

#[inline]
pub fn make_stmt(kind: StmtKind, span: Span) -> Box<Stmt> {
  box Stmt::new(kind, span)
}

#[inline]
pub fn make_empty_stmt() -> Box<Stmt> {
  box Stmt::new(StmtKind::Empty, SPAN_ZERO)
}

#[inline]
pub fn make_block_stmt(kind: Vec<Box<Stmt>>, span: Span) -> Box<Stmt> {
  box Stmt::new(StmtKind::Block { kind }, span)
}

#[inline]
pub fn make_ret_stmt(expr: Box<Expr>, span: Span) -> Box<Stmt> {
  box Stmt::new(StmtKind::Ret(expr), span)
}

#[inline]
pub fn make_expr_stmt(expr: Box<Expr>, span: Span) -> Box<Stmt> {
  box Stmt::new(StmtKind::Expr(expr), span)
}

#[inline]
pub fn make_fun_stmt(
  ident: Box<Expr>,
  ty: Box<Ty>,
  args: Vec<(Box<Expr>, Box<Ty>)>,
  block: Box<Stmt>,
  span: Span,
) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Fun(box FunDecl {
      ident,
      ty,
      args,
      block,
    }),
    span,
  )
}

#[inline]
pub fn make_local_stmt(
  ident: Box<Expr>,
  immutable: bool,
  ty: Box<Ty>,
  value: Box<Expr>,
  span: Span,
) -> Box<Stmt> {
  box Stmt::new(
    StmtKind::Local(box Local {
      ident,
      immutable,
      ty,
      value,
    }),
    span,
  )
}

#[inline]
pub fn make_ident_expr(symbol: Symbol, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Ident(symbol), span)
}

#[inline]
pub fn make_array_expr(data: Vec<Box<Expr>>, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Array(data), span)
}

#[inline]
pub fn make_bool_expr(boolean: bool, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Bool(boolean), span)
}

#[inline]
pub fn make_int_expr(number: i32, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Int(number), span)
}

#[inline]
pub fn make_float_expr(number: f32, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Float(number), span)
}

#[inline]
pub fn make_char_expr(ch: char, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Char(ch), span)
}

#[inline]
pub fn make_str_expr(string: String, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Str(string), span)
}

#[inline]
pub fn make_index_expr(
  index: Box<Expr>,
  data: Box<Expr>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::Index { index, data }, span)
}

#[inline]
pub fn make_closure_expr(
  ident: Box<Expr>,
  args: Vec<(Box<Expr>, Box<Ty>)>,
  ty: Box<Ty>,
  block: Box<Stmt>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(
    ExprKind::Closure(box FunDecl {
      ident,
      args,
      ty,
      block,
    }),
    span,
  )
}

#[inline]
pub fn make_call_expr(
  callee: Box<Expr>,
  args: Vec<Box<Expr>>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::Call { callee, args }, span)
}

#[inline]
pub fn make_hash_expr(
  data: Vec<(Box<HashKind>, Box<Expr>)>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::Hash(data), span)
}

#[inline]
pub fn make_hash_data_expr(
  key: Box<Expr>,
  value: Box<Expr>,
) -> (Box<HashKind>, Box<Expr>) {
  (box HashKind::from(key), value)
}

// #[inline]
// pub fn make_if_else_expr(
//   condition: Box<Expr>,
//   consequence: Box<Expr>,
//   alternative: Box<Expr>,
//   span: Span,
// ) -> Box<Expr> {
//   box Expr::new(ExprKind::If {
//     condition, consequence, alternative,
//   }, span)
// }

#[inline]
pub fn make_binop_expr(
  lhs: Box<Expr>,
  op: BinopKind,
  rhs: Box<Expr>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::Binop { lhs, op, rhs }, span)
}

#[inline]
pub fn make_unop_expr(
  operand: UnopKind,
  rhs: Box<Expr>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::Unop { operand, rhs }, span)
}

#[inline]
pub fn make_for_loop_expr(
  iterable: Box<Expr>,
  iterator: Box<Expr>,
  block: Box<Stmt>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(
    ExprKind::For {
      iterable,
      iterator,
      block,
    },
    span,
  )
}

#[inline]
pub fn make_loop_loop_expr(block: Box<Stmt>, span: Span) -> Box<Expr> {
  box Expr::new(ExprKind::Loop { block }, span)
}

#[inline]
pub fn make_while_loop_expr(
  condition: Box<Expr>,
  block: Box<Stmt>,
  span: Span,
) -> Box<Expr> {
  box Expr::new(ExprKind::While { condition, block }, span)
}

#[inline]
pub fn make_ty(kind: TyKind, span: Span) -> Box<Ty> {
  box Ty::new(kind, span)
}
