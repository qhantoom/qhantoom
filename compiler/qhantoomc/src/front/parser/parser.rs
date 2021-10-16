use std::mem;

use super::ast::{
  BinopKind, Expr, ExprKind, Local, Program, Stmt, StmtKind, Ty, TyKind,
  UnopKind,
};

use super::interface::Precedence;

use crate::front::tokenizer::token::{Token, TokenKind, TOKEN_EOF};
use crate::front::tokenizer::Tokenizer;
use crate::util::error::{Error, Result};

// parse a source code into an AST
#[inline]
pub fn parse(src: &str) -> Result<Program> {
  let mut tokenizer = Tokenizer::new(src);
  let mut parser = Parser::new(&mut tokenizer);

  parser.parse()
}

pub struct Parser<'a> {
  current: Token,
  errors: Vec<Error>,
  first: Token,
  tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
  #[inline]
  pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
    Self {
      current: TOKEN_EOF,
      errors: vec![],
      first: TOKEN_EOF,
      tokenizer,
    }
  }

  #[inline]
  fn expect_first(&mut self, kind: &TokenKind) -> Result<()> {
    if self.first.is(kind.to_owned()) {
      self.next();
      return Ok(());
    }

    Err(Error::Custom(
      "expected first token [..], but the current token is [..]!",
    ))
  }

  #[inline]
  fn next(&mut self) {
    self.current = mem::replace(&mut self.first, self.tokenizer.next());
  }

  #[inline]
  pub fn parse(&mut self) -> Result<Program> {
    let mut stmts = vec![];

    self.next();
    self.next();

    loop {
      match self.current.kind() {
        TokenKind::EOF => break,
        TokenKind::Semicolon
        | TokenKind::CommentLine
        | TokenKind::CommentLineDoc
        | TokenKind::CommentBlock => {
          self.next();
          continue;
        }
        _ => match self.parse_stmt() {
          Ok(stmt) => stmts.push(stmt),
          Err(e) => self.errors.push(e),
        },
      }

      self.next();
    }

    Ok(Program { stmts })
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Stmt> {
    match self.current.kind() {
      TokenKind::Val | TokenKind::Mut => self.parse_var_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  #[inline]
  fn parse_var_stmt(&mut self) -> Result<Stmt> {
    let kw = self.current.to_owned();

    self.next();

    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty_expr()?;
    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::Semicolon)?;

    let kind = match kw.kind() {
      TokenKind::Val => StmtKind::Val(box Local {
        name: box Expr::new(name),
        immutable: true,
        ty,
        value,
      }),
      TokenKind::Mut => StmtKind::Mut(box Local {
        name: box Expr::new(name),
        immutable: false,
        ty,
        value,
      }),
      _ => unreachable!(),
    };

    Ok(Stmt::new(kind))
  }

  #[inline]
  fn parse_expr_stmt(&mut self) -> Result<Stmt> {
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(Stmt::new(StmtKind::Expr(expr)))
  }

  fn parse_expr_by_precedence(
    &mut self,
    precedence: &Precedence,
  ) -> Result<Box<Expr>> {
    let mut lhs = self.parse_expr()?;

    while !self.first.is(TokenKind::Semicolon)
      && self.should_precedence_has_priority(precedence)
    {
      self.next();

      lhs = self.parse_binop_rhs(lhs)?;
    }

    Ok(box Expr::new(lhs))
  }

  #[inline]
  fn parse_binop_rhs(&mut self, lhs: ExprKind) -> Result<ExprKind> {
    match self.current.kind() {
      _ => self.parse_binop_expr(lhs),
    }
  }

  #[inline]
  fn parse_binop_expr(&mut self, lhs: ExprKind) -> Result<ExprKind> {
    let precedence = self.precedence();
    let op = self.binop();

    self.next();

    let rhs = self.parse_expr_by_precedence(&precedence)?;

    Ok(ExprKind::Binop {
      lhs: box Expr::new(lhs),
      op,
      rhs,
    })
  }

  #[inline]
  fn parse_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::True | TokenKind::False => self.parse_bool_expr(),
      TokenKind::Int(..) => self.parse_int_expr(),
      TokenKind::Float(..) => self.parse_float_expr(),
      TokenKind::CharAscii(..) => self.parse_char_expr(),
      TokenKind::StrBuffer(..) => self.parse_str_expr(),
      TokenKind::Identifier(..) => self.parse_ident_expr(),
      TokenKind::Sub | TokenKind::Not => self.parse_unop_expr(),
      _ => Err(Error::Custom("expr error")),
    }
  }

  #[inline]
  fn parse_bool_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::True => Ok(ExprKind::Bool(true)),
      TokenKind::False => Ok(ExprKind::Bool(false)),
      _ => Err(Error::ExpectedExpr(
        "bool",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_int_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::Int(ref num) => Ok(ExprKind::Int(*num)),
      _ => Err(Error::ExpectedExpr(
        "int",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_float_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::Float(ref num) => Ok(ExprKind::Float(*num)),
      _ => Err(Error::ExpectedExpr(
        "float",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_char_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::CharAscii(ref ascii) => Ok(ExprKind::Char(*ascii)),
      _ => Err(Error::ExpectedExpr(
        "char",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_str_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::StrBuffer(ref buf) => Ok(ExprKind::Str(buf.into())),
      _ => Err(Error::ExpectedExpr(
        "str",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_ident_expr(&mut self) -> Result<ExprKind> {
    match self.current.kind() {
      TokenKind::Identifier(ref ident) => Ok(ExprKind::Ident(ident.into())),
      _ => Err(Error::ExpectedExpr(
        "ident",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_unop_expr(&mut self) -> Result<ExprKind> {
    let op = self.unop();

    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Unary)?;

    Ok(ExprKind::Unop { op, rhs })
  }

  #[inline]
  fn parse_ty_expr(&mut self) -> Result<Box<Ty>> {
    self.next();

    if self.current.is(TokenKind::ColonAssign) {
      self.next();
      return Ok(box Ty::new(TyKind::Dynamic));
    }

    self.next();

    let ty = match self.current.kind() {
      _ => self.parse_ty()?,
    };

    self.next();

    Ok(ty)
  }

  #[inline]
  fn parse_ty(&mut self) -> Result<Box<Ty>> {
    let kind = match self.current.kind() {
      TokenKind::U8 => TyKind::U8,
      TokenKind::U16 => TyKind::U16,
      TokenKind::U32 => TyKind::U32,
      TokenKind::U64 => TyKind::U64,
      TokenKind::UInt => TyKind::UInt,
      TokenKind::S8 => TyKind::S8,
      TokenKind::S16 => TyKind::S16,
      TokenKind::S32 => TyKind::S32,
      TokenKind::S64 => TyKind::S64,
      TokenKind::SInt => TyKind::SInt,
      TokenKind::F32 => TyKind::F32,
      TokenKind::F64 => TyKind::F64,
      TokenKind::Bool => TyKind::Bool,
      TokenKind::Char => TyKind::Char,
      TokenKind::Str => TyKind::Str,
      TokenKind::Void => TyKind::Void,
      _ => return Err(Error::Custom("type error")),
    };

    self.next();

    Ok(box Ty::new(kind))
  }

  #[inline]
  fn unop(&mut self) -> UnopKind {
    UnopKind::from(self.current.kind())
  }

  #[inline]
  fn binop(&mut self) -> BinopKind {
    BinopKind::from(&self.current.kind())
  }

  #[inline]
  fn precedence(&mut self) -> Precedence {
    Precedence::from(self.current.kind())
  }

  #[inline]
  fn should_precedence_has_priority(&mut self, kind: &Precedence) -> bool {
    *kind < Precedence::from(self.first.kind())
  }
}
