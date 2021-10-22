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
  Ident(String),
  Bool(bool),
  Int(i64),
  Float(f64),
  Char(char),
  Str(String),
  Array(Vec<Expr>),
  Assign {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Binop {
    lhs: Box<Expr>,
    op: BinopKind,
    rhs: Box<Expr>,
  },
  Index {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Box<Expr>>,
  },
  If {
    condition: Box<Expr>,
    consequence: Box<Block>,
    alternative: Option<Box<Block>>,
  },
  Unop {
    op: UnopKind,
    rhs: Box<Expr>,
  },
  Loop {
    block: Box<Block>,
  },
  While {
    condition: Box<Expr>,
    block: Box<Block>,
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

impl BinopKind {
  pub fn from(kind: &TokenKind) -> Self {
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
  fn from(operand: &TokenKind) -> Self {
    match operand {
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
