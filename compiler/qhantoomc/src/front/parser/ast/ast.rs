use crate::front::tokenizer::token::TokenKind;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
  pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
  pub kind: StmtKind,
}

impl Stmt {
  #[inline]
  pub const fn new(kind: StmtKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub fn kind(&self) -> &StmtKind {
    &self.kind
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
  Ext(Box<Prototype>),
  Fun(Box<Fun>),
  Val(Box<Local>),
  Mut(Box<Local>),
  Return(Box<Expr>),
  Break(Option<Box<Expr>>),
  Continue(Option<Box<Expr>>),
  Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
  pub prototype: Prototype,
  pub body: Box<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prototype {
  pub name: Box<Expr>,
  pub args: Vec<Box<Expr>>,
  pub ty: Box<Ty>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
  pub name: Box<Expr>,
  pub immutable: bool,
  pub ty: Box<Ty>,
  pub value: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
  pub kind: ExprKind,
}

impl Expr {
  #[inline]
  pub const fn new(kind: ExprKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub fn kind(&self) -> &ExprKind {
    &self.kind
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
  Bool(bool),
  Int(i64),
  Float(f64),
  Char(char),
  Str(String),
  Ident(String),
  Array(Vec<Box<Expr>>),
  Binop {
    lhs: Box<Expr>,
    op: BinopKind,
    rhs: Box<Expr>,
  },
  Unop {
    op: UnopKind,
    rhs: Box<Expr>,
  },
  Assign {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Index {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Closure(Box<Fun>),
  Call {
    callee: Box<Expr>,
    args: Vec<Box<Expr>>,
  },
  If {
    condition: Box<Expr>,
    consequence: Box<Block>,
    alternative: Option<Box<Block>>,
  },
  Loop {
    body: Box<Block>,
  },
  While {
    condition: Box<Expr>,
    body: Box<Block>,
  },
  For {
    iterable: Box<Expr>,
    iterator: Box<Expr>,
    body: Box<Block>,
  },
  Range {
    start: Box<Expr>,
    end: Box<Expr>,
    body: Box<Block>,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinopKind {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  And,
  Or,
  Lt,
  Le,
  Gt,
  Ge,
  Eq,
  Ne,
}

impl From<&TokenKind> for BinopKind {
  #[inline]
  fn from(kind: &TokenKind) -> Self {
    match kind {
      TokenKind::Add => Self::Add,
      TokenKind::Sub => Self::Sub,
      TokenKind::Mul => Self::Mul,
      TokenKind::Div => Self::Div,
      TokenKind::Mod => Self::Mod,
      TokenKind::AndAnd => Self::And,
      TokenKind::PipePipe => Self::Or,
      TokenKind::Lt => Self::Lt,
      TokenKind::Gt => Self::Gt,
      TokenKind::Le => Self::Le,
      TokenKind::Ge => Self::Ge,
      TokenKind::Equal => Self::Eq,
      TokenKind::NotAssign => Self::Ne,
      k => unreachable!("{:?} is not a operator", k),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnopKind {
  Neg,
  Not,
}

impl From<&TokenKind> for UnopKind {
  #[inline]
  fn from(kind: &TokenKind) -> Self {
    match kind {
      TokenKind::Sub => Self::Neg,
      TokenKind::Not => Self::Not,
      _ => unreachable!(),
    }
  }
}

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
pub const fn mk_program(stmts: Vec<Stmt>) -> Program {
  Program { stmts }
}

#[inline]
pub const fn mk_block(stmts: Vec<Stmt>) -> Block {
  Block { stmts }
}

#[inline]
pub const fn mk_stmt(kind: StmtKind) -> Stmt {
  Stmt::new(kind)
}

#[inline]
pub const fn mk_fun(fun: Box<Fun>) -> StmtKind {
  StmtKind::Fun(fun)
}

#[inline]
pub const fn mk_prototype(
  name: Box<Expr>,
  ty: Box<Ty>,
  args: Vec<Box<Expr>>,
) -> Prototype {
  Prototype { name, ty, args }
}

#[inline]
pub const fn mk_mut(local: Box<Local>) -> StmtKind {
  StmtKind::Mut(local)
}

#[inline]
pub const fn mk_val(local: Box<Local>) -> StmtKind {
  StmtKind::Val(local)
}

#[inline]
pub const fn mk_local(
  name: Box<Expr>,
  immutable: bool,
  ty: Box<Ty>,
  value: Box<Expr>,
) -> Local {
  Local {
    name,
    immutable,
    ty,
    value,
  }
}

#[inline]
pub const fn mk_return(expr: Box<Expr>) -> StmtKind {
  StmtKind::Return(expr)
}

#[inline]
pub const fn mk_break(expr: Option<Box<Expr>>) -> StmtKind {
  StmtKind::Break(expr)
}

#[inline]
pub const fn mk_continue(expr: Option<Box<Expr>>) -> StmtKind {
  StmtKind::Continue(expr)
}

#[inline]
pub const fn mk_expr(kind: ExprKind) -> Expr {
  Expr::new(kind)
}

#[inline]
pub const fn mk_array(exprs: Vec<Box<Expr>>) -> ExprKind {
  ExprKind::Array(exprs)
}

#[inline]
pub const fn mk_assign(lhs: Box<Expr>, rhs: Box<Expr>) -> ExprKind {
  ExprKind::Assign { lhs, rhs }
}

#[inline]
pub const fn mk_binop(
  op: BinopKind,
  lhs: Box<Expr>,
  rhs: Box<Expr>,
) -> ExprKind {
  ExprKind::Binop { lhs, op, rhs }
}

#[inline]
pub const fn mk_bool(value: bool) -> ExprKind {
  ExprKind::Bool(value)
}

#[inline]
pub const fn mk_int(value: i64) -> ExprKind {
  ExprKind::Int(value)
}

#[inline]
pub const fn mk_float(value: f64) -> ExprKind {
  ExprKind::Float(value)
}

#[inline]
pub const fn mk_char(value: char) -> ExprKind {
  ExprKind::Char(value)
}

#[inline]
pub const fn mk_str(value: String) -> ExprKind {
  ExprKind::Str(value)
}

#[inline]
pub const fn mk_ident(name: String) -> ExprKind {
  ExprKind::Ident(name)
}

#[inline]
pub const fn mk_closure(fun: Box<Fun>) -> ExprKind {
  ExprKind::Closure(fun)
}

#[inline]
pub const fn mk_call(callee: Box<Expr>, args: Vec<Box<Expr>>) -> ExprKind {
  ExprKind::Call { callee, args }
}

#[inline]
pub const fn mk_index(lhs: Box<Expr>, rhs: Box<Expr>) -> ExprKind {
  ExprKind::Index { lhs, rhs }
}

#[inline]
pub const fn mk_if(
  condition: Box<Expr>,
  consequence: Box<Block>,
  alternative: Option<Box<Block>>,
) -> ExprKind {
  ExprKind::If {
    condition,
    consequence,
    alternative,
  }
}

#[inline]
pub const fn mk_loop(body: Box<Block>) -> ExprKind {
  ExprKind::Loop { body }
}

#[inline]
pub const fn mk_while(condition: Box<Expr>, body: Box<Block>) -> ExprKind {
  ExprKind::While { condition, body }
}

#[inline]
pub const fn mk_for(
  iterable: Box<Expr>,
  iterator: Box<Expr>,
  body: Box<Block>,
) -> ExprKind {
  ExprKind::For {
    iterable,
    iterator,
    body,
  }
}

#[inline]
pub const fn mk_range(
  start: Box<Expr>,
  end: Box<Expr>,
  body: Box<Block>,
) -> ExprKind {
  ExprKind::Range { start, end, body }
}

#[inline]
pub const fn mk_unop(op: UnopKind, rhs: Box<Expr>) -> ExprKind {
  ExprKind::Unop { op, rhs }
}

#[inline]
pub const fn mk_ty(kind: TyKind) -> Ty {
  Ty::new(kind)
}
