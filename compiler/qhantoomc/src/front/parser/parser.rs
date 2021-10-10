use std::mem;

use super::ast::{
  BinopKind, /* Block, */ Expr, ExprKind, /* Function, */ /* Local, */ Program,
  /* Prototype, */ Stmt, StmtKind, /* Ty, */ /* TyKind, */ UnopKind,
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
      tokenizer: tokenizer,
    }
  }

  #[inline]
  fn _expect_first(&mut self, kind: &TokenKind) -> Result<()> {
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

      self.tokenizer.next();
    }

    Ok(Program { stmts })
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Box<Stmt>> {
    match self.current.kind() {
      _ => self.parse_expr_stmt(),
    }
  }

  #[inline]
  fn parse_expr_stmt(&mut self) -> Result<Box<Stmt>> {
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(box Stmt::new(StmtKind::Expr(expr)))
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
    // move to the next token
    self.next();
    // parse the right hand side of the expression
    let rhs = self.parse_expr_by_precedence(&precedence)?;
    // return the binary expression
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
    // SYNTAX: - <expr> | ! <expr>
    // store the operand
    let op = UnopKind::from(self.current.kind());
    // move to the next token
    self.next();
    // parse right hand side expression by precedence
    let rhs = self.parse_expr_by_precedence(&Precedence::Unary)?;
    // return the unary expression
    Ok(ExprKind::Unop { op: op, rhs: rhs })
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
