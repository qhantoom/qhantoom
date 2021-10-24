use std::mem;

use super::ast;

use super::ast::{
  BinopKind, Block, Expr, Program, Prototype, Stmt, StmtKind, Ty, TyKind,
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
      return Ok(self.next());
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
        | TokenKind::Newline
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

    Ok(ast::mk_program(stmts))
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Stmt> {
    match self.current.kind() {
      TokenKind::Fun => self.parse_fun_stmt(),
      TokenKind::Val | TokenKind::Mut => self.parse_var_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  // TODO: this will be change in the future
  #[inline]
  fn parse_fun_stmt(&mut self) -> Result<Stmt> {
    let prototype = self.parse_prototype()?;
    let body = self.parse_block()?;

    Ok(ast::mk_stmt(ast::mk_fun(prototype, body)))
  }

  // TODO: this will be change in the future
  #[inline]
  fn parse_prototype(&mut self) -> Result<Prototype> {
    self.next();

    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty_expr()?;
    let args = self.parse_args()?;

    Ok(ast::mk_prototype(name, ty, args))
  }

  // TODO: this will be change in the future
  #[inline]
  fn parse_args(&mut self) -> Result<Vec<Box<Expr>>> {
    let mut args = vec![];

    if self.first.is(TokenKind::CloseParen) {
      self.next();
      return Ok(args);
    }

    self.next();
    args.push(self.parse_ident_expr()?);

    while self.first.is(TokenKind::Comma) {
      self.next();
      self.next();
      args.push(self.parse_ident_expr()?);
    }

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(args)
  }

  #[inline]
  fn parse_block(&mut self) -> Result<Box<Block>> {
    let mut stmts = vec![];

    self.expect_first(&TokenKind::OpenBrace)?;
    self.next();

    while *self.current.kind() != TokenKind::CloseBrace
      && *self.current.kind() != TokenKind::EOF
    {
      // TODO: this must be change in the future
      if self.current.is(TokenKind::Newline) {
        self.next();
        continue;
      }
    
      stmts.push(self.parse_stmt()?);
      self.next();
    }

    Ok(box ast::mk_block(stmts))
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
      TokenKind::Val => ast::mk_val(name, true, ty, value),
      TokenKind::Mut => ast::mk_mut(name, false, ty, value),
      _ => unreachable!(),
    };

    Ok(ast::mk_stmt(kind))
  }

  #[inline]
  fn parse_expr_stmt(&mut self) -> Result<Stmt> {
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(ast::mk_stmt(StmtKind::Expr(expr)))
  }

  #[inline]
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

    Ok(lhs)
  }

  #[inline]
  fn parse_binop_rhs(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::Assign => self.parse_assign_expr(lhs),
      TokenKind::OpenBracket => self.parse_index_expr(lhs),
      TokenKind::OpenParen => self.parse_call_expr(lhs),
      _ => self.parse_binop_expr(lhs),
    }
  }

  #[inline]
  fn parse_assign_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::Semicolon)?;

    Ok(box ast::mk_expr(ast::mk_assign(lhs, rhs)))
  }

  #[inline]
  fn parse_index_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::CloseBracket)?;

    Ok(box ast::mk_expr(ast::mk_index(lhs, rhs)))
  }

  #[inline]
  fn parse_call_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    let args = self.parse_until(&TokenKind::CloseParen)?;

    Ok(box ast::mk_expr(ast::mk_call(lhs, args)))
  }

  #[inline]
  fn parse_binop_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    let precedence = self.precedence();
    let op = self.binop();

    self.next();

    let rhs = self.parse_expr_by_precedence(&precedence)?;

    Ok(box ast::mk_expr(ast::mk_binop(op, lhs, rhs)))
  }

  #[inline]
  fn parse_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::True | TokenKind::False => self.parse_bool_expr(),
      TokenKind::Int(..) => self.parse_int_expr(),
      TokenKind::Float(..) => self.parse_float_expr(),
      TokenKind::CharAscii(..) => self.parse_char_expr(),
      TokenKind::StrBuffer(..) => self.parse_str_expr(),
      TokenKind::Identifier(..) => self.parse_ident_expr(),
      TokenKind::Sub | TokenKind::Not => self.parse_unop_expr(),
      TokenKind::OpenParen => self.parse_group_expr(),
      TokenKind::OpenBracket => self.parse_array_expr(),
      TokenKind::If => self.parse_if_expr(),
      TokenKind::Loop => self.parse_loop_expr(),
      _ => Err(Error::Custom("expr error")),
    }
  }

  #[inline]
  fn parse_bool_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::True => Ok(box ast::mk_expr(ast::mk_bool(true))),
      TokenKind::False => Ok(box ast::mk_expr(ast::mk_bool(false))),
      _ => Err(Error::ExpectedExpr(
        "bool",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_int_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::Int(ref num) => Ok(box ast::mk_expr(ast::mk_int(*num))),
      _ => Err(Error::ExpectedExpr(
        "int",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_float_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::Float(ref num) => Ok(box ast::mk_expr(ast::mk_float(*num))),
      _ => Err(Error::ExpectedExpr(
        "float",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_char_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::CharAscii(ref ascii) => {
        Ok(box ast::mk_expr(ast::mk_char(*ascii)))
      }
      _ => Err(Error::ExpectedExpr(
        "char",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_str_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::StrBuffer(ref buf) => {
        Ok(box ast::mk_expr(ast::mk_str(buf.into())))
      }
      _ => Err(Error::ExpectedExpr(
        "str",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_ident_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::Identifier(ref ident) => {
        Ok(box ast::mk_expr(ast::mk_ident(ident.into())))
      }
      _ => Err(Error::ExpectedExpr(
        "ident",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_unop_expr(&mut self) -> Result<Box<Expr>> {
    let op = self.unop();

    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Unary)?;

    Ok(box ast::mk_expr(ast::mk_unop(op, rhs)))
  }

  #[inline]
  fn parse_group_expr(&mut self) -> Result<Box<Expr>> {
    self.next();

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(expr)
  }

  #[inline]
  fn parse_array_expr(&mut self) -> Result<Box<Expr>> {
    let exprs = self.parse_until(&TokenKind::CloseBracket)?;

    Ok(box ast::mk_expr(ast::mk_array(exprs)))
  }

  #[inline]
  fn parse_if_expr(&mut self) -> Result<Box<Expr>> {
    self.next();

    let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    let consequence = self.parse_block()?;

    let alternative = if self.first.is(TokenKind::Else) {
      self.next();
      Some(self.parse_block()?)
    } else {
      None
    };

    Ok(box ast::mk_expr(ast::mk_if(
      condition,
      consequence,
      alternative,
    )))
  }

  #[inline]
  fn parse_loop_expr(&mut self) -> Result<Box<Expr>> {
    let block = self.parse_block()?;

    Ok(box ast::mk_expr(ast::mk_loop(block)))
  }

  // TODO: implement a dynamic type system
  #[inline]
  fn parse_ty_expr(&mut self) -> Result<Box<Ty>> {
    self.next();

    if self.current.is(TokenKind::ColonAssign) {
      self.next();
      return Ok(box ast::mk_ty(TyKind::Dynamic));
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

    Ok(box ast::mk_ty(kind))
  }

  #[inline]
  fn parse_until(&mut self, kind: &TokenKind) -> Result<Vec<Box<Expr>>> {
    let mut exprs = vec![];

    if self.first.is(kind.to_owned()) {
      self.next();
      return Ok(exprs);
    }

    self.next();
    exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);

    while self.first.is(TokenKind::Comma) {
      self.next();
      self.next();
      exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);
    }

    self.expect_first(&kind)?;

    Ok(exprs)
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
