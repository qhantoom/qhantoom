use std::mem;
use std::path::Path;

use crate::front::tokenizer::{
  Token,
  TokenKind::{self, *},
  Tokenizer, TOKEN_ZERO,
};

use crate::util::reader;
use crate::util::session::session;
use crate::util::span::SPAN_ZERO;

use super::ast;

use super::ast::{
  BinopKind, Capsule, Expr, Item, Mod, Stmt, Ty, TyKind, UnopKind,
};

use super::interface::Precedence;

pub fn parse_capsule_from_file<'a>(
  path: &Path,
) -> Result<Box<Capsule>, String> {
  let file = reader::readfile(path)?;
  let mut tokenizer = Tokenizer::new(&file);
  let mut parser = Parser::new(&mut tokenizer);
  let capsule = parser.parse_capsule_mod()?;

  session().symgen.store();

  Ok(capsule)
}

pub fn parse_capsule_from_source<'a>(
  src: &str,
) -> Result<Box<Capsule>, String> {
  let mut tokenizer = Tokenizer::new(src);
  let mut parser = Parser::new(&mut tokenizer);
  let capsule = parser.parse_capsule_mod()?;

  session().symgen.store();

  Ok(capsule)
}

pub struct Parser<'a> {
  tokenizer: &'a mut Tokenizer<'a>,
  first: Token,
  current: Token,
}

impl<'a> Parser<'a> {
  #[inline]
  pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
    Self {
      tokenizer,
      first: TOKEN_ZERO,
      current: TOKEN_ZERO,
    }
  }

  #[inline]
  fn expect_first(&mut self, kind: &TokenKind) -> Result<(), String> {
    if self.is_first(&kind) {
      return Ok(self.next());
    }

    Err(format!(
      "token {:?} expected, but the current token is {:?}!",
      kind,
      self.first.kind()
    ))
  }

  #[inline]
  fn is_current(&self, kind: &TokenKind) -> bool {
    self.current.kind() == kind
  }

  #[inline]
  fn is_first(&self, kind: &TokenKind) -> bool {
    self.first.kind() == kind
  }

  #[inline]
  fn is_eof(&mut self) -> bool {
    self.current.is_eof()
  }

  #[inline]
  fn next(&mut self) {
    self.current =
      mem::replace(&mut self.first, self.tokenizer.advance_token());
  }

  #[inline]
  fn precedence(&self) -> Precedence {
    Precedence::from(self.current.kind())
  }

  #[inline]
  fn should_precedence_has_priority(&self, kind: &Precedence) -> bool {
    *kind < Precedence::from(self.first.kind())
  }

  #[inline]
  fn parse_capsule_mod(&mut self) -> Result<Box<Capsule>, String> {
    let module = self.parse_mod()?;

    Ok(ast::make_capsule(module, SPAN_ZERO))
  }

  #[inline]
  fn parse_mod(&mut self) -> Result<Mod, String> {
    let mut items = vec![];

    self.next();
    self.next();

    while !self.is_eof() {
      match self.parse_item() {
        Ok(item) => items.push(item),
        Err(e) => {}
      }

      self.next();
    }

    Ok(ast::make_mod(items, SPAN_ZERO))
  }

  #[inline]
  fn parse_item(&mut self) -> Result<Box<Item>, String> {
    print!("\nITEM: {:?}\n\n", self.current);

    match self.current.kind() {
      TokenKind::FunLower => self.parse_fun_decl_item(),
      _ => unreachable!(),
      // _ => self.parse_stmt(),
    }
  }

  // fun main := () {}
  #[inline]
  fn parse_fun_decl_item(&mut self) -> Result<Box<Item>, String> {
    self.next();

    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty()?;

    self.expect_first(&TokenKind::OpenParen)?;

    let args = self.parse_args()?;

    self.expect_first(&TokenKind::OpenBrace)?;

    let block = self.parse_stmt()?;

    Ok(ast::make_fun_decl_item(name, ty, args, block, SPAN_ZERO))
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Box<Stmt>, String> {
    print!("\nSTMT: {:?}\n\n", self.current);

    match self.current.kind() {
      TokenKind::OpenBrace => self.parse_block_stmt(),
      TokenKind::Ret => self.parse_ret_stmt(),
      TokenKind::FunLower => self.parse_fun_decl_stmt(),
      TokenKind::Val | TokenKind::Mut => self.parse_local_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  #[inline]
  fn parse_block_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let mut stmts = vec![];

    self.next();

    while !self.is_current(&TokenKind::CloseBrace) {
      match self.parse_stmt() {
        Err(_) => break,
        Ok(stmt) => stmts.push(stmt),
      };

      self.next();
    }

    Ok(ast::make_block_stmt(stmts, SPAN_ZERO))
  }

  #[inline]
  fn parse_ret_stmt(&mut self) -> Result<Box<Stmt>, String> {
    self.next();

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    while *self.current.kind() != TokenKind::Semicolon
      && *self.current.kind() != TokenKind::NewLine
    {
      self.next();
    }

    Ok(ast::make_ret_stmt(expr, SPAN_ZERO))
  }

  fn parse_fun_decl_stmt(&mut self) -> Result<Box<Stmt>, String> {
    self.next();

    let name = self.parse_ident_expr()?;

    self.expect_first(&TokenKind::OpenParen)?;

    let args = self.parse_args()?;

    let ty;

    if self.is_first(&TokenKind::Arrow) {
      self.next();

      ty = self.parse_ty()?
    } else {
      ty = ast::make_ty(TyKind::Void, SPAN_ZERO);
    }

    self.expect_first(&TokenKind::OpenBrace)?;

    let block = self.parse_block_stmt()?;

    Ok(ast::make_fun_stmt(name, ty, args, block, SPAN_ZERO))
  }

  #[inline]
  fn parse_arg(&mut self) -> Result<(Box<Expr>, Box<Ty>), String> {
    self.next();

    let name = self.parse_ident_expr()?;

    self.expect_first(&TokenKind::Colon)?;

    let ty = self.parse_ty()?;

    Ok((name, ty))
  }

  #[inline]
  fn parse_args(&mut self) -> Result<Vec<(Box<Expr>, Box<Ty>)>, String> {
    let mut args = vec![];

    if self.is_first(&TokenKind::CloseParen) {
      self.next();
      return Ok(args);
    }

    args.push(self.parse_arg()?);

    while self.is_first(&TokenKind::Comma) {
      self.next();
      args.push(self.parse_arg()?);
    }

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(args)
  }

  #[inline]
  fn parse_local_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let kw = self.current.to_owned();

    self.next();
    
    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty()?;

    self.next();

    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    print!("\nLOCAL: {:?}\n", self.current);

    self.expect_first(&TokenKind::Semicolon)?;

    match kw.kind() {
      TokenKind::Mut => {
        Ok(ast::make_local_stmt(name, false, ty, value, SPAN_ZERO))
      }
      TokenKind::Val => {
        Ok(ast::make_local_stmt(name, true, ty, value, SPAN_ZERO))
      }
      _ => Err(format!("parser:parse_local_stmt:error")),
    }
  }

  #[inline]
  fn parse_expr_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.is_first(&TokenKind::Semicolon) {
      self.next();
    }

    Ok(ast::make_expr_stmt(expr, SPAN_ZERO))
  }

  fn parse_expr_by_precedence(
    &mut self,
    precedence: &Precedence,
  ) -> Result<Box<Expr>, String> {
    let mut expr = self.parse_expr()?;

    while !self.is_first(&TokenKind::Semicolon)
      && self.should_precedence_has_priority(precedence)
    {
      self.next();

      expr = self.parse_binop_expr_by_lhs(expr)?;
    }

    Ok(expr)
  }

  fn parse_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      // TokenKind::Comment(..) => self.parse_comment_expr(),
      TokenKind::OpenBrace => self.parse_hash_expr(),
      TokenKind::OpenBracket => self.parse_array_expr(),
      TokenKind::OpenParen => self.parse_group_expr(),
      TokenKind::Ident(..) => self.parse_ident_expr(),
      TokenKind::False | True => self.parse_bool_expr(),
      // TokenKind::For => self.parse_for_loop_expr(),
      // TokenKind::If => self.parse_if_else_expr(),
      TokenKind::Loop => self.parse_infinite_loop_expr(),
      TokenKind::While => self.parse_while_loop_expr(),
      TokenKind::FloatNumber(..) => self.parse_float_expr(),
      TokenKind::IntNumber(..) => self.parse_int_expr(),
      TokenKind::StrBuffer(..) => self.parse_str_expr(),
      TokenKind::CharAscii(..) => self.parse_char_expr(),
      // TokenKind::OpenParen => self.parse_closure_expr(),
      TokenKind::Sub | TokenKind::Bang => self.parse_unop_expr(),
      _ => Err(format!("to unary error: {:?}", &self.current.kind())),
    }
  }

  #[inline]
  fn parse_binop_expr_by_lhs(
    &mut self,
    lhs: Box<Expr>,
  ) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::OpenBracket => self.parse_index_expr(lhs),
      TokenKind::OpenParen => self.parse_call_expr(lhs),
      _ => self.parse_binop_expr(lhs),
    }
  }

  #[inline]
  fn parse_index_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>, String> {
    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::CloseBracket)?;

    Ok(ast::make_index_expr(lhs, rhs, SPAN_ZERO))
  }

  #[inline]
  fn parse_call_expr(
    &mut self,
    callee: Box<Expr>,
  ) -> Result<Box<Expr>, String> {
    let args = self.parse_until(&TokenKind::CloseParen)?;

    Ok(ast::make_call_expr(callee, args, SPAN_ZERO))
  }

  fn parse_binop_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>, String> {
    let precedence = self.precedence();
    let op = BinopKind::from(&self.current);

    self.next();

    let rhs = self.parse_expr_by_precedence(&precedence)?;

    Ok(ast::make_binop_expr(lhs, op, rhs, SPAN_ZERO))
  }

  // fn parse_if_else_expr(&mut self) -> Result<Box<Expr>, String> {
  //   self.next();

  //   let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;

  //   self.expect_first(&TokenKind::OpenBrace)?;

  //   let consequence = self.parse_block_stmt()?;

  //   let alternative = if self.is_first(&TokenKind::Else) {
  //     self.next();
  //     self.expect_first(&TokenKind::OpenBrace)?;

  //     Some(self.parse_block_stmt()?)
  //   } else {
  //     None
  //   };

  //   Ok(ast::make_if_else_expr(
  //     condition,
  //     consequence,
  //     alternative,
  //     SPAN_ZERO,
  //   ))
  // }

  #[inline]
  fn parse_array_expr(&mut self) -> Result<Box<Expr>, String> {
    let expr = self.parse_until(&TokenKind::CloseBracket)?;

    Ok(ast::make_array_expr(expr, SPAN_ZERO))
  }

  // fn parse_closure_expr(&mut self) -> Result<Box<Expr>, String> {
  //   let args = self.parse_args()?;

  //   self.expect_first(&TokenKind::Arrow)?;

  //   let ty = self.parse_ty()?;

  //   self.expect_first(&TokenKind::OpenBrace)?;

  //   let block = self.parse_block_stmt()?;

  //   Ok(ast::make_closure_expr(, args, ty, block, SPAN_ZERO))
  // }

  // fn parse_comment_expr(&mut self) -> Result<Box<Expr>, String> {
  //   match self.current.kind() {
  //     TokenKind::Comment(Line) => {
  //       let value = self.current.text();

  //       Ok(InsKind::Comment(Line, value))
  //     }
  //     _ => Err(format!("comment error: {}", self.current.literal)),
  //   }
  // }

  // pub fn parse_for_loop_expr(&mut self) -> Result<Box<Expr>, String> {
  //   self.is_first(&TokenKind::OpenBracket);

  //   let iterable = self.parse_array_expr()?;

  //   self.next();
  //   self.expect_first(&Operator(OperatorKind::Or))?;
  //   self.expect_first(&Identifier)?;

  //   let variable = self.parse_ident_expr()?;

  //   self.expect_first(&Operator(OperatorKind::Or))?;
  //   self.next();
  //   self.expect_first(&TokenKind::OpenBrace)?;

  //   let block = self.parse_block_stmt()?;

  //   Ok(InsKind::Loop(LoopKind::For(
  //     Box::new(Instruction::new(iterable)),
  //     Box::new(Instruction::new(variable)),
  //     block,
  //   )))
  // }

  fn parse_group_expr(&mut self) -> Result<Box<Expr>, String> {
    self.next();

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(expr)
  }

  fn parse_hash_expr(&mut self) -> Result<Box<Expr>, String> {
    let mut data = vec![];

    while !self.is_first(&TokenKind::CloseBrace) {
      self.next();

      let key = self.parse_expr_by_precedence(&Precedence::Lowest)?;

      self.expect_first(&TokenKind::Colon)?;
      self.next();

      let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;

      data.push(ast::make_hash_data_expr(key, value));

      if !self.is_first(&TokenKind::CloseBrace) {
        self.expect_first(&TokenKind::Comma)?;
      }
    }

    self.expect_first(&TokenKind::CloseBrace)?;

    Ok(ast::make_hash_expr(data, SPAN_ZERO))
  }

  #[inline]
  fn parse_ident_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::Ident(ref ident) => {
        Ok(ast::make_ident_expr(*ident, SPAN_ZERO))
      }
      _ => Err(format!(
        "unexpected error on ident parse with {:?}",
        &self.current
      )),
    }
  }

  #[inline]
  fn parse_int_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::IntNumber(ref int) => Ok(ast::make_int_expr(*int, SPAN_ZERO)),
      _ => Err(format!("parse int literal error.")),
    }
  }

  #[inline]
  fn parse_float_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::FloatNumber(ref float) => {
        Ok(ast::make_float_expr(*float, SPAN_ZERO))
      }
      _ => Err(format!("parse float literal error.")),
    }
  }

  #[inline]
  fn parse_bool_expr(&mut self) -> Result<Box<Expr>, String> {
    let is_true = self.current.is(TokenKind::True);

    Ok(ast::make_bool_expr(is_true, SPAN_ZERO))
  }

  fn parse_char_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::CharAscii(ref ch) => Ok(ast::make_char_expr(*ch, SPAN_ZERO)),
      _ => Err(format!("parse char literal error.")),
    }
  }

  #[inline]
  fn parse_str_expr(&mut self) -> Result<Box<Expr>, String> {
    match self.current.kind() {
      TokenKind::StrBuffer(ref buf) => {
        Ok(ast::make_str_expr(buf.to_string(), SPAN_ZERO))
      }
      _ => Err(format!("parse str error expr.")),
    }
  }

  #[inline]
  fn parse_infinite_loop_expr(&mut self) -> Result<Box<Expr>, String> {
    self.expect_first(&TokenKind::OpenBrace)?;

    let block = self.parse_block_stmt()?;

    Ok(ast::make_loop_loop_expr(block, SPAN_ZERO))
  }

  fn parse_ty(&mut self) -> Result<Box<Ty>, String> {
    self.next();

    // print!("\nTY: {:?}\n", self.current);

    if self.current.is(TokenKind::ColonEq) {
      let ty = ast::make_ty(TyKind::Void, SPAN_ZERO);
      return Ok(ast::make_ty(TyKind::Array(ty), SPAN_ZERO));
    }

    let kind = match self.current.kind() {
      TokenKind::Uint => Ok(TyKind::Uint),
      TokenKind::Bool => Ok(TyKind::Bool),
      TokenKind::Char => Ok(TyKind::Char),
      TokenKind::Uint => Ok(TyKind::Uint),
      // TokenKind::Int => Ok(TyKind::Int),
      TokenKind::Str => Ok(TyKind::Str),
      _ => Err(format!("parse ty error expr.")),
    }?;

    self.next();

    Ok(ast::make_ty(kind, SPAN_ZERO))
  }

  fn parse_unop_expr(&mut self) -> Result<Box<Expr>, String> {
    let op = UnopKind::from(self.current.kind());

    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Unary)?;

    Ok(ast::make_unop_expr(op, rhs, SPAN_ZERO))
  }

  fn parse_while_loop_expr(&mut self) -> Result<Box<Expr>, String> {
    self.next();

    let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::OpenBrace)?;

    let block = self.parse_block_stmt()?;

    Ok(ast::make_while_loop_expr(condition, block, SPAN_ZERO))
  }

  fn parse_until(
    &mut self,
    kind: &TokenKind,
  ) -> Result<Vec<Box<Expr>>, String> {
    let mut exprs = vec![];

    if self.is_first(kind) {
      self.next();
      return Ok(exprs);
    }

    self.next();
    exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);

    while self.is_first(&TokenKind::Comma) {
      self.next();
      self.next();
      exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);
    }

    self.expect_first(&kind)?;

    Ok(exprs)
  }
}
