use super::ty::Ty;
use crate::front::tokenizer::token::TokenKind;
use crate::util::symbol::Symbol;

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

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
  Ext(Box<Prototype>),
  Fun(Box<Fun>),
  Val(Box<Local>),
  Mut(Box<Local>),
  Return(Option<Box<Expr>>),
  Break(Option<Box<Expr>>),
  Continue(Option<Box<Expr>>),
  Struct(Box<Struct>),
  Expr(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
  pub prototype: Prototype,
  pub body: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Prototype {
  pub name: Box<Expr>,
  pub args: Vec<Box<Arg>>,
  pub ty: Box<Ty>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
  pub name: Box<Expr>,
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
pub struct Struct {
  pub name: Box<Expr>,
  pub fields: Vec<Box<Field>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
  pub name: Box<Expr>,
  pub ty: Box<Ty>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructExpr {
  pub name: Box<Expr>,
  pub fields: Vec<Box<FieldExpr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldExpr {
  pub name: Box<Expr>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
  Bool(bool),
  Int(i64),
  Float(f64),
  Char(char),
  Str(Symbol),
  Ident(Symbol),
  Array(Vec<Box<Expr>>),
  Binop(BinopKind, Box<Expr>, Box<Expr>),
  Unop(UnopKind, Box<Expr>),
  Assign(Box<Expr>, Box<Expr>),
  AssignOp(BinopKind, Box<Expr>, Box<Expr>),
  Index(Box<Expr>, Box<Expr>),
  Closure(Box<Fun>),
  Call(Box<Expr>, Vec<Box<Expr>>),
  If(Box<Expr>, Box<Block>, Option<Box<Block>>),
  Loop(Box<Block>),
  While(Box<Expr>, Box<Block>),
  For(Box<Expr>, Box<Expr>, Box<Block>),
  Range(Box<Expr>, Box<Expr>, Box<Block>),
  StructExpr(Box<StructExpr>),
  FieldAccess(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinopKind {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  And,
  Or,
  Lt,
  Le,
  Gt,
  Ge,
  Eq,
  Ne,
  Shl,
  Shr,
  BitAnd,
  BitOr,
  BitXor,
  AddAssign,
  SubAssign,
  MulAssign,
  DivAssign,
  RemAssign,
  BitXorAssign,
  BitAndAssign,
  BitOrAssign,
}

impl From<&TokenKind> for BinopKind {
  #[inline]
  fn from(kind: &TokenKind) -> Self {
    match kind {
      TokenKind::Add => Self::Add,
      TokenKind::Sub => Self::Sub,
      TokenKind::Mul => Self::Mul,
      TokenKind::Div => Self::Div,
      TokenKind::Mod => Self::Rem,
      TokenKind::Lt => Self::Lt,
      TokenKind::Gt => Self::Gt,
      TokenKind::Le => Self::Le,
      TokenKind::Ge => Self::Ge,
      TokenKind::Eq => Self::Eq,
      TokenKind::Ne => Self::Ne,
      TokenKind::AndAnd => Self::And,
      TokenKind::OrOr => Self::Or,
      TokenKind::Shl => Self::Shl,
      TokenKind::Shr => Self::Shr,
      TokenKind::And => Self::BitAnd,
      TokenKind::Caret => Self::BitXor,
      TokenKind::Or => Self::BitOr,
      TokenKind::AddAssign => Self::AddAssign,
      TokenKind::SubAssign => Self::SubAssign,
      TokenKind::MulAssign => Self::MulAssign,
      TokenKind::DivAssign => Self::DivAssign,
      TokenKind::RemAssign => Self::RemAssign,
      TokenKind::CaretAssign => Self::BitXorAssign,
      TokenKind::BitAndAssign => Self::BitAndAssign,
      TokenKind::BitOrAssign => Self::BitOrAssign,
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
  args: Vec<Box<Arg>>,
) -> Prototype {
  Prototype { name, ty, args }
}

#[inline]
pub const fn mk_arg(name: Box<Expr>, ty: Box<Ty>) -> Arg {
  Arg { name, ty }
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
pub const fn mk_return(expr: Option<Box<Expr>>) -> StmtKind {
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
pub const fn mk_struct_def(struct_def: Box<Struct>) -> StmtKind {
  StmtKind::Struct(struct_def)
}

#[inline]
pub const fn mk_field(name: Box<Expr>, ty: Box<Ty>) -> Field {
  Field { name, ty }
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
  ExprKind::Assign(lhs, rhs)
}

#[inline]
pub const fn mk_assign_op(
  op: BinopKind,
  lhs: Box<Expr>,
  rhs: Box<Expr>,
) -> ExprKind {
  ExprKind::AssignOp(op, lhs, rhs)
}

#[inline]
pub const fn mk_binop(
  op: BinopKind,
  lhs: Box<Expr>,
  rhs: Box<Expr>,
) -> ExprKind {
  ExprKind::Binop(op, lhs, rhs)
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
pub const fn mk_str(value: Symbol) -> ExprKind {
  ExprKind::Str(value)
}

#[inline]
pub const fn mk_ident(name: Symbol) -> ExprKind {
  ExprKind::Ident(name)
}

#[inline]
pub const fn mk_closure(fun: Box<Fun>) -> ExprKind {
  ExprKind::Closure(fun)
}

#[inline]
pub const fn mk_call(callee: Box<Expr>, args: Vec<Box<Expr>>) -> ExprKind {
  ExprKind::Call(callee, args)
}

#[inline]
pub const fn mk_index(lhs: Box<Expr>, rhs: Box<Expr>) -> ExprKind {
  ExprKind::Index(lhs, rhs)
}

#[inline]
pub const fn mk_if(
  condition: Box<Expr>,
  consequence: Box<Block>,
  alternative: Option<Box<Block>>,
) -> ExprKind {
  ExprKind::If(condition, consequence, alternative)
}

#[inline]
pub const fn mk_loop(body: Box<Block>) -> ExprKind {
  ExprKind::Loop(body)
}

#[inline]
pub const fn mk_while(condition: Box<Expr>, body: Box<Block>) -> ExprKind {
  ExprKind::While(condition, body)
}

#[inline]
pub const fn mk_for(
  iterable: Box<Expr>,
  iterator: Box<Expr>,
  body: Box<Block>,
) -> ExprKind {
  ExprKind::For(iterable, iterator, body)
}

#[inline]
pub const fn mk_range(
  start: Box<Expr>,
  end: Box<Expr>,
  body: Box<Block>,
) -> ExprKind {
  ExprKind::Range(start, end, body)
}

#[inline]
pub const fn mk_struct_expr(struct_expr: Box<StructExpr>) -> ExprKind {
  ExprKind::StructExpr(struct_expr)
}

#[inline]
pub const fn mk_field_expr(name: Box<Expr>, value: Box<Expr>) -> FieldExpr {
  FieldExpr { name, value }
}

#[inline]
pub const fn mk_field_access(lhs: Box<Expr>, name: Box<Expr>) -> ExprKind {
  ExprKind::FieldAccess(lhs, name)
}

#[inline]
pub const fn mk_unop(op: UnopKind, rhs: Box<Expr>) -> ExprKind {
  ExprKind::Unop(op, rhs)
}
