use std::mem;

use super::ast::{self, StructExpr};

use super::ast::{
  Arg, BinopKind, Block, Expr, ExprKind, Field, FieldExpr, Fun, Program,
  Prototype, Stmt, StmtKind, Struct, Ty, TyKind, UnopKind,
};

use super::interface::Precedence;

use crate::front::tokenizer::token::{Token, TokenKind, TOKEN_EOF};
use crate::front::tokenizer::Tokenizer;
use crate::util::error::{Error, Result};

use crate::util::symbol::SymbolTable;

// parse a source code into an AST
#[inline]
pub fn parse(src: &str) -> Result<Program> {
  let mut symbol_table = SymbolTable::new();
  let mut tokenizer = Tokenizer::new(src, &mut symbol_table);
  let mut parser = Parser::new(&mut tokenizer);
  let ast = parser.parse();

  symbol_table.store();
  ast
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
    match self.tokenizer.next() {
      token => match token.kind() {
        TokenKind::Newline
        | TokenKind::CommentLine
        | TokenKind::CommentLineDoc
        | TokenKind::CommentBlock => {
          self.next();
        }
        _ => {
          self.current = mem::replace(&mut self.first, token);
        }
      },
    }
  }

  #[inline]
  pub fn parse(&mut self) -> Result<Program> {
    let mut stmts = vec![];

    self.next();
    self.next();

    loop {
      match self.current.kind() {
        TokenKind::EOF => break,
        TokenKind::Semicolon => {
          self.next();
          continue;
        }
        _ => match self.parse_stmt() {
          Ok(s) => stmts.push(s),
          Err(e) => self.errors.push(e),
        },
      }

      self.next();
    }

    Ok(ast::mk_program(stmts))
  }

  #[inline]
  fn parse_block(&mut self) -> Result<Box<Block>> {
    let mut stmts = vec![];

    self.next();

    while *self.current.kind() != TokenKind::CloseBrace
      && *self.current.kind() != TokenKind::EOF
    {
      match self.current.kind() {
        TokenKind::Semicolon => {
          self.next();
          continue;
        }
        _ => match self.parse_stmt() {
          Ok(s) => stmts.push(s),
          Err(e) => self.errors.push(e),
        },
      }

      self.next();
    }

    Ok(box ast::mk_block(stmts))
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Stmt> {
    match self.current.kind() {
      TokenKind::Fun => self.parse_fun_stmt(),
      TokenKind::Val | TokenKind::Mut => self.parse_var_stmt(),
      TokenKind::Return => self.parse_return_stmt(),
      TokenKind::Break => self.parse_break_stmt(),
      TokenKind::Continue => self.parse_continue_stmt(),
      TokenKind::Struct => self.parse_struct_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  #[inline]
  fn parse_fun_stmt(&mut self) -> Result<Stmt> {
    let prototype = self.parse_prototype()?;
    let body = self.parse_block()?;

    Ok(ast::mk_stmt(ast::mk_fun(box Fun { prototype, body })))
  }

  #[inline]
  fn parse_prototype(&mut self) -> Result<Prototype> {
    self.next();

    let name = self.parse_ident_expr()?;
    let args = self.parse_args()?;
    let ty = self.parse_ty_expr()?;

    Ok(ast::mk_prototype(name, ty, args))
  }

  #[inline]
  fn parse_args(&mut self) -> Result<Vec<Box<Arg>>> {
    let mut args = vec![];

    self.next();

    if self.first.is(TokenKind::CloseParen) {
      self.next();
      return Ok(args);
    }

    args.push(self.parse_arg()?);

    while self.first.is(TokenKind::Comma) {
      self.next();
      args.push(self.parse_arg()?);
    }

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(args)
  }

  #[inline]
  fn parse_arg(&mut self) -> Result<Box<Arg>> {
    self.next();
    let name = self.parse_ident_expr()?;
    self.expect_first(&TokenKind::Colon)?;
    let ty = self.parse_ty_expr()?;

    Ok(box ast::mk_arg(name, ty))
  }

  #[inline]
  fn parse_var_stmt(&mut self) -> Result<Stmt> {
    let kw = self.current.to_owned();

    self.next();

    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty_expr()?;

    if self.first.is(TokenKind::Assign) {
      self.expect_first(&TokenKind::Assign)?;
      self.next();
    }

    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::Semicolon)?;

    let kind = match kw.kind() {
      TokenKind::Val => ast::mk_val(box ast::mk_local(name, true, ty, value)),
      TokenKind::Mut => ast::mk_mut(box ast::mk_local(name, false, ty, value)),
      _ => unreachable!(),
    };

    Ok(ast::mk_stmt(kind))
  }

  #[inline]
  fn parse_return_stmt(&mut self) -> Result<Stmt> {
    self.next();

    if self.current.is(TokenKind::Semicolon) {
      return Ok(ast::mk_stmt(ast::mk_return(None)));
    }

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    while !self.current.is(TokenKind::Semicolon)
      && !self.current.is(TokenKind::EOF)
    {
      self.next();
    }

    Ok(ast::mk_stmt(ast::mk_return(Some(expr))))
  }

  #[inline]
  fn parse_break_stmt(&mut self) -> Result<Stmt> {
    self.next();

    if self.current.is(TokenKind::Semicolon) {
      return Ok(ast::mk_stmt(ast::mk_break(None)));
    }

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(ast::mk_stmt(ast::mk_break(Some(expr))))
  }

  #[inline]
  fn parse_continue_stmt(&mut self) -> Result<Stmt> {
    self.next();

    if self.current.is(TokenKind::Semicolon) {
      return Ok(ast::mk_stmt(ast::mk_continue(None)));
    }

    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(ast::mk_stmt(ast::mk_continue(Some(expr))))
  }

  #[inline]
  fn parse_struct_stmt(&mut self) -> Result<Stmt> {
    self.next();

    let name = self.parse_ident_expr()?;
    let fields = self.parse_fields()?;

    Ok(ast::mk_stmt(ast::mk_struct_def(box Struct {
      name,
      fields,
    })))
  }

  #[inline]
  fn parse_fields(&mut self) -> Result<Vec<Box<Field>>> {
    let mut fields = vec![];

    self.next();

    if self.first.is(TokenKind::CloseBrace) {
      self.next();
      return Ok(fields);
    }

    fields.push(self.parse_field()?);

    // TODO: should be use TokenKind::CloseBrace
    while self.first.is(TokenKind::Comma) {
      self.next();

      if self.first.is(TokenKind::CloseBrace) {
        break;
      }

      fields.push(self.parse_field()?);
    }

    self.expect_first(&TokenKind::CloseBrace)?;

    Ok(fields)
  }

  #[inline]
  fn parse_field(&mut self) -> Result<Box<Field>> {
    self.next();

    let name = self.parse_ident_expr()?;
    let ty = self.parse_ty_expr()?;

    Ok(box ast::mk_field(name, ty))
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
      TokenKind::AddAssign => self.parse_assign_op_expr(lhs),
      TokenKind::SubAssign => self.parse_assign_op_expr(lhs),
      TokenKind::MulAssign => self.parse_assign_op_expr(lhs),
      TokenKind::DivAssign => self.parse_assign_op_expr(lhs),
      TokenKind::ModAssign => self.parse_assign_op_expr(lhs),
      TokenKind::OpenBracket => self.parse_index_expr(lhs),
      TokenKind::OpenParen => self.parse_call_expr(lhs),
      TokenKind::OpenBrace => self.parse_struct_expr(lhs),
      TokenKind::Dot => self.parse_field_access(lhs),
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
  fn parse_assign_op_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    let op = self.binop();

    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    self.expect_first(&TokenKind::Semicolon)?;

    let kind = match op {
      BinopKind::AddAssignOp => ast::mk_add_assign_op(lhs, rhs),
      BinopKind::SubAssignOp => ast::mk_sub_assign_op(lhs, rhs),
      BinopKind::MulAssignOp => ast::mk_mul_assign_op(lhs, rhs),
      BinopKind::DivAssignOp => ast::mk_div_assign_op(lhs, rhs),
      BinopKind::RemAssignOp => ast::mk_rem_assign_op(lhs, rhs),
      _ => unreachable!(),
    };

    Ok(box ast::mk_expr(kind))
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
  fn parse_struct_expr(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    let fields = self.parse_field_exprs()?;

    Ok(box ast::mk_expr(ast::mk_struct_expr(box StructExpr {
      name: lhs,
      fields,
    })))
  }

  #[inline]
  fn parse_field_exprs(&mut self) -> Result<Vec<Box<FieldExpr>>> {
    let mut fields = vec![];

    self.next();

    if self.first.is(TokenKind::CloseBrace) {
      self.next();
      return Ok(fields);
    }

    fields.push(self.parse_field_expr()?);

    while self.first.is(TokenKind::Comma) {
      self.next();

      if self.first.is(TokenKind::CloseBrace) {
        break;
      }

      self.next();

      fields.push(self.parse_field_expr()?);
    }

    self.expect_first(&TokenKind::CloseBrace)?;

    Ok(fields)
  }

  #[inline]
  fn parse_field_expr(&mut self) -> Result<Box<FieldExpr>> {
    let name = self.parse_ident_expr()?;

    self.expect_first(&TokenKind::Assign)?;
    self.next();

    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    Ok(box FieldExpr { name, value })
  }

  #[inline]
  fn parse_field_access(&mut self, lhs: Box<Expr>) -> Result<Box<Expr>> {
    self.next();

    let rhs = self.parse_expr_by_precedence(&Precedence::Highest)?;

    match lhs.kind() {
      ExprKind::Ident(..) => {
        Ok(box ast::mk_expr(ast::mk_field_access(lhs, rhs)))
      }
      _ => Err(Error::Unexpected("Expected identifier after '.'")),
    }
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
      TokenKind::While => self.parse_while_expr(),
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
      TokenKind::StrBuffer(ref buf) => Ok(box ast::mk_expr(ast::mk_str(*buf))),
      _ => Err(Error::ExpectedExpr(
        "str",
        format!("{:?}", self.current.kind()),
      )),
    }
  }

  #[inline]
  fn parse_ident_expr(&mut self) -> Result<Box<Expr>> {
    match self.current.kind() {
      TokenKind::Identifier(ref name) => {
        Ok(box ast::mk_expr(ast::mk_ident(*name)))
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

  #[inline]
  fn parse_while_expr(&mut self) -> Result<Box<Expr>> {
    self.next();

    let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    let block = self.parse_block()?;

    Ok(box ast::mk_expr(ast::mk_while(condition, block)))
  }

  // TODO: implement a dynamic type system
  #[inline]
  fn parse_ty_expr(&mut self) -> Result<Box<Ty>> {
    if self.first.is(TokenKind::ColonAssign) {
      self.next();
      return self.parse_dynamic_ty();
    }

    if self.current.is(TokenKind::CloseParen)
      && self.first.is(TokenKind::OpenBrace)
    {
      return self.parse_dynamic_ty();
    }

    self.next();

    let kind = match self.current.kind() {
      TokenKind::Colon => {
        self.next();
        self.ty()
      }
      _ => self.ty(),
    };

    Ok(box ast::mk_ty(kind))
  }

  #[inline]
  fn parse_dynamic_ty(&mut self) -> Result<Box<Ty>> {
    self.next();

    Ok(box ast::mk_ty(TyKind::Dynamic))
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

    self.expect_first(kind)?;

    Ok(exprs)
  }

  #[inline]
  fn binop(&mut self) -> BinopKind {
    BinopKind::from(self.current.kind())
  }

  #[inline]
  fn precedence(&mut self) -> Precedence {
    Precedence::from(self.current.kind())
  }

  #[inline]
  fn ty(&mut self) -> TyKind {
    TyKind::from(self.current.kind())
  }

  #[inline]
  fn unop(&mut self) -> UnopKind {
    UnopKind::from(self.current.kind())
  }

  #[inline]
  fn should_precedence_has_priority(&mut self, p: &Precedence) -> bool {
    *p < Precedence::from(self.first.kind())
  }
}
